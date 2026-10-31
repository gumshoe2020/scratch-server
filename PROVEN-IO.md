# PROVEN-IO.md: stage 16 (startup and I/O; port 8098)

Branch `proven-io`, built on `proven-record` after `6860b91`. Approved by Andrew 2026-10-30
("10-seconds seems correct. Yes, let's approve this spec and build it."), including the 10 s request
deadline. Authoritative for this stage.

## Why
After stages 14 and 15, most unproven first-party code is startup and I/O: `src/main.rs` (`port`,
`run`, `healthcheck`), `src/transport.rs` (`DeadlineSocket`, `serve_tcp`) and the read loop in
`src/lib.rs` (`serve_with_started`, `log_unknown`, `reply`). Verus can't reason about syscalls,
threads, atomics or the clock, so the aim is **not** to prove this code wholesale. It is to:
1. pull out the few *decisions* this code makes about attacker-controlled timing and connection
   counts, and prove them; and
2. name everything left over as the trusted base, function by function, with the reason it can't be
   proven and the test that covers it.

The small rustls accessor methods in `src/tls.rs` (`output_len`, `key_len`, `scheme` and similar) are
**out of scope**. Proving them would need `external_trait_specification`, which stays banned.

## Finding that motivates property 1 (behaviour change, approved by Andrew 2026-10-30)
The idle deadline (15 s before the first request byte) is enforced. After the first byte, the only
limit is 2 s *between* reads. There is no limit on the whole request. Measured live on 8097
(2026-10-30): a 35-byte request sent one byte every 1.5 s, one TLS record per byte, held the
connection for 52.7 s and then got `200`. With the 4,096-byte cap, one connection can be held for
about 2 h 16 min, and 64 such connections fill the concurrency cap. Not tested live: filling all 64
slots. IDLE-CONNECTIONS.md says "a slow client still can't hold a worker by trickling bytes"; that is
not what the code does.

Fix: **a request deadline of 10 s, measured from the first decrypted request byte.** When it expires,
the existing slow path runs (`ReadTimeout` gives `400` and logs `reason=slow`). Nothing else about
the response or the log changes.

## Approach
Same pattern as stages 8, 14 and 15. Add `crates/transport-core/src/io.rs`, declared `pub mod io;`
in `crates/transport-core/src/lib.rs` inside its own `verus!` block, so `./verify.sh` and
formal-coverage pick it up with no tool or config change. **Do not edit the formal-coverage
repository.** The shell converts `Instant`, `io::ErrorKind` and environment values to plain integers,
enums and byte slices before calling the core, and converts the results back.

## Properties to prove (all inputs, no bounded checking)

1. **Read deadlines.** Constants `IDLE_NS = 15_000_000_000`, `READ_NS = 2_000_000_000`,
   `REQUEST_NS = 10_000_000_000` (u64 nanoseconds). Add
   `read_timeout(elapsed: u64, first_byte: Option<u64>) -> Option<u64>`, where `elapsed` is the time
   since accept and `first_byte` is the time since accept at which the first decrypted request byte
   was read. `None` means time is up; the shell returns `io::ErrorKind::TimedOut` without calling
   the socket.
   - Not started (`first_byte == None`): `Some(IDLE_NS - elapsed)` iff `elapsed < IDLE_NS`, else
     `None`. This is exactly today's `checked_duration_since(...).filter(!is_zero)`.
   - Started (`Some(t)`): let `deadline = t.saturating_add(REQUEST_NS)`. Return `None` iff
     `elapsed >= deadline`, else `Some(min(READ_NS, deadline - elapsed))`.
   - Prove: `Some(d)` implies `d > 0`. Not started implies `elapsed + d <= IDLE_NS`. Started implies
     `d <= READ_NS` and `elapsed + d <= deadline`. Prove a lemma: under these rules, a connection
     whose first byte arrives at `t` never gets a read timeout past `t + REQUEST_NS`, and one that
     never sends a byte never gets one past `IDLE_NS`.
   - Shell: `DeadlineSocket` records the first-byte time once (the `started` callback sets it only
     if unset) and gets `elapsed` from `saturating_duration_since(accepted)`, saturating to
     `u64::MAX`.
2. **Read classification.** Add `ReadResult { Data(usize), Interrupted, Timeout, Other }` (the shell
   maps `Ok(n)`, `Interrupted`, `TimedOut | WouldBlock` and everything else) and
   `classify(r) -> Step { Retry, Eof, Bytes(n), ReadTimeout, Fail }`. Prove: `Data(0)` gives `Eof`,
   `Data(n > 0)` gives `Bytes(n)`, `Interrupted` gives `Retry`, `Timeout` gives `ReadTimeout`, `Other`
   gives `Fail`. Prove that **only `Bytes` sets the first-byte time** (return a `starts: bool`), so TLS
   handshake reads and timeouts can never start or extend the request clock. `serve_with_started`
   uses this instead of its inline `match`.
3. **Connection admission.** `MAX_CONNECTIONS = 64`; `admit(active: usize) -> Option<usize>` returns
   `Some(active + 1)` iff `active < 64`. Prove the result never exceeds 64. `run` passes it to the
   existing `fetch_update`. `release(active: usize) -> usize` returns `active - 1`, saturating at 0.
   Prove it never underflows and that `admit` followed by `release` returns the start value.
4. **Port.** `parse_port(s: Option<&[u8]>) -> Option<u16>`. `None` (unset or non-UTF-8, exactly as
   `env::var` + `unwrap_or_else` behaves today) gives `Some(8080)`. Otherwise it mirrors
   `str::parse::<u16>` exactly: one optional leading `+`, then one or more ASCII digits, value
   `<= 65_535`, and nothing else (no whitespace, no `-`). Prove that `Some(p)` implies the input is
   that form with value `p`, and that every input of that form with value `<= 65_535` is accepted.
5. **One source for sizes.** Add `HASH_LEN = 32` to the core. Prove `aead::KEY_LEN == HASH_LEN` and that
   the X25519 and P-256 lengths used by `record.rs` match the constants they use. `output_len`,
   `hash_len` and `key_len` in `tls.rs` return the core constants. Those three methods stay shell.

## Behaviour must not change (except property 1's request deadline and the admission fix)
Responses, log lines, exit codes and error kinds must be identical to `6860b91` for every input,
except that a request whose first byte arrived more than 10 s ago now times out as `slow`, and
except the admission fix below.

**Admission fix (approved by Andrew 2026-10-30, after the `af18014` blocker).** The old closure
`(n < 64).then_some(n + 1)` evaluates `n + 1` eagerly, so at `usize::MAX` it panics under overflow
checks (debug builds) instead of returning `None`. Release builds already return `None`. The
proven `admit` must return `None` for every `active >= 64`, including `usize::MAX`, in every build
profile, and must never evaluate `active + 1` unless `active < 64`. This is the one permitted
admission difference: the old closure panicking in an overflow-checking build where `admit` returns
`None`. Update `tests/stage16_admission.rs` so the `usize::MAX` case asserts the new `admit` returns
`None` in both debug and release; keep the old-closure panic evidence as a documented historical
case, not a failure. Every other admission input must still agree exactly with the old closure.
- **Read deadlines:** an oracle test runs the old `DeadlineSocket` timeout computation and the new
  one over 100,000 (elapsed, started) pairs, including 0, `IDLE_NS - 1`, `IDLE_NS`, `IDLE_NS + 1`,
  and `u64::MAX`. With the request deadline disabled they must agree exactly. With it enabled, the
  only allowed difference is a shorter timeout or `None` past `t + REQUEST_NS`.
- **Classification:** every `io::ErrorKind` variant, plus `Ok(0)`, `Ok(1)` and `Ok(1024)`, against
  the old match.
- **Admission:** every `active` in `0..=200` against the old closure (exact agreement), plus
  `usize::MAX` (new `admit` returns `None`; see the admission fix above).
- **Port:** every byte string up to length 6 over the alphabet `0-9 + - space a \0` plus 10,000
  random strings and the boundary values `0`, `65535`, `65536`, `+0`, `+65535`, `00080` and the
  empty string, against `str::parse::<u16>`; also an unset variable and a non-UTF-8 value.
- All existing tests (43) unchanged in meaning and passing.
If an equivalence test shows a difference you believe is a real bug in the old code, **stop, commit
the evidence and explain it in NOTES.md.** Don't fix it; a fix needs Andrew's approval.

## Trusted base (new file `TRUSTED-BASE.md`)
List every first-party function that is still unproven after this stage, `src/tls.rs` included. For
each, give its file:line, SLoC, what it does, **why it can't be proven** (syscall, thread, atomic,
clock, rustls trait wiring, libcrux call, startup-only stdout) and **which test or live check covers
it**. Group the entries and give subtotals. This file is the stage's main deliverable, alongside the
proofs.

## Acceptance
- `./verify.sh` passes with 0 errors. Report verified counts per root. There must be no `assume`,
  `admit`, `external_body`, `external` or `unsafe` anywhere first-party (scan and show it).
  PROOFS.md lists each new property with file:line.
- `cargo test --workspace` passes: the existing 43 tests plus the new oracle and deadline tests.
- New deadline tests over plain TCP and TLS:
  - trickle one byte every 1.5 s, which must give `400` with `reason=slow` between 10 and 12 s
    after the first byte;
  - a complete request sent over 8 s, which must give `200`;
  - idle 3 s then send the request, which must give `200`;
  - idle 17 s, which must close with 0 bytes.
- The stage 13 spec-mutant run still catches 15 of 15.
- `./nix-build.sh check` and `./nix-build.sh image-static` pass; the image is static, shell-free
  and key-free. Tag the image `proven-io:static` (don't reuse `scratch-server:static`).
- Live on container `proven-io` on **8098**, hardened as before (read-only, no capabilities,
  non-root, no-new-privileges), TLS directory and header policy mounted read-only. The stage 15 live
  suite passes against 8098. Its responses and log lines for the scripted request set are
  byte-identical to `proven-record` on 8097, and the live trickle test gives `400 reason=slow` on 8098
  but `200` on 8097.
- Do not touch the containers `scratch-server`, `nix-scratch` (8089), `parser-http` (8090),
  `parser-https` (8091), `header-policy` (8095), `proven-dispatch` (8096) or `proven-record` (8097).

## Measurement
Dudley measures with formal-coverage v1.5 at acceptance. Codex does **not** run formal-coverage.
In NOTES.md, list every function moved or added with old and new location and proven-or-shell
status, and the SLoC of `main.rs`, `transport.rs` and `lib.rs` before and after.

## Out of scope
The `tls.rs` accessors and all other rustls trait impls; `healthcheck` (a local TLS client, which
stays shell and is listed in TRUSTED-BASE.md); `root_body` and its stale "FROM-scratch" text; the
startup policy-summary line; `load_config`, `server_config` and `load_private_key`; the parser,
policy, grammar and existing proofs (extend them only if needed, never weaken them); rustls itself;
EVOLUTION.md, ATTACK-SURFACE.md and IDLE-CONNECTIONS.md (Dudley corrects these after acceptance);
pushing.
