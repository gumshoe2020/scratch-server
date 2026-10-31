# Progress

## Stage 16 — proven-io

Resumed at 401bbe2 after Andrew approved the admission-overflow exception.
The guarded core fix and the ten-second request deadline are the only permitted
behavior changes. All Codex acceptance commands pass, including build/live/mutants. Ready for
Dudley's independent acceptance and coverage measurement. Historical blocker
evidence is retained below.

- [x] `./verification/verify.sh`: new properties, zero errors; prohibited-token scan; PROOFS.md property locations.
- [x] `cargo test --workspace`: inherited tests plus full deadline/classification/admission/port oracles.
- [x] Plain TCP and TLS deadline tests: trickle 400 in 10–12 s, complete over 8 s 200, idle 3 s 200, idle 17 s zero bytes.
- [x] Stage 13 spec mutants: 15/15 killed in temporary copies, zero survivors/invalid; all refinement gates pass (logs/stage16-mutants/results.json).
- [x] `./nix-build.sh check` and `image-static`: static, shell-free, key-free image tagged proven-io:static.
- [x] Hardened proven-io on 8098: stage 15 live suite, response/log equivalence to 8097, comparative trickle test.
- [x] Protected containers untouched; branch remains proven-io; no push or protected-document changes.
- [x] TRUSTED-BASE.md complete function inventory/subtotals; NOTES.md moved/added functions and before/after SLoC (63 shell functions / 618 SLoC; audit receipt logs/stage16-function-inventory.json).

Blocker validation (separate from stage acceptance):

- [x] Debug and release admission evidence tests: 201 matching counts and the required MAX boundary, two tests passed in each profile.
- [x] Baseline workspace validation: 46 passed, including all 44 inherited tests; prohibited-token scan zero matches across 19 Rust files.
- [x] Baseline `./verification/verify.sh`: 211/0 parser, 298/0 transport, 9/0 standalone AEAD; no new proofs claimed.

## Stage 15 — plaintext capacity fix

Preallocate `inner_plaintext` for payload plus content type; preserve its contract.
Add an exact-capacity regression for payload lengths 0, 1, 255 and 16,384.

- [x] Verus roots: 211/0 parser, 298/0 transport, 9/0 AEAD; no proof adjustment.
- [x] Stage 15 prohibited-construct scan: zero matches across 18 tracked Rust files.
- [x] Full workspace suite: 44 passed (43 inherited tests plus one capacity test).
- [x] Spec-mutant campaign: 15/15 killed, zero survivors/invalid; all refinement phases pass.

Receipts use `logs/stage15-capacity-`; independent acceptance remains Dudley's.

## Stage 14 — proven-dispatch

Implementation resumed after Andrew approved resolution A in spec amendment 0cb7016.
Bare CR/LF compatibility tests are retained. All stage acceptance commands passed;
ready for Dudley's independent acceptance and coverage measurement.

- [x] `./verification/verify.sh`: final source 211/0 parser, 285/0 transport, 9/0 AEAD; prohibited-token scan zero matches; PROOFS.md lists properties/locations and remaining shell (logs/stage14-verify-final.log).
- [x] `cargo test --workspace`: all 31 existing tests plus 8 evidence/equivalence/dispatch tests (39 total), logs/stage14-tests-final.log.
- [x] `python3 verification/scripts/spec-mutants.py`: 15/15 killed, 0 survivors, 0 invalid; every refinement phase passed; runtime/config/fixture hashes unchanged (logs/stage14-mutants/results.json).
- [x] `./nix-build.sh check` and `./nix-build.sh image-static`: final aarch64-linux checks pass; exported image is static, shell-free and key-free (stage14-nix-final-check/image/live logs).
- [x] Live hardened `proven-dispatch` on 8096: TLS/curl, delayed 200, idle 0 bytes, all browser fixtures, 48 malformed cases; 14 byte-identical rejection responses/stdout comparisons with 8095 (logs/stage14-live.log).
- [x] NOTES.md inventory of every moved/added function, location, proven/shell/test status and SLoC (logs/stage14-inventory.py and JSON receipt).

### Preserved compatibility evidence (resolved by 0cb7016)

- [x] Preserve bare-CR/bare-LF name spans and strict-policy diagnostics under resolution A.
- [x] Run unchanged baseline Verus roots and workspace tests with the evidence tests: `./verification/verify.sh` 211/0 parser, 237/0 transport, 9/0 AEAD; workspace 34/0; prohibited-token scan zero matches.

## Stage 15 — proven-record

All listed commands passed. Ready for Dudley's independent acceptance and coverage measurement.

- [x] `./verification/verify.sh`: 211/0 parser, 298/0 transport, 9/0 AEAD; prohibited-token scan zero matches; PROOFS.md updated (logs/stage15-core-verify-final.log).
- [x] `cargo test --workspace`: 43/0, all 39 inherited tests unchanged plus four record equivalence tests (logs/stage15-tests.log).
- [x] `python3 verification/scripts/spec-mutants.py`: 15/15 killed, 0 survivors/invalid, all refinement phases pass; working-tree hashes preserved (logs/stage15-mutants/results.json).
- [x] `./nix-build.sh check` and `./nix-build.sh image-static`; static, shell-free, key-free inspection passed (stage15-nix-check/image/live logs).
- [x] Hardened `proven-record` on 8097; TLS/curl, delay/idle, browser fixtures; 18 byte-identical response/log comparisons with 8096 (logs/stage15-live.log).
- [x] Protected containers unchanged: identity/pid/start-time snapshots compare equal with `cmp` (stage15-protected-before/after.log).
- [x] NOTES.md complete moved/added function inventory and TLS per-function before/after SLoC (logs/stage15-inventory.py, stage15-function-inventory.json).

## Stage 17 — rustls-surface

- [x] M1 reproducible crate total / compiled / in-binary line and symbol measurements (verification/scripts/rustls-surface-build.sh, verification/scripts/rustls-surface.py; rerun output identical); Results table.
- [x] Attacker-facing module counts and unused compiled features documented (RUSTLS-SURFACE.md Results, logs/stage17-m1.json).
- [x] All three cargo-fuzz targets build using the production configuration and shell (`cargo fuzz build -O`, logs/stage17-fuzz-build.log).
- [x] OpenSSL, curl (Homebrew/OpenSSL and system) and rustls ClientHello seed corpora captured without secrets (scripts/stage17-capture-hellos.py, fuzz/seeds/hellos).
- [x] Each target fuzzed for 30 minutes, no findings; logs and execution/corpus counts (logs/stage17-fuzz-*.log).
- [x] Corpus coverage measured against M1 in-binary line sets for all three crates (verification/scripts/stage17-coverage.py, logs/stage17-coverage-summary.txt).
- [x] Unchanged ./verification/verify.sh passes 211/317/9, zero errors (logs/stage17-verify.log).
- [x] Unchanged cargo test --workspace passes 52 tests (logs/stage17-cargo-test.log).
- [x] Scope, branch, secret and runtime-change audit; local checkpoint commits (no diff under src/ crates/ config/ flake.* Cargo.*; no keys tracked; no containers left; not pushed).
