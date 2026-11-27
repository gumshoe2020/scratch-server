# BERTIE-BOUNDARY.md: stage 19 (Bertie replaces rustls behind the proven core; port 8099)

> **Dependency (updated 2026-11-21):** Bertie is now maintained by us, based on
> cryspen/bertie `ef65f76`, with encrypted close_notify `fbbcc13`, verified DER
> codec `b10a3f0` and integration `8ff624a`, dependency fixes `fd49638` and `03374d4`, Kyber removal
> and patched KEM `d34ad5e`, and restricted server/client-feature separation
> `a20fa69`, wire rejection tests `2f2c2c2`, and RNG implementation-boundary
> cleanup `e1af495`, and RNG instance-boundary repair `e6ea574`. Local source remains `file://~/src/bertie-deps`.
> Andrew chose the fixed-ClientHello readiness probe on 2026-11-21; client remains
> disabled in production. The E results below supersede the D blockers.


Branch `bertie-boundary`, built on `spec-layout` (`0b7dec4`). Requested by Andrew 2026-11-01 ("I would
like to keep making progress on our scratch server"; he chose Bertie over rustls on 2026-10-31).
Authoritative for this stage.

## Why
Stage 17 measured rustls at up to 5,095 in-binary lines, none of them proven. The Bertie spike
(`projects/bertie-spike`, report in its REPORT.md) served TLS 1.3 from Cryspen's Bertie with about
900 in-binary Bertie lines, and Safari, curl, OpenSSL and a default rustls client all completed
handshakes. Bertie's handshake and key schedule carry published proofs (CCS 2025; see the spike
REPORT for exactly what is and is not established at the pinned commit). But the spike's own glue was
unproven and ad hoc: it inspected the ClientHello and parsed HTTP itself.

This stage puts Bertie in front of the **existing** proven pipeline: the Verus HTTP parser,
transport state machine, dispatch, response formatting, deadlines and connection admission stay
exactly as they are. Only the TLS layer changes. The one new piece of attacker-facing glue, the
ClientHello inspection Bertie needs before `Server::accept`, is written in Verus.

## Dependency
- Candidate `bertie` git pin: `file://~/src/bertie-deps`, rev
  `e6ea574f698aadf8c56174b878f945c251a7fccb`, on `deps/libcrux-advisories`,
  maintained by us. `version = "=0.1.0-pre.2"`; Cargo.lock records the exact rev.
  Defaults remain `api,std,defensive`. Client API/stream/handshake entry points
  require the non-default `client` feature; production does not enable it.
  The production healthcheck sends a constant ClientHello and accepts only a bounded
  first ServerHello record within two seconds. It is unauthenticated readiness,
  not certificate/HTTP verification. No TLS client feature is enabled.
- Remove `rustls`, `rustls-webpki`, `rustls-pki-types`, our custom libcrux `CryptoProvider` and
  anything only they need from the **production** dependency tree. `rcgen` or a rustls client may stay
  as dev-dependencies for tests. `cargo tree -e normal --target aarch64-unknown-linux-musl` must
  show no rustls crate.
- The Nix flake must build the static image with the git dependency (cargoLock `outputHashes` or the
  equivalent), still gated on the proof and cargo-deny checks. Update `deny.toml` for the git source
  only as narrowly as needed (allow that one git repo), and log the change.

## M1: proven ClientHello inspection (`transport-core/src/hello.rs`)
Bertie's `Server::accept` needs an `Algorithms` value chosen before it parses the hello. The spike's
`inspect()` (bertie-spike `server/src/main.rs`) does this in unproven Rust. Move this decision into a
new Verus module `transport-core/src/hello.rs`:
1. Input: the bytes of the first TLS record, as read by the shell with the existing size bound.
   Output: `Option<Choice>` where `Choice` names one cipher suite, one group and one signature
   scheme, plus the offered-group list for logging (bounded length).
2. Supported set (same as the spike): suite `TLS_CHACHA20_POLY1305_SHA256`; groups
   `X25519MLKEM768`, `X25519`, `secp256r1`, preferring them in that order among groups for which the
   client sent a key share; signature `ecdsa_secp256r1_sha256` (our certificate type).
3. Proofs: no out-of-bounds access and no arithmetic overflow for any input; total (always
   terminates, never panics); the result is `Some` only if the chosen suite, group and signature
   scheme were each offered by the client, the chosen group has a key share in the hello, and each
   is in the supported set; the group is the most preferred one satisfying that. Spec in
   `hello.rs`'s spec functions, following the stage 18 per-crate `spec.rs` layout.
4. Intent lemmas (in the stage 13 style): non-vacuity (a concrete Safari-, curl- and
   OpenSSL-shaped hello each yield the expected choice); a hello offering only AES suites, only
   unsupported groups, or a supported group without a key share yields `None`.
5. Add at least 5 deliberate spec mutants for `hello.rs` to the existing mutant harness (e.g. drop
   the key-share condition, flip the preference order, accept an unoffered suite) and show each is
   caught. The existing 15 must still be caught.
6. Return `None` (and the shell closes silently) for anything else, including a ClientHello split
   across records and HelloRetryRequest cases. Do not implement HRR or reassembly; they are out of
   scope and are known upstream gaps.

## M2: Bertie TLS shell
Replace the rustls path in `src/tls.rs` and `src/transport.rs` with a Bertie adapter:
1. A `BertieStream` (or similar) that implements `Read + Write` over Bertie's record API, so the
   existing `serve_with_started` and everything beneath it run **unchanged**. Handshake: read the
   first record through the existing deadline socket, call `hello::choose`, then
   `Server::accept`, write ServerHello, the compatibility ChangeCipherSpec and the server flight,
   read client Finished. Application data: decrypt records into a bounded buffer for `read`;
   encrypt `write` output as application data records.
2. Keep every existing deadline and limit: the idle allowance from accept covers the handshake;
   only decrypted HTTP bytes start the request deadline; the 10 s request deadline; the 64
   connection cap. All timing decisions go through the existing proven `io.rs` functions.
3. On success, after the response, send the encrypted close_notify (Bertie's `close_notify()` from
   the pinned rev). On any Bertie error, close silently without dispatching; never log record
   contents, keys or secrets.
4. Certificate and key loading: reuse the existing TLS directory format (cert.pem, key.pem, ECDSA
   P-256). Load once at startup.
5. Plain HTTP mode (no TLS directory) must still work unchanged.
6. List every new unproven production function in `verification/TRUSTED-BASE.md` with its reason
   and test, as stage 16 did. Remove entries for deleted rustls glue.
7. `transport-core/src/record.rs` and `aead.rs` stay in the crate and stay verified, but are no
   longer called by production code (Bertie does its own record layer). Say so in TRUSTED-BASE.md
   and in the stage notes; do not delete them in this stage.

## M3: tests and live checks
1. `cargo test --workspace` passes. Port the tests in `src/tls.rs` that exercised the rustls path
   to the Bertie path (use a rustls or OpenSSL client from dev-dependencies or the host as the peer):
   routes, Safari-shaped headers, malformed requests never dispatch, case and count boundaries,
   idle/partial deadlines over TCP and TLS, absolute handshake budget.
2. Build the image (`./nix-build.sh` image-static) and run it as a new container named
   `bertie-boundary` on host port **8099**, hardened exactly like `proven-io` (read-only root,
   non-root uid, all capabilities dropped, no-new-privileges, TLS directory mounted read-only,
   restart unless-stopped). This is the only long-lived container you may create or start. For comparison only, one
   temporary `proven-io-ref` may use the existing `proven-io:static` image on
   `127.0.0.1:8098`, with the same hardening and existing read-only TLS mount. Remove that
   reference after comparison; never rebuild/retag its image or modify other containers.
3. Live: the 18 stage-16 live requests over TLS on 8099 give byte-identical HTTP responses to
   `proven-io` on 8098. Rerun the spike's interop matrix (`bertie-spike/scripts/interop.py`,
   pointed at 8099) with certificate verification on; record results in the stage doc. Expected:
   same rows as the spike; any difference is explained.
4. Measure, with the stage 17 method and scripts: in-binary lines for Bertie and for any remaining
   TLS crates, and the formal-coverage proven/unproven numbers for running server code, compared
   with stage 16.

## Rerun amendments (2026-11-21)

Keep the canonical-signature expected-success assertion unchanged and also complete at least
10,000 native handshakes with a strict DER-validating client. Rerun every acceptance gate,
including all twenty mutants and locked no-fail-fast workspace tests. Pure Nix may fetch the
local exact-revision source through builtins.fetchGit; no vendoring, source copy or weakened
gate is allowed. If pure fetching fails, stop that line and mark image-dependent checks
"pending push" until Andrew publishes the same rev to his fork. Run formal-coverage only if
its executable is available; do not install it. Commit R1, R2, R3 and final as configured Dudley,
without pushing. The full rerun instructions are in dev/CODEX_PROMPT-stage19b.md.

## Acceptance
- `./verification/verify.sh`: zero errors. Parser and transport counts may only go up; report the
  new counts. 15 + the new hello mutants all caught.
- `cargo test --workspace` passes; `cargo tree` shows no rustls in the production tree;
  `nix flake check` and the image-static build pass.
- `bertie-boundary` is running on 8099, healthy over HTTPS with the existing certificate, and the
  byte-identical comparison and interop matrix are recorded.
- A "Results" section at the end of this file: proof counts, mutants, tests, interop table,
  in-binary line counts (Bertie vs stage 17 rustls), coverage numbers, and the list of new trusted
  functions.
- Browser checks (Safari, Firefox, Brave) are Andrew's; leave a checklist for them in the stage doc.

## Out of scope

The 2026-11-21 "our Bertie" instructions supersede the historical prohibitions
on modifying Bertie and replaying its hax/F* proofs, within D1/D1b/D2 only.
All other scratch-server fences remain in force.

HelloRetryRequest, ClientHello reassembly, AES-GCM, changes to Bertie itself (report upstream
issues in NOTES.md instead), replaying Bertie's F*/SSProve/ProVerif proofs, touching other
containers, formal-coverage, EVOLUTION.md and ATTACK-SURFACE.md (Dudley updates them after
acceptance).


## Results — 2026-11-01 (blocked; not accepted) — superseded by 2026-11-21

M1's new module and five new mutants passed locally; M2 is implemented but fails TLS
acceptance at the required Bertie pin. M3 release/live acceptance is blocked. M1 commit
`92cae38`; M2 commit `15536b5`; M3 commit `5000cff`. These are Codex's local checks, not independent acceptance.

| Check | Observed result |
|---|---|
| Native `verification/verify.sh` | PASS: parser 211, transport 347, AEAD 9; hello standalone 30; zero errors |
| Hello obligations | 22 executable/spec termination obligations and 8 intent witnesses; included in transport's 347 |
| Five new hello mutants at M1 | 5/5 killed after successful refinement; all five fail actual tests, four also fail intent |
| Existing fifteen / final twenty-mutant run | Retained unchanged; final harness stops on failing TLS baseline; no final kills counted |
| Final workspace tests (`--locked --workspace --no-fail-fast`) | 57 executed: 55 passed, 2 failed (canonical signatures and original TLS case/count suite) |
| Production `cargo tree -e normal --target aarch64-unknown-linux-musl` | PASS: no rustls, rustls-webpki or rustls-pki-types |
| Bertie features | default = api + std + defensive; exact rev fbbcc131de30d7e8f1599f66a83f16e694fe1997 |
| Catalog / protected scope | 9 entries, 95 spec functions, zero catalog errors; original HTTP/transport/dispatch/deadline/admission/record/AEAD sources unchanged |
| Nix flake check | FAIL: Linux TLS tests reject Bertie signatures (final glibc library target 27 passed / 1 failed; noncanonical DER regression) |
| Static image | FAIL: musl TLS library tests 25 passed / 3 failed; no archive loaded and no image created |
| Container / port 8099 | Not deployed; existing proven-io and bertie-spike untouched |
| 18 byte-identical HTTP comparisons | NOT RUN: no accepted image/container on 8099 |
| In-binary audit | Final-source musl audit: 1,184 Bertie lines (671 outermost-source functions); stage 17 rustls 4,152 + webpki 693 + pki-types 250 = 5,095 |
| Formal coverage | NOT MEASURED: no running stage 19 image and formal-coverage executable unavailable on PATH |

The failure is Bertie's `tls13formats.rs::ecdsa_signature`: it retains fixed-width r/s
leading zeros instead of removing redundant DER INTEGER padding. Strict peer DER decoding
rejects some CertificateVerify signatures. Repeated TLS tests reproduce this on macOS and
in the Nix Linux build. The added regression calls the production adapter and retains an
expected-success assertion; it does not disable validation. It is randomized, not a fixed
seed transcript. See dev/NOTES.md for evidence and the other upstream gaps. Bertie was not
patched. No signing retries, weakened tests or skipped release gates were introduced.

The historical README formal-coverage v1.4 baseline is 644/1,363 proven first-party security
SLoC (47.25%), 300 tested and 419 unverified, explicitly from stage 13. It is not a new
stage 16 or 19 measurement; no tool change or invented before/after coverage is claimed.
The hand-counted production shell is 28 functions / 487 SLoC versus stage 16's 63 / 618;
this is an inventory, not formal-coverage. All eleven new trusted production functions are
in src/tls.rs: load_config, load_certificate, invalid, algorithms, read_record,
BertieStream::accept, close_notify, Read::read, Write::write, flush, healthcheck.
The changed main healthcheck and transport serve_tcp remain trusted too. Reasons and test
limits are in verification/TRUSTED-BASE.md. record.rs and aead.rs remain verified but unused
by production; Bertie's own record layer does not inherit their local proofs.

### Interop matrix

No stage 19 live result is claimed. The following rows preserve the spike matrix's scope.
`scripts/stage19-interop.py` runs the spike source read-only with port/container/evidence
paths changed, and the real server's root/health bodies substituted for spike-only body
assertions. Certificate/hostname verification and all protocol/deadline probes stay enabled.
`scripts/stage19-compare.py` uses the exact eighteen requests extracted from stage16-live.py
and verifies TLS certificates on both 8098 and 8099. Scripts were syntax-checked, not live-run.

| Probe | Stage 19 observed |
|---|---|
| /usr/bin/curl default | NOT RUN — image acceptance blocked |
| /usr/bin/curl explicit TLS1.3 | NOT RUN — image acceptance blocked |
| /opt/homebrew/opt/curl/bin/curl default | NOT RUN — image acceptance blocked |
| /opt/homebrew/opt/curl/bin/curl explicit TLS1.3 | NOT RUN — image acceptance blocked |
| group X25519 | NOT RUN — image acceptance blocked |
| group P-256 | NOT RUN — image acceptance blocked |
| group X25519MLKEM768 | NOT RUN — image acceptance blocked |
| suite TLS_CHACHA20_POLY1305_SHA256 | NOT RUN — image acceptance blocked |
| suite TLS_AES_128_GCM_SHA256 | NOT RUN — image acceptance blocked |
| suite TLS_AES_256_GCM_SHA384 | NOT RUN — image acceptance blocked |
| suite TLS_AES_128_CCM_SHA256 | NOT RUN — image acceptance blocked |
| suite TLS_AES_128_CCM_8_SHA256 | NOT RUN — image acceptance blocked |
| TLS1.2 only | NOT RUN — image acceptance blocked |
| no common group | NOT RUN — image acceptance blocked |
| no common suite | NOT RUN — image acceptance blocked |
| HRR supported X25519 but only P384 key share | NOT RUN — image acceptance blocked |
| garbage bytes | NOT RUN — image acceptance blocked |
| truncated ClientHello | NOT RUN — image acceptance blocked |
| ClientHello split across many records | NOT RUN — image acceptance blocked |
| slow trickle handshake | NOT RUN — image acceptance blocked |
| HTTP root and encrypted close_notify | NOT RUN — image acceptance blocked |
| HTTP 404 and encrypted close_notify | NOT RUN — image acceptance blocked |
| HTTP malformed and encrypted close_notify | NOT RUN — image acceptance blocked |
| 12s idle then request (15s idle allowance) | NOT RUN — image acceptance blocked |
| idle over 15s closes | NOT RUN — image acceptance blocked |
| 10s HTTP deadline against trickle | NOT RUN — image acceptance blocked |
| 64 connection cap | NOT RUN — image acceptance blocked |

The default rustls-client row from the spike's separate client program is also NOT RUN.
Received close_notify is unsupported by Bertie; malformed/empty HTTP EOF fixtures therefore
use TCP half-close at a record boundary while preserving every HTTP result/dispatch assertion.
The server still sends encrypted close_notify. ALPN selection is absent and the client checks
HTTP/1.1 fallback. These are protocol differences from the old rustls shell, not HTTP changes.

### Historical browser checklist (superseded; no browser acceptance claimed)

- [ ] Safari: certificate-verified root and /health on 8099, no trust-warning bypass.
- [ ] Firefox: certificate-verified root and /health on 8099.
- [ ] Brave: certificate-verified root and /health on 8099, including cache-disabled request.

No physical browser, x86_64, end-to-end TLS proof, Bertie proof replay, independent acceptance,
new fuzz coverage or formal-coverage percentage is claimed. Restoring acceptance requires
an upstream DER fix and a reviewed spec/pin update; the required pin was not changed here.

Final-source audit reproduction: `verification/scripts/bertie-surface-build.sh`, then
`python3 verification/scripts/bertie-surface.py` (redirect each to logs/stage19-*).
This is the stage-17 musl release profile with debug information retained and stripping off;
its measurement-only test override does not build an image or bypass release gates. The
1,184 lines include Bertie's local healthcheck client role and are an upper bound on
reachable code. Remaining rustls/webpki/pki-types have zero production dep-info and zero
in-binary lines. The result is not byte identity, dynamic line coverage or proof coverage.
No Docker image was produced, loaded or left for cleanup; all build containers used --rm.


## Results — 2026-11-21 — superseded by libcrux update below

**Not accepted: the DER fix passes, but the unchanged cargo-deny advisory gate blocks release.**
R1 `601228a` passes locally; R2 `774a59a` records pure local-source support and the advisory
blocker; R3 `d699d0e` records the fresh source audit and blocked live scope. These are Codex's
checks, not independent acceptance. All production Rust, tests, proofs and core configuration
are unchanged from dac4d41; the strict DER regression retains its expected-success assertion.

| Check | Observed result |
|---|---|
| Native `verification/verify.sh` | PASS: parser 211, transport 347, AEAD 9; hello standalone 30; zero errors |
| Hello obligations | Unchanged 22 executable/spec termination obligations and 8 intent witnesses; included in transport's 347 |
| Five new hello mutants | PASS: 5/5 caught after successful refinement; all five actual test failures, 4 intent kills |
| Existing fifteen / final twenty-mutant run | PASS: 15 + 5 = 20/20 caught; zero survivors/invalid; all actual test failures, 17 intent kills |
| Final workspace tests (`--locked --workspace --no-fail-fast`) | PASS: 57 passed, zero failed, one ignored detector (executed separately below) |
| 10,000 strict-DER native handshakes | PASS: 10,000 attempted, zero failures; first_failed_case=None, reasons={}; 550.93 seconds; M4_ASSERT_ZERO=1 |
| Production `cargo tree -e normal --target aarch64-unknown-linux-musl` | PASS: no rustls, rustls-webpki or rustls-pki-types |
| Bertie features | Unchanged default = api + std + defensive; exact local git rev 8ff624adda8b74c8c0edbec29b7c92492164233f |
| Catalog / protected scope | PASS: 9 entries, 95 spec functions, zero errors; src/crates/tests/config unchanged from dac4d41 |
| Nix flake check | FAIL: advisory gate; pure local fetch, proofs and glibc tests pass (55 root-package tests, zero failed, one ignored) |
| Static image | FAIL: advisory gate; separate musl server-static derivation passes all 55 root-package tests; no image/archive produced or loaded |
| Container / port 8099 | Not deployed; no bertie-boundary or proven-io-ref created; other containers untouched |
| 18 byte-identical HTTP comparisons | NOT RUN: no accepted image; blocked by dependency advisory gate |
| In-binary audit | Fresh source-audit musl binary: 1,199 Bertie lines / 671 functions; stage 17 TLS total 5,095; old TLS crates zero; no image measurement |
| Formal coverage | NOT MEASURED: executable unavailable on PATH/in this checkout; nothing installed; no new percentage claimed |

### Release blocker and reproducibility

Pure local-source fetching succeeds with cargoLock.allowBuiltinFetchGit and the exact revision
in Cargo.lock. The ephemeral Linux runner mounts the linked worktree and its resolved common
Git metadata read-only. NAR hash: `sha256-6aU+7cDOYOVPZmfEjusOh8KE7k/wfn2Yu+ZY1IevQVY=`.
Bertie source was fetched only through Cargo/Nix caches, with no vendoring or source copy
added to the tracked checkout; no upstream changes or pushes occurred.
No impure expression was added to flake.nix. Sources, licenses and bans pass after adding an
explicit `=0.1.0-pre.2` version constraint beside the exact git revision.

The already pinned advisory snapshot rejects eleven vulnerability findings:
RUSTSEC-2026-0387, -0124, -0023, -0026, -0075, -0330, -0331, -0073, -0074, -0207, -0208;
and two unmaintained dependencies: RUSTSEC-2024-0436 (paste), RUSTSEC-2024-0370
(proc-macro-error). These are dependency-level cargo-deny findings, not an assertion of
reachability of every issue. No ignore, feature reduction, upstream patch or gate bypass
was introduced. **Nothing is pending push on source/purity grounds: publishing this same
revision alone cannot clear the advisory gate.** The image, container, comparison, interop
and audit of a shipped image require dependency remediation and passing release gates.

Native commands use `CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup"`.
Evidence is under `logs/stage19b-*`; full-mutant receipts use `logs/stage19b-mutants/`.
The 10,000-handshake command is `python3 scripts/stage19b-handshakes.py --tls-dir <existing-TLS-directory>`;
it starts the production native binary, invokes the unchanged dac4d41 rustls detector with
certificate/hostname and strict signature validation, asserts zero failures and terminates
its own server. Nix uses `NIX_LOG_PREFIX=stage19b ./nix-build.sh check` and `image-static`.
Native workspace tests include two transport-core hello tests beyond Nix's 55 root-package
tests; no test was removed. Optional cargo fmt --check reports pre-existing formatting
in unchanged Rust files; it is not claimed passing. No git hook was bypassed.

The source audit is reproduced with `NIX_LOG_PREFIX=stage19b verification/scripts/bertie-surface-build.sh`,
then `NIX_LOG_PREFIX=stage19b python3 verification/scripts/bertie-surface.py`. The helper now
imports the tracked Git worktree to avoid traversing temporary mutant copies under ignored
target/. Its historical measurement-only debug/strip/test override is separate from image
acceptance. Binary SHA-256: `3738a720ed4f1bd08bb3e810ccc217650722899aa392a5a6d742874a2dc438d8`.
The count includes the local Bertie healthcheck client and is an upper bound on reachability,
not dynamic line coverage or proof coverage. It is not an audit of a running/shipped image.

No formal-coverage measurement was possible. The historical stage-13 v1.4 baseline remains
644/1,363 first-party security SLoC proven (47.25%), 300 tested and 419 unverified; no stage-16
to stage-19 percentage comparison is inferred. The unchanged hand inventory is 28 trusted
shell functions / 487 SLoC versus stage 16's 63 / 618; this is not formal coverage.
No new production function was added by this rerun. Stage 19's eleven new trusted TLS
functions remain load_config, load_certificate, invalid, algorithms, read_record,
BertieStream::accept, close_notify, Read::read, Write::write, flush and healthcheck; main's
healthcheck and transport's serve_tcp remain trusted too. Reasons/tests/limits are in
verification/TRUSTED-BASE.md. record.rs and aead.rs remain verified and unused by production.
The DER fix's upstream proof artifacts were not replayed here.

### Interop matrix — rerun

All verification remains enabled in the scripts. No probe is claimed executed against a
stage-19 image, because the dependency gate rejected image creation. SCRATCH_TLS_DIR now
selects the existing TLS directory explicitly; STAGE19_LOG_PREFIX=stage19b selects evidence
paths. The exact eighteen historical comparison requests and protocol/deadline probes remain.

| Probe | Stage 19 observed |
|---|---|
| /usr/bin/curl default | NOT RUN — dependency advisory gate |
| /usr/bin/curl explicit TLS1.3 | NOT RUN — dependency advisory gate |
| /opt/homebrew/opt/curl/bin/curl default | NOT RUN — dependency advisory gate |
| /opt/homebrew/opt/curl/bin/curl explicit TLS1.3 | NOT RUN — dependency advisory gate |
| group X25519 | NOT RUN — dependency advisory gate |
| group P-256 | NOT RUN — dependency advisory gate |
| group X25519MLKEM768 | NOT RUN — dependency advisory gate |
| suite TLS_CHACHA20_POLY1305_SHA256 | NOT RUN — dependency advisory gate |
| suite TLS_AES_128_GCM_SHA256 | NOT RUN — dependency advisory gate |
| suite TLS_AES_256_GCM_SHA384 | NOT RUN — dependency advisory gate |
| suite TLS_AES_128_CCM_SHA256 | NOT RUN — dependency advisory gate |
| suite TLS_AES_128_CCM_8_SHA256 | NOT RUN — dependency advisory gate |
| TLS1.2 only | NOT RUN — dependency advisory gate |
| no common group | NOT RUN — dependency advisory gate |
| no common suite | NOT RUN — dependency advisory gate |
| HRR supported X25519 but only P384 key share | NOT RUN — dependency advisory gate |
| garbage bytes | NOT RUN — dependency advisory gate |
| truncated ClientHello | NOT RUN — dependency advisory gate |
| ClientHello split across many records | NOT RUN — dependency advisory gate |
| slow trickle handshake | NOT RUN — dependency advisory gate |
| HTTP root and encrypted close_notify | NOT RUN — dependency advisory gate |
| HTTP 404 and encrypted close_notify | NOT RUN — dependency advisory gate |
| HTTP malformed and encrypted close_notify | NOT RUN — dependency advisory gate |
| 12s idle then request (15s idle allowance) | NOT RUN — dependency advisory gate |
| idle over 15s closes | NOT RUN — dependency advisory gate |
| 10s HTTP deadline against trickle | NOT RUN — dependency advisory gate |
| 64 connection cap | NOT RUN — dependency advisory gate |
| Default rustls client (spike's separate client scope) | NOT RUN — dependency advisory gate |

### Historical browser checklist (19b; superseded)

Blocked until the dependency gate, image and automated live checks pass. No browser check
has been performed in this rerun; do not bypass certificate warnings.

- [ ] Safari: certificate-verified root and /health on 8099.
- [ ] Firefox: certificate-verified root and /health on 8099.
- [ ] Brave: certificate-verified root and /health on 8099, including cache-disabled request.

No physical-browser, second-device, x86_64, end-to-end TLS proof, proof replay, independent
acceptance or new fuzz/formal-coverage result is claimed. No container was created for a
failed release; the existing proven-io image was neither rebuilt nor retagged. No push.


## Results — 2026-11-21 (libcrux update) — superseded by our Bertie below

**Not accepted. B1 is partial; the Nix dependency advisory gate still blocks B3.**
The independent AEAD/Ed25519 fixes are committed in the new Bertie worktree on
`deps/libcrux-advisories` (`fd49638`, followed by `03374d4`, the final pin).
No Bertie source, DER proof or default-feature change. The minimum fixed KEM
release removed the legacy Kyber-draft API; changing that behavior conflicts with
the supplied fence and awaits Andrew's scope decision. Fresh Bertie hax/F* checks
are also blocked by the missing historical toolchain cache. These are Codex-run
local checks, not independent acceptance. See [Bertie dependency report](../../dev/BERTIE-DEPS-REPORT.md).

| Check | Observed result |
|---|---|
| Native `verification/verify.sh` | PASS: parser 211, transport 347, AEAD 9; hello standalone 30; zero errors |
| Hello obligations | Unchanged 22 executable/spec termination obligations and 8 intent witnesses; included in transport's 347 |
| Five new hello mutants | PASS: 5/5 caught; all actual test failures, 4 intent kills |
| Existing fifteen / final twenty-mutant run | PASS: 20/20 caught; zero survivors/invalid; all refinement gates pass and all twenty fail actual tests; 17 intent kills |
| Final workspace tests (`--locked --workspace --no-fail-fast`) | PASS: 57 passed, zero failed; one ignored detector executed separately below |
| 10,000 strict-DER native handshakes | PASS: 10,000 attempted, zero failures; first_failed_case=None, reasons={}; 424.43 seconds; M4_ASSERT_ZERO=1 |
| Production `cargo tree -e normal --target aarch64-unknown-linux-musl` | PASS: no rustls, rustls-webpki or rustls-pki-types |
| Bertie features | Unchanged default = api + std + defensive; exact rev 03374d49c29f5dad55252584fa362ade0ea3010e at file://~/src/bertie-deps |
| Bertie used-member tests (Docker, release) | PASS: member bertie only; 15 library + 2 handshake + 1 doctest; zero failed, 2 existing ignored, 1 network-dependent test filtered as in bertie-der |
| Bertie hax 0.4.2 extraction + F* Der / Tls13formats | BLOCKED before extraction: original scripts' read-only toolchain bind source no longer exists; no fresh proof result claimed |
| Catalog / protected scope | PASS: 9 entries, 95 spec functions, zero errors; no production Rust, tests, proofs or core configuration changed |
| Nix dependency gate (`nix-build.sh deny-check`) | FAIL: 6 vulnerability advisories plus proc-macro-error2 unmaintained; bans/licenses/sources ok; pure local fetch works |
| Nix full flake check | NOT RUN: conditional B3 blocked by failed Nix dependency gate |
| Static image | NOT BUILT: failed dependency gate; no ungated image |
| Container / port 8099 | No bertie-boundary container; no proven-io-ref created; existing containers unchanged |
| 18 byte-identical HTTP comparisons | NOT RUN: no accepted image/container |
| In-binary audit of shipped image | NOT RUN: no shipped image. Historical 19b source audit 1,199 Bertie lines / 671 functions; stage 17 TLS total 5,095; neither is a new image measurement |
| Formal coverage | NOT MEASURED: executable unavailable on PATH; not installed |

### Dependency versions and exceptions

| Dependency | Before | After |
|---|---|---|
| libcrux-kem | 0.0.2 | 0.0.2 (blocked; fixed >=0.0.10 removes Kyber-draft support) |
| libcrux-sha3 (old KEM) | 0.0.2 | 0.0.2 (blocked by KEM's exact pin; fixed >=0.0.10) |
| libcrux-ecdh | 0.0.2 | 0.0.2 (blocked by KEM's exact pin; fixed >=0.0.6) |
| libcrux-chacha20poly1305 | 0.0.2 | 0.0.9 |
| libcrux-poly1305 (Bertie) | 0.0.2 | 0.0.6 (exact dependency of AEAD 0.0.9) |
| libcrux-ed25519 | 0.0.3 | 0.0.9 |
| p256 (scratch-server) | 0.13.2 | 0.14.0 (stable) |
| elliptic-curve | 0.13.8 | 0.14.3 |
| crypto-bigint | 0.5.5 | 0.7.11 |

AEAD 0.0.8 and Ed25519 0.0.7/0.0.8 pin hax-lib =0.3.6 transitively;
our existing graph needs ^0.3.7. Both 0.0.9 releases are the smallest compatible
patched choices. No Bertie API/source adaptation is needed for these independent
updates. Stable p256 0.14 preserves the existing key-loader API unchanged;
crypto-bigint is patched without a prerelease, rewrite or advisory exception.

Exactly two documented unmaintained exceptions: RUSTSEC-2024-0436 (paste 1.0.15)
and RUSTSEC-2024-0370 (proc-macro-error 1.0.4), each pulled by hax-lib-macros 0.2.0
(proc-macro). Final all-target/all-feature inverse trees confirm those paths;
normal production tree with proc macros excluded contains neither. No linkage to
the server binary. The old cfg(hax)-only RUSTSEC-2026-0173 ignore is removed to
honor the explicit two-only policy; proc-macro-error2 now fails the gate as an
additional unmaintained dependency. No other ignore, snapshot or clock change.

Remaining vulnerabilities: RUSTSEC-2026-0023 (ecdh); RUSTSEC-2026-0330/0331 (kem);
RUSTSEC-2026-0074/0207/0208 (sha3). Cargo-deny no longer reports crypto-bigint,
ChaCha20-Poly1305, Poly1305 or Ed25519 vulnerabilities. Full receipts are under
`logs/stage19c-*`; the gate itself is `stage19c-deny-log.log`.

### Interop matrix — dependency update

All certificate/hostname verification stays enabled in the existing scripts. No
stage-19 image is available, so none of the following probes ran in 19c.

| Probe | Stage 19c observed |
|---|---|
| /usr/bin/curl default | NOT RUN — dependency advisory gate |
| /usr/bin/curl explicit TLS1.3 | NOT RUN — dependency advisory gate |
| /opt/homebrew/opt/curl/bin/curl default | NOT RUN — dependency advisory gate |
| /opt/homebrew/opt/curl/bin/curl explicit TLS1.3 | NOT RUN — dependency advisory gate |
| group X25519 | NOT RUN — dependency advisory gate |
| group P-256 | NOT RUN — dependency advisory gate |
| group X25519MLKEM768 | NOT RUN — dependency advisory gate |
| suite TLS_CHACHA20_POLY1305_SHA256 | NOT RUN — dependency advisory gate |
| suite TLS_AES_128_GCM_SHA256 | NOT RUN — dependency advisory gate |
| suite TLS_AES_256_GCM_SHA384 | NOT RUN — dependency advisory gate |
| suite TLS_AES_128_CCM_SHA256 | NOT RUN — dependency advisory gate |
| suite TLS_AES_128_CCM_8_SHA256 | NOT RUN — dependency advisory gate |
| TLS1.2 only | NOT RUN — dependency advisory gate |
| no common group | NOT RUN — dependency advisory gate |
| no common suite | NOT RUN — dependency advisory gate |
| HRR supported X25519 but only P384 key share | NOT RUN — dependency advisory gate |
| garbage bytes | NOT RUN — dependency advisory gate |
| truncated ClientHello | NOT RUN — dependency advisory gate |
| ClientHello split across many records | NOT RUN — dependency advisory gate |
| slow trickle handshake | NOT RUN — dependency advisory gate |
| HTTP root and encrypted close_notify | NOT RUN — dependency advisory gate |
| HTTP 404 and encrypted close_notify | NOT RUN — dependency advisory gate |
| HTTP malformed and encrypted close_notify | NOT RUN — dependency advisory gate |
| 12s idle then request (15s idle allowance) | NOT RUN — dependency advisory gate |
| idle over 15s closes | NOT RUN — dependency advisory gate |
| 10s HTTP deadline against trickle | NOT RUN — dependency advisory gate |
| 64 connection cap | NOT RUN — dependency advisory gate |
| Default rustls client (spike's separate client scope) | NOT RUN — dependency advisory gate |

### Andrew's browser checklist — dependency update

Blocked until dependency, Bertie proof, image and automated live gates pass.

- [ ] Safari: certificate-verified root and /health on 8099.
- [ ] Firefox: certificate-verified root and /health on 8099.
- [ ] Brave: certificate-verified root and /health on 8099, including cache-disabled request.

No new trusted production function. The existing 11 TLS functions and 28-function /
487-SLoC shell inventory remains unchanged (not formal coverage). record.rs and
aead.rs remain verified and unused by production; those proofs do not cover
Bertie's record layer. No independent, browser, x86_64, fresh fuzz/coverage or
end-to-end TLS proof claim. Historical coverage remains the stage-13 baseline,
644/1,363 first-party security SLoC (47.25%), not a new stage-19 measurement.
No push, external message, key copy/print or existing-container mutation.

Final local receipt: `logs/stage19c-summary.json`; native proof/test/mutant and
10,000-handshake gates pass on the final pin. All twenty mutants have distinct
identities, zero refinement errors and actual failed-test evidence. No fresh
Bertie proof extraction/typecheck or independent acceptance is claimed. Native
cargo-deny was attempted offline and lacks a local database; the completed Nix
check is the substantive advisory verdict, using the unchanged pinned snapshot.

B2 commit `aacb09c` records the dependency/policy subset and native receipts as
Dudley. B3 commit `5757723` records the blocked release, with no image build or live action. Both the
container and bertie-boundary image inventories are empty at the final check.
Nothing is pending push: this is a dependency/proof-environment blocker, and no
publishing action has been authorized or attempted.


## Results — 2026-11-21 (our Bertie) — superseded by E below

**Not accepted.** D1 is locally complete; D1b's client gate reveals a production
healthcheck dependency on Client/get_server_info (src/tls.rs:183). The requested
client-free production build cannot preserve that healthcheck unchanged. Andrew's
decision is pending; no production feature enablement or healthcheck change is assumed.
These are Codex-run checks; independent acceptance remains outstanding.

| Check | Observed result |
|---|---|
| Bertie D1 | d34ad5e: 25 workspace tests pass, zero failures; 3 ignored, 1 external-network test filtered |
| Bertie D1b | Final e1af495 (including 2f2c2c2 wire tests): 25 workspace tests pass, zero failures; 3 ignored, 1 external-network test filtered |
| Retained groups | X25519MLKEM768, X25519, P-256: all stream handshakes pass; wire IDs/lengths/shared-secret agreement pinned |
| Removed configurations | RSA/Ed25519 implementations/dependencies removed; AES/RSA/Ed25519/X448/PSK reject at client/server API |
| Bertie dependencies | KEM 0.0.2→0.0.10; ECDH 0.0.2→0.0.9; old SHA3 0.0.2→0.0.11; RSA 0.0.2 and Ed25519 0.0.9 removed |
| Production features/tree | No client feature; no rustls/RSA/Ed25519; 112→109 unique production package lines including root/workspace |
| Native verify.sh | PASS: 211 parser / 347 transport / 9 AEAD / 30 standalone hello, zero errors |
| Locked scratch workspace tests | FAIL TO COMPILE: E0432 production healthcheck imports client gated out by D1b |
| Twenty mutants | Proof baseline passes; test baseline fails compilation, zero new kills claimed |
| Strict 10,000 handshakes | Not run: production compile conflict |
| Nix cargo-deny | PASS: advisories/bans/licenses/sources all ok, unchanged snapshot/clock; native offline deny cannot evaluate without its local DB |
| Exceptions | Authorized build-only unmaintained proc-macro-error2 RUSTSEC-2026-0173; two existing approved exceptions now unmatched |
| D2 toolchain/proofs | Rebuilt pinned nightly/hax/F*/Z3; baseline/current extraction PASS (21 files each); Der PASS (333 s); six strict checks FAIL at crypto.fsti:9, missing Non_hax namespace; stopped |
| Nix check / static image | Full check FAILS E0432 healthcheck import; image not built after failed gate |
| Container 8099 / temporary reference | None created; other containers unchanged |
| 18 byte-identical requests / live interop | Not run: no accepted image |
| D1b in-binary lines | Fresh before audit: 1,200 / 665 functions; after unavailable while production cannot compile |
| Formal coverage | No fresh measurement or independent/browser acceptance claimed |

No scratch-server parser/transport/dispatch/deadline/admission/proof/mutant source
was changed. Shared Bertie formats and proof definitions remain; whole removed items
and limits are listed in dev/BERTIE-DEPS-REPORT.md. No assume/admit/lax/new patch.

| Live interop | Result |
|---|---|
| Certificate-verified clients and all three retained groups | NOT RUN — production compile conflict |
| Unsupported algorithms, protocol/deadline/admission probes | NOT RUN — production compile conflict |

### Andrew's browser checklist — our Bertie

Blocked until proof, compile, image and automated live acceptance passes.

- [ ] Safari: certificate-verified root and /health on 8099.
- [ ] Firefox: certificate-verified root and /health on 8099.
- [ ] Brave: certificate-verified root and /health on 8099, cache disabled.


## Results — 2026-11-21 (E)

Supersedes the 19d results above. Andrew authorized the RNG-boundary repair and
unauthenticated ServerHello readiness probe. Bertie E1 `e6ea574`; scratch E2
`a923749`. These are Codex-run checks; independent acceptance and physical
browser checks remain Andrew/Dudley's.

| Check | E result |
|---|---|
| Bertie workspace | PASS: 25 tests, zero failures; 3 original ignored, 1 external-network filtered; all three retained groups |
| Hax / strict F* | PASS: 21 fresh extraction files; Der, crypto interface, formats, API, handshake and both key schedulers, all seven exit 0 / all VCs discharged |
| Native Verus | PASS: 211/347/9 plus standalone hello30, zero errors (overlapping roots) |
| Workspace tests | PASS: 60 passed, zero failed; strict detector ignored here, run separately |
| Mutants | PASS: 20/20 caught, zero survivors/invalid; all refinement gates pass, all twenty actual test failures, 17 intent kills |
| Strict native handshakes | PASS: 10,000 attempted, zero failed, 494.90 seconds |
| Production target trees | PASS: no Bertie client feature, rustls/webpki/pki-types, RSA or Ed25519; 109 package identities including root |
| Nix cargo-deny | PASS: all four categories, snapshot/clock/policy unchanged; native offline DB absent, no native advisory verdict |
| Nix flake check / image-static | PASS: aarch64-linux; x86_64 server omitted, not tested |
| Container | PASS: bertie-boundary left running healthy on 127.0.0.1:8099; all local automated gates pass |
| Reference / HTTP equivalence | PASS: 18/18 byte-identical, certificate verified on both endpoints; proven-io-ref removed |
| Interop | PASS: 26 passing rows and 2 local client-unsupported cases; certificate/hostname verification enabled |
| In-binary audit | Stage17-method debug companion: Bertie 736 lines / 409 functions; before 1,200 / 665. Old TLS crates: zero production dep-info. Root-inclusive production packages 112→109 (111→108 dependencies). |
| Coverage | No current whole-server percentage: formal-coverage unavailable, not installed. Historical 644/1,363 (47.25%) describes source 2fa5eb7 only. Trusted shell inventory: 29 functions / 469 SLoC. |

### E proof and healthcheck boundaries

E1 moves the two RNG-consuming libcrux calls into the existing non_hax module,
keeping rand_core trait instances out of the extracted crypto interface. E1's
only extracted diff is removal of its unresolved implicit open. No proven item,
proof/model patch, or new assume/admit/lax directive; 147 model files unchanged.
The three RNG forwarding methods and two KEM wrappers are trusted, documented in
Bertie's TRUSTED.md and [the dependency report](../../dev/BERTIE-DEPS-REPORT.md).
Existing abstract crypto assumptions remain; these checks do not prove libcrux
or an end-to-end TLS security theorem.

The healthcheck's constant 208-byte ClientHello offers the supported suite,
signature and all three groups, with valid X25519/P-256 public basepoint shares.
It uses no RNG/TLS client, reads one <=16,384-byte first record, requires 0x16 /
0x0303, plausible record/handshake lengths and ServerHello type 0x02, then shuts
down both directions. One absolute two-second budget covers connect/write/reads;
a short write fails. Main exits 0/1. Four focused tests cover a real healthy
server (zero HTTP dispatch), closed port, non-TLS, alert, invalid version/length/
type, truncation and slow partial-read timeout. Executable positive/closed-port
exit codes also pass. This is unauthenticated readiness, not certificate or HTTP
verification. See [trusted base](../../verification/TRUSTED-BASE.md).

### E image audit method

The gated image is a0db071cbc3f6d3297a2cd273f25bbdcf0e13ae5b9d4d9c2007b2b190cf29af1;
its extracted executable SHA256 is
4b65f2bfad349e33f283e867593442bed7fdac4fc8cc325335df3facd8508af9.
The prescribed stage17 DWARF companion keeps debug information with the same
source/target/profile, all other codegen settings unchanged. It is an upper bound
on source retained by that companion, not runtime coverage or byte equivalence:
.text is 616,344 bytes versus the stripped image's 616,792. Common masked AArch64
instruction multiset is 97.8813%, which is not a semantic-equivalence proof.
Reproduce: NIX_LOG_PREFIX=stage19e verification/scripts/bertie-surface-build.sh,
then bertie-surface.py with the same prefix, then scripts/stage19e-image-audit.py.
The before 1,200-line result uses the same method. Stage17 rustls/webpki/pki-types
baseline was 5,095 lines; no new formal-coverage percentage is inferred.

### E interoperability

The first spike-derived run stopped at its empty-EOF trickle assertion: this
server's unchanged deadline contract is HTTP 400 at ten seconds. The final
stage19e wrapper retains all rows/certificate checks, requires 400, authenticated
closure and 10<=elapsed<12, and checks both proven-io-ref and the new server.
This is a protocol-response adaptation to the established stage16 test, not a
server/deadline change or relaxed timing assertion. Original spike/stage19 matrix
scripts are unchanged. First failed receipts are retained as stage19e-first-*.
Final run: scripts/stage19e-live.py with SCRATCH_TLS_DIR pointing at the existing
read-only TLS directory; reference removed even on failure.

| Probe | Result |
|---|---|
| /usr/bin/curl default | PASS |
| /usr/bin/curl explicit TLS1.3 | CLIENT UNSUPPORTED/FAIL |
| /opt/homebrew/opt/curl/bin/curl default | PASS |
| /opt/homebrew/opt/curl/bin/curl explicit TLS1.3 | PASS |
| group X25519 | PASS |
| group P-256 | PASS |
| group X25519MLKEM768 | PASS |
| suite TLS_CHACHA20_POLY1305_SHA256 | PASS |
| suite TLS_AES_128_GCM_SHA256 | PASS |
| suite TLS_AES_256_GCM_SHA384 | PASS |
| suite TLS_AES_128_CCM_SHA256 | PASS |
| suite TLS_AES_128_CCM_8_SHA256 | CLIENT UNSUPPORTED/FAIL |
| TLS1.2 only | PASS |
| no common group | PASS |
| no common suite | PASS |
| HRR supported X25519 but only P384 key share | PASS |
| garbage bytes | PASS |
| truncated ClientHello | PASS |
| ClientHello split across many records | PASS |
| slow trickle handshake | PASS |
| HTTP root and encrypted close_notify | PASS |
| HTTP 404 and encrypted close_notify | PASS |
| HTTP malformed and encrypted close_notify | PASS |
| 12s idle then request (15s idle allowance) | PASS |
| idle over 15s closes | PASS |
| 10s HTTP deadline against trickle | PASS |
| 64 connection cap | PASS |
| rustls default config | PASS |

PASS for unsupported protocol/suite/group rows means clean rejection; it does not
mean AES/TLS1.2/HRR became supported. The two client-unsupported rows are system
curl's explicit TLS1.3 option and OpenSSL's TLS_AES_128_CCM_8_SHA256 selection.
The default rustls client verifies the certificate and negotiates TLS1.3 /
ChaCha20-Poly1305 / X25519MLKEM768.

### E browser checklist (manual; outstanding)

- [ ] Safari: certificate-verified root and /health on 8099, no trust-warning bypass.
- [ ] Firefox: certificate-verified root and /health on 8099.
- [ ] Brave: certificate-verified root and /health on 8099, cache disabled.

E acceptance: all requested automated gates pass locally. Manual browsers and
independent acceptance remain outstanding. README now describes the current
server, source/build requirements, dated assurance evidence and limits; stage
narratives stay here and in the linked records. E4 changes only README; E3 records
and code are committed separately. No push or GitHub action. E2 a923749; E3 and
README-only final revisions follow in git log.

## Results — 2026-11-26 (bertie-tls 380c23f)

Andrew approved moving the bertie-tls pin from `9b55643a3724bf347dd45e998844bf18776dfccd`
to bertie-tls main `380c23fbd2f96e22c73727538fa14b20c89b9a89`, so the Shipped tree
credits Bertie's F* proofs. The version requirement and the eleven enabled features are
unchanged; no feature was renamed. Bertie's own Cargo.lock is byte-identical at both
revisions (blob `90a5f9380702f9f054614b4ff996afb667653cf2`). scratch-server's Cargo.lock
changes only the bertie-tls source line. No scratch-server code changed: the API this
server uses compiles unchanged. These are Codex and Claude Code runs on one arm64 Mac,
loopback only. Independent acceptance remains Andrew/Dudley's.

| Check | 380c23f result |
|---|---|
| Workspace tests (`--locked --no-fail-fast`) | PASS: 60 passed, zero failed, 1 ignored (strict detector, run separately) |
| Native Verus | PASS: 211/347/9 plus standalone hello 30, zero errors (overlapping roots) |
| Hax / strict F* | PASS: all seven stage-19 checks exit 0 on a fresh 380c23f extraction (see below) |
| Mutants | PASS: 20/20 caught, zero survivors or invalid runs; all twenty failed tests, 17 intent kills (`logs/stage19-mutants/results.json`, ignored) |
| Strict native handshakes | PASS: 10,000 attempted, zero failed |
| HTTP equivalence | PASS: 18/18 byte-identical against the same scratch-server source on the 9b55643 pin |
| Interop | PASS: 26 passing rows and the same 2 local client-unsupported cases as E |
| Nix coverage | PASS: receipt built at 380c23f, Bertie F* verification exit 0; first-party numbers unchanged |
| Not rerun | Nix flake check, static image, container, cargo-deny, in-binary audit, browsers |

F* checks: the historical stage19d/19e extractors hard-code Bertie revisions e1af495
and e6ea574. They were rerun against 380c23f with the same F* invocations, the new
`Bertie_tls` module prefix and fresh include directories. The extraction now has 37
files (18 implementations, 19 interfaces) instead of 21; the crypto backend stays
interface-only. Der, Tls13crypto.fsti, Tls13formats, Tls13api, Tls13handshake and both
key schedulers each exit 0, with no proof cache or lax options. The checks still assume
the abstract crypto models; they do not prove libcrux or an end-to-end TLS theorem.

HTTP equivalence: against the historical `proven-io:static` image, 16 of 18 cases
are byte-identical. GET / and HEAD / differ (Content-Length 125 versus 65), and the
base branch on the 9b55643 pin shows the same difference, so it predates this move.
That reference ran as a temporary loopback container, `bertie380-ref`, since removed.
Against the immediate base build, all 18 cases are identical.

Interop: `scripts/stage19e-interop.py` and `scripts/stage19-compare.py` ran against a
native loopback server built from this revision, with a loopback reference server built
from the base branch. The row matrix matches the E table. The
"rustls default config" row (TLS 1.3, ChaCha20-Poly1305, X25519MLKEM768, certificate
verified) ran the spike's default rustls client from a copy under `target/`.

Coverage (`nix build .#coverage`, formal-coverage v1.13.0, aarch64-darwin):

| | Proven | Tested | Trusted | Unverified |
|---|---|---|---|---|
| bertie-tls receipt at 9b55643 (no F* script) | 0 | 1,151 | 25 | 3,655 |
| bertie-tls receipt at 380c23f | 4,063 | 208 | 161 | 444 |
| Bertie CI, 380c23f, default features | 4,063 | 253 | 161 | 553 |
| Shipped tree before | 887 | 1,366 | 25 | 300,403 |
| Shipped tree after | 4,950 | 423 | 161 | 297,786 |
| First party, before and after | 887 | 215 | 0 | 264 |

The receipt reports 175 verified F* files and 310 credited units. The gap to Bertie's
CI record is exactly `src/stream/client.rs` (45 tested and 109 unverified lines). That
module needs the default `client` feature, which this server leaves off. Every other
unit matches CI in lines and status. Bertie's shipped files grew from 6,588 to 7,227
lines between the two revisions. 2,351 of them are outside the receipt and stay
unverified in the Shipped tree (1,757 before).
