# PROVEN-DISPATCH.md: stage 14 (refinement of the HTTPS architecture, port 8091)

Branch `proven-dispatch`, built on `spec-lemmas` at `5b880fb` (the code published as
gumshoe2020/scratch-server `70e09b0`). Approved by Andrew 2026-10-30 ("Yes please", for step 1 of
Dudley's coverage plan). Authoritative for this stage.

## Why
formal-coverage v1.5 (aarch64-linux-musl, compiled-files) puts first-party security code at
**644 / 1,348 SLoC proven (47.77%)**. The biggest unproven block that is *pure logic* rather than
I/O is the request-handling code in `src/lib.rs`: what happens after the connection machine
returns an `Action`. That code decides the response status, walks header names for diagnostics,
sanitizes names before they reach the log, and rate-limits unknown-header log lines. All of it
is reachable from untrusted bytes, and all of it is currently unverified or only tested:

| Function (src/lib.rs) | SLoC | Today |
|---|---:|---|
| `rejection_line` | 31 | tested |
| `dispatch` | 29 | unverified |
| `header_names` | 20 | unverified |
| `should_log` | 15 | tested |
| `log_unknown` | 14 | unverified |
| `reject` | 13 | tested |
| `sanitized` | 8 | tested |
| `with_policy`, `reply`, `serve_connection`, `default`, `action`, `whitespace` | ~30 | mixed |
| `serve_with_started` (read loop) | 57 | unverified |

Stage 8's approach worked: a **pure core** in Verus, a **thin shell** that only moves bytes. Do
the same here.

## Approach
Add a module `crates/transport-core/src/app.rs`, declared `pub mod app;` in
`crates/transport-core/src/lib.rs`, inside `verus!`. This keeps it under the existing
transport-core Verus root, so `./verify.sh` and formal-coverage pick it up with no tool or
config change. If you find a real reason it can't live there, log it in NOTES.md, choose the
nearest equivalent, and state exactly what verify.sh and the formal-coverage config would need.
**Do not edit the formal-coverage repository.**

`src/lib.rs` keeps only the shell: socket reads, `Instant::now()` (converted to a `u64`
millisecond count for the core), the `Mutex`, stdout writes, the filesystem check that builds
the `/` page body, and `write_all`/`flush`. Every decision moves into `app.rs`.

## Properties to prove (all inputs, no bounded checking)

1. **Routing.** `route(path: &[u8]) -> Route` with `Route { Health, Root, NotFound }`:
   `Health` iff `path == b"/health"`, `Root` iff `path == b"/"`, otherwise `NotFound`.
   `route_status(r)` is `Ok` for `Health`/`Root` and `NotFound` otherwise. The status that
   `serve_with_started` logs and the status that `dispatch` sends **must both come from this one
   function**. Today they come from two separate `match`es. Prove that the logged status code equals the
   status line actually sent.
2. **Post-machine plan.** `plan(action) -> Plan`, a pure mapping from the connection machine's
   `Action` to what the shell must do. Prove:
   - a 200 or 404 response happens **only** for `Action::Dispatch`, and the handler counter
     increments only on that path, exactly once;
   - every `Reject(_)` yields status 400 with the fixed body `b"bad request\n"` and `head ==
     false`, whatever the diagnostics produce;
   - `CloseSilently` and `Done` yield **zero** response bytes;
   - `ReadMore` never produces a response.
3. **Header-name walk.** Replace the `header_names` iterator with an index-based
   `next_header_name(bytes, pos) -> Option<(start, end, next)>`. Prove: `start <= end <= bytes.len()`;
   every span lies after the request line and before the first empty line; a yielded name contains
   no `:` and no `\r\n` pair; `next > pos` (termination); the walk never reads past `bytes.len()`. The
   names yielded, in order, must be the same as the current iterator's (see the equivalence tests below).
   **Amended 2026-10-30 (Andrew: "A is good"):** a name may contain a bare `\r` or bare `\n`, because
   the current walk splits only on the `\r\n` pair and behaviour must not change. Log safety for those
   bytes comes from property 4 (the sanitizer), not from this walk.
4. **Sanitizer (log injection).** `sanitize(name) -> Vec<u8>`: every output byte is ASCII
   alphanumeric or `-`; `len <= 64`; output equals the spec "filter, then take 64". Hence no CR,
   LF, space, `=` or control byte from a client can reach a log line.
5. **Rejection diagnostic.** `diagnose(bytes, policy) -> Diagnostic` with variants
   `TooManyHeaders`, `HardReject`, `Duplicate`, `Guarded`, `Unknown` (each carrying the name span) and
   `Grammar`. The spec must mirror the current rules exactly: more than 24 names gives
   `TooManyHeaders`; `classify` in `19..28` gives `HardReject`; a second Host (class 0) or
   Connection (class 2) gives `Duplicate`; `!drop_header(policy, class)` gives `Guarded` for
   class < 44 and `Unknown` otherwise; the first match wins; no match gives `Grammar`. Then
   `diagnostic_line(d, bytes) -> Vec<u8>` produces the exact bytes printed today (without the
   trailing newline). Prove that the line equals a fixed prefix from a closed set followed by
   `sanitize(name)` where there is a name, that it contains no byte outside printable ASCII, and that
   its length is at most 128.
6. **Unknown-header log limiter.** A core `UnknownLog` with `entries: Vec<(Vec<u8>, u64)>` (lowercase
   name, ms timestamp) and `should_log(&mut self, name, limit, now_ms) -> bool` that keeps today's
   semantics (60,000 ms window, cap `min(limit, 64)`, case-insensitive dedup, lowercase storage).
   Prove: `entries.len() <= min(limit, 64)` is invariant; a `true` result appends exactly one entry;
   a name already present case-insensitively within the window returns `false`. Therefore, for
   non-decreasing `now_ms`, at most `min(limit, 64)` lines are logged in any 60-second window.
   If `now_ms` goes backwards, the core must stay safe: no panic and no underflow. Document the choice.

## Behaviour must not change
Responses, log lines and their order must be **byte-identical** to `5b880fb` for every input.
Before you delete the old `rejection_line`, `header_names`, `sanitized` and `UnknownLog`
implementations, move them into a test-only module as oracles, then add equivalence tests:
- 100,000 generated header blocks (mutated from the browser fixtures and the malformed corpus,
  including bare CR, bare LF, colons in odd places, empty names, more than 24 headers, non-ASCII
  bytes and 4,097-byte inputs): old and new `rejection_line` give identical output, and old and new
  `header_names` give identical name sequences;
- every byte value through old and new `sanitize`, plus random names up to 300 bytes;
- the existing `unknown_log_is_bounded_in_every_sliding_minute` test, run against both limiters
  with identical results.
If an equivalence test exposes a difference you believe is a real bug in the old code, **stop,
commit the evidence, and explain it in NOTES.md.** Don't fix it; a fix needs Andrew's approval.

## Acceptance
- `./verify.sh` passes with 0 errors. Report the verified count per root. There must be no `assume`,
  `admit`, `external_body`, `external` or `unsafe` anywhere first-party (scan and show it).
  PROOFS.md lists each new property with file:line and an updated "still unverified" shell list.
- `cargo test --workspace` passes: all 31 existing tests unchanged in meaning (Safari/Firefox/Brave
  fixtures over HTTP and HTTPS; the malformed corpus giving 400 with 0 handler calls; idle, slow and
  silent-close tests; 100,000 differential cases; policy tests), plus the new equivalence tests.
- The stage 13 spec-mutant run still catches 15 of 15.
- `./nix-build.sh check` and `./nix-build.sh image-static` pass; the image is static, shell-free
  and key-free.
- Live on a test port: container `proven-dispatch` on **8096**, hardened as before (read-only,
  no capabilities, non-root), with the TLS cert mounted read-only. The stage 5–8 openssl/curl
  checks pass, including the 3-second delayed request returning 200 and the 17-second idle
  connection closing with 0 bytes. Safari, Firefox and Brave fixture replays return 200, and the
  stdout log lines for a scripted set of rejected requests are byte-identical to the same requests
  against `header-policy` on 8095.
- Do not touch the containers `scratch-server` (stopped), `nix-scratch` (8089), `parser-http` (8090),
  `parser-https` (8091) or `header-policy` (8095). Dudley redeploys onto 8091 after his independent
  acceptance.

## Measurement
Dudley measures with formal-coverage v1.5 at acceptance. Codex does **not** run formal-coverage.
In NOTES.md, list every function moved or added, with its old and new location, and say whether
each new one is proven or left in the shell, so the measurement can be checked against your list.
Expected result: about +150 proven SLoC, roughly 47.8% to 59%. Report what actually happened.

## Out of scope
`src/tls.rs` (stage 15 candidate), `src/main.rs`, `src/transport.rs`, the parser, the policy, the
grammar, the connection machine's existing proofs (extend them only if needed, never weaken
them), EVOLUTION.md and ATTACK-SURFACE.md (Dudley updates these after acceptance), and pushing.
