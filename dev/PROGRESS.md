# Verified boundary acceptance checklist

Only checked items have passed the recorded command. Independent review remains Dudley's.

- [x] M1: unbounded safety, grammar/fields equivalence and limits; `./verify.sh`
- [x] M1: PROOFS.md properties, references and trusted boundary
- [x] M2: normal routes, HEAD and healthcheck; malformed corpus >=30 and zero handler calls
- [x] M3: >=100,000 differential cases, zero mismatches, regressions retained
- [ ] M4: primary-source AWS-LC coverage and restricted TLS configuration
- [ ] M4: runtime certificates and external-directory certificate generation
- [ ] M4: static musl AWS-LC build (not attempted: earlier coverage blocker)
- [x] M5: `./nix-build.sh image-static` builds/loads image
- [x] M5: containerized `nix flake check`: tests, cargo-deny and pinned proofs if feasible
- [x] M5: exported filesystem binary only, static executable, image <8 MB
- [x] M5: hardened verified-boundary:8090; routes and healthcheck; shell fails; left running

M1 acceptance: 2026-10-27, `./verify.sh` exits 0 (18 verified, 0 errors); `CARGO_HOME="$PWD/.tools/cargo" cargo check -p http-subset` exits 0. PROOFS.md reviewed against the source references.

M2 acceptance: 2026-10-27, `CARGO_HOME="$PWD/.tools/cargo" cargo test --workspace` exits 0: 48 malformed inputs each at read chunks 1/7/1024, zero dispatch calls; normal/HEAD/404/TCP and appended-byte tests pass. Live curl `/` and `/health`, binary `--healthcheck` passed with SCRATCH_PORT=8090. `./verify.sh` rerun: 18 verified, 0 errors.

M3 acceptance: 2026-10-27, `CARGO_HOME="$PWD/.tools/cargo" cargo test --test differential` exits 0: 50,000 valid and 50,000 mutated/arbitrary requests, zero mismatches (23.91 seconds). `cargo test --test differential grammar_boundaries_and_regressions` also passed, including exact 4096/4097-byte boundary. No counterexamples found.

M4: blocked 2026-10-27 on published implementation/API/CPU coverage, documented in TLS-COVERAGE.md and NOTES.md. Certificate preparation script syntax and repo-path refusal passed; generation/runtime TLS not tested. TLS acceptance items remain unchecked. No musl AWS-LC build result claimed.

M5 acceptance: 2026-10-27:

- `./nix-build.sh versions`: pinned aarch64-linux rustc 1.99.0, cargo-deny 0.20.2, cargo-fuzz 0.13.2; no nixpkgs Verus/Kani attributes.
- `./nix-build.sh check`: exits 0. Verus pinned separately with Rust 1.99.1: 18 verified, 0 errors; full tests including 100,000 proptest cases; cargo-deny advisories/bans/licenses/sources passed. Offline yanked-crate detection unavailable (warnings retained). x86_64-linux not built/tested.
- `./nix-build.sh image-static`: exits 0, loaded `verified-boundary:static`.
- `LAN_HOST=203.0.113.28 ./scripts/acceptance.py`: exits 0. Image size 1,343,421 bytes (<8 MB); binary 648,992 bytes; ELF has no interpreter or DT_NEEDED. Saved image layers contain only the binary; container export additionally has Docker-injected metadata/network/console entries.
- Same acceptance script: / and /health on loopback and host LAN address, --healthcheck, all 48 malformed requests rejected, `docker exec verified-boundary sh` fails, uid/gid 65532, read-only, all capabilities dropped, no-new-privileges, 8090 -> 8080. Container left running and Docker health is healthy. No second LAN device was available/tested.
- Final `./verify.sh`: exits 0, 18 verified, 0 errors. `git diff --check` clean.

Independent acceptance review by Dudley remains outstanding. M4 remains blocked; M1–M3 and M5 are accepted by the commands above, not a claim of end-to-end verified networking.

## Stage 4 — browser-whitelist

- [x] Nine-name specification and proofs; `./verify.sh`; PROOFS.md updated.
- [x] Captured Safari requests whole and at 1/7/1024-byte reads; boundary tests.
- [x] Original 48 malformed cases plus new case/value/prefix cases; zero handler calls.
- [x] Rejection status/name-only logging, sanitization and 64-byte cap; native workspace tests and `./scripts/acceptance.py` passed.
- [x] M3: 100,000 differential cases, zero mismatches.
- [x] `./nix-build.sh check` (aarch64-linux, exit 0).
- [x] `./nix-build.sh image-static` and `./scripts/acceptance.py`; static, shell-free image under 8 MB.
- [x] Hardened browser-whitelist:8091; Safari curl replay; `./scripts/acceptance.py` passed; Docker inspect healthy; left running.
- [x] Protected containers unchanged; Python comparison of `docker inspect` against pre-deployment snapshot passed; final notes recorded.

2026-10-27 14:32 ET: `./verify.sh` exit 0, 18 verified/0 errors; proof exemptions search in crates/http-subset/src, src, tests found none. `CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" cargo test --workspace` exit 0: Safari fixtures whole/1/7/1024, 48 original malformed cases plus 18 new malformed cases with zero dispatch calls, 256-byte accepted new-header values, diagnostic sanitization/cap tests, 50,000 valid plus 50,000 mutated/arbitrary differential cases (zero mismatches). `./nix-build.sh check` exit 0; `./nix-build.sh proof-log` confirms 18/0 and `./nix-build.sh test-log` confirms all 100,000 cases pass in Linux.

2026-10-27 14:34 ET final acceptance: preserved captured Referer verbatim (port 8099), with only Host changed to 8091; native boundary test, Nix check, static image build and `./scripts/acceptance.py` all rerun and passed. Final image sha256:d025f684e2ea310dcf51f0f91fef2897aba8dc5d4959feb769ff9a8b816683af, 1,353,661 bytes; binary 651,224 bytes. Script confirms both captured statuses/connection-close, curl 200, Docker stdout exactly `status=400 unknown_header=Cookie` for Cookie: x, 48 malformed 400s, routes/healthcheck, ELF without interpreter/DT_NEEDED, one image binary, shell absence, uid 65532, read-only/cap-drop/no-new-privileges and 8091 binding. Final `./verify.sh` passed 18/0; `git diff --check` and `git diff --cached --check` passed. Protected containers retained same identity/configuration/mounts/PID/lifecycle. No TLS, x86_64 or second-device LAN acceptance claimed. Dudley's independent acceptance remains outstanding.

## Stage 5 — https

- [x] Crypto operation/source coverage table in TLS-PROVIDER.md; published package README/API review, primary HACL sources, `test -s TLS-PROVIDER.md`.
- [x] Custom libcrux provider: TLS 1.3/ChaCha20Poly1305/SHA256/X25519, http/1.1 only; workspace TLS tests and `./nix-build.sh check` passed.
- [x] External runtime certificate; `./scripts/make-lan-cert.py .../scratch-server-tls 203.0.113.28` passed; acceptance stat checks 700/600 passed; `openssl x509 -noout -checkip 203.0.113.28` passed.
- [x] Rust TLS handshake/root/Safari/malformed-corpus tests; zero malformed dispatches; `CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" cargo test --workspace` (5 lib tests, 6 boundary, 3 differential).
- [x] External OpenSSL suite/group/version and negative-policy checks, curl HTTPS, plaintext HTTP refused; `LAN_HOST=203.0.113.28 ./scripts/acceptance.py` exit 0.
- [x] Unchanged parser proofs: `./verify.sh` (18/0); workspace tests pass 100,000 differential cases; parser diff empty.
- [x] `./nix-build.sh check` (aarch64-linux proofs/tests/cargo-deny) exit 0; proof/test/deny logs retrieved.
- [x] `./nix-build.sh image-static` exit 0; `./scripts/acceptance.py` confirms ELF static, shell-free image/export binary only, no certificates/keys in image.
- [x] Hardened https:8092 left running healthy; acceptance Docker inspect checks passed. Python before/after snapshot comparison passed: protected IDs/config/mounts/PIDs/lifecycle unchanged.
- [x] Final evidence/trusted-boundary summary in NOTES.md; `git commit` passed locally on https (no push).

2026-10-27 14:49 EDT: final runtime acceptance passed. Image sha256:4201e75b7a801afac4306f7f85ea432c302e2bbce7c6ff5cb9ba2f5ec7555c7b; 3,096,498 bytes, binary 1,523,248 bytes. OpenSSL TLS 1.3/ChaCha20Poly1305/X25519; TLS 1.2, AES suite, P256 KX, h2ALPN fail. Curl via host LAN address 200, plain HTTP refused. Both Safari fixtures/root/health/cookie privacy/malformed 48 cases passed. Rust TLS malformed 66 cases assert zero handler calls. Final ./verify.sh 18/0. Physical Safari and second-device LAN tests and independent Dudley acceptance remain outstanding; upstream crypto proof release/target correspondence remains unconfirmed. Evidence in logs/stage5-*.

## Stage 6 — https-headers

- [x] Twelve-name spec, PROOFS.md and no escape hatches; `./verify.sh`.
- [x] HTTPS Safari fixture whole/1/7/1024 TLS writes, favicon 404; workspace tests.
- [x] Original malformed corpus and new wrong-case/near-miss inputs 400 with zero handlers; workspace tests.
- [x] 100,000 differential cases over 12 names, zero mismatches; workspace tests.
- [x] `./nix-build.sh check` (proofs/tests/cargo-deny).
- [x] `./nix-build.sh image-static`; static/shell-free/key-free image; acceptance script.
- [x] Stage 5 OpenSSL/curl policy checks on 8093 and fixture curl 200; acceptance script.
- [x] Hardened https-headers:8093 running healthy; acceptance script.
- [x] Protected containers unchanged; before/after inspect comparison.
- [x] Final trusted-boundary notes; `test -s NOTES.md`.

Native commands passed: `./verify.sh` (18 verified, 0 errors); `CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" cargo test --workspace` (5 library, 6 boundary, 3 differential tests; 100,000 cases, 0 mismatches). `rg` exemption scan found zero matches.

Final runtime: `LAN_HOST=203.0.113.28 ./scripts/acceptance.py` exit 0 (53 malformed 400s, TLS policy, fixture curl 200/favicon 404, static/shell-free/key-free image, healthy hardened 8093). `./nix-build.sh image-static` exit 0; image sha256:fe1e9a4dadd02972947de3304f8fd067daceab66806a86ffb8703bc4f96c7b13; 3096506 bytes. Python before/after `docker inspect` comparison passed all five protected containers. `cargo fmt --all -- --check`, fixture order/CRLF and certificate SAN checks passed. Physical Safari/second-device/x86_64 and independent acceptance remain untested.

## Stage 7 — idle-connections

- [x] Plain TCP and TLS: idle 3s then 200, idle past 15s with zero response bytes, partial request stalls then 400 with zero handlers; native cargo test --workspace.
- [x] Absolute idle budget includes TLS handshake, including no-handshake and trickled-record cases; native cargo test --workspace.
- [x] Classified grammar/size/slow/unknown_header/idle logs, sanitized names and no values; native cargo test --workspace and LAN_HOST=203.0.113.28 ./scripts/acceptance.py.
- [x] 64-worker cap closes excess connections and recovers released slots; LAN_HOST=203.0.113.28 ./scripts/acceptance.py.
- [x] Unchanged parser/spec, 18 verified / 0 errors; ./verify.sh. No escape hatches: rg scan exit 1; logs/stage7-exemptions.log empty.
- [x] Safari fixtures whole/1/7/1024, favicon 404, malformed corpus 400 with zero dispatch, 100,000 differential cases; native cargo test --workspace.
- [x] Linux proofs/tests/cargo-deny; ./nix-build.sh check (final source), proof-log, test-log and deny-log.
- [x] ./nix-build.sh image-static; static, shell-free, key-free image; LAN_HOST=203.0.113.28 ./scripts/acceptance.py.
- [x] Stage 5/6 OpenSSL/curl policy and fixture replay on 8094; LAN_HOST=203.0.113.28 ./scripts/acceptance.py.
- [x] Requested live delayed OpenSSL pipeline returns 200; (sleep 3; printf request; sleep 2) | openssl s_client -quiet -connect 203.0.113.28:8094 -tls1_3; logs/stage7-live-openssl.log.
- [x] Hardened idle-connections:8094 healthy and left running; LAN_HOST=203.0.113.28 ./scripts/acceptance.py; final docker inspect.
- [x] All six protected containers unchanged; Python docker-inspect comparison, logs/stage7-protected-comparison.log.
- [x] Formatting, unchanged parser/manifests/provider policy and clean diff; cargo fmt --all -- --check, cargo check --workspace, Python source comparison and git diff --check.
- [x] Final summary/trusted boundary; test -s NOTES.md.

Native Cargo commands used CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup".
Evidence is retained under logs/stage7-*. Local stage commit follows; no push.
Independent acceptance by Dudley and physical Safari testing remain outstanding.

## Stage 8

- [x] Core safety, parser-gated single dispatch, terminality, rejection classes and event bound; ./verify.sh.
- [x] Response status/length/injection/HEAD properties; ./verify.sh.
- [x] TLS glue proofs and explicit remaining limits; ./verify.sh and PROOFS.md.
- [x] Existing Safari/corpus/idle tests, shell event tests and 100,000 differential cases; cargo test --workspace.
- [x] Linux proofs/tests/dependency audit; ./nix-build.sh check and proof-log/test-log/deny-log (aarch64-linux).
- [x] Static image; ./nix-build.sh image-static.
- [x] Hardened proven-transport:8095, static/shell-free/key-free, OpenSSL/curl, delayed 3s and idle 17s/zero-byte checks; LAN_HOST=203.0.113.28 ./scripts/acceptance.py and delayed OpenSSL pipeline (logs/stage8-live-openssl-status.log).
- [x] Protected containers unchanged; Python before/after docker inspect comparison (logs/stage8-protected-comparison.log).
- [x] Formal-coverage before/after using unchanged stage 7 config; baseline tool report and python3 scripts/measure-coverage.py (explicit three-receipt unit union).
- [x] Final proof references and limitations in PROOFS.md and NOTES.md; Python reference/source checks, cargo fmt --all -- --check, python3 -m py_compile and git diff --check.

Stage 8 final executable source passed native and aarch64-linux tests after removing all response-size preconditions.
`./verify.sh`: 18/0, 43/0, 9/0 (overlapping modules).
Cargo commands used CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup".
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Final measurement: 128/737 (17.37%) →337/981 (34.35%) first-party security SLoC; explicit union of three source-identical formal-coverage reports, not a single multi-root tool report.
Evidence: logs/stage8-*.
No pushes.
Dudley's independent acceptance, physical Safari, second-device LAN, x86_64 and upstream F* remain untested.

## Stage 9 — firefox-headers

- [x] Thirteen-name parser specification and all proof groups; ./verify.sh.
- [x] Firefox TLS fixtures whole/1/7/1024, favicon 404, new rejections with zero handlers and all inherited tests; cargo test --workspace.
- [x] 100,000 differential cases over 13 names, zero mismatches.
- [x] Linux proofs/tests/dependency checks; ./nix-build.sh check.
- [x] Static image build; ./nix-build.sh image-static.
- [x] Hardened firefox-headers:8095 and inherited OpenSSL/curl/idle checks plus Firefox replay; scripts/acceptance.py.
- [x] Formal-coverage unchanged config and explicit three-run union.
- [x] Protected containers unchanged and final scope/format/privacy checks.

Stage 9 local commands passed: ./verify.sh 18/0, 43/0, 9/0 (overlapping groups); native workspace tests and aarch64-linux ./nix-build.sh check; image-static; LAN_HOST=203.0.113.28 ./scripts/acceptance.py and delayed OpenSSL request.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage: 338/983 first-party security SLoC=34.38%, explicit three-run union using unchanged config.
Evidence: logs/stage9-*.
Independent acceptance by Dudley, physical Firefox/Safari, second LAN device, x86_64 and upstream F* reruns remain outstanding.
Local stage commit follows; no push.

## Stage 10 — chromium-headers

- [x] Case-insensitive eighteen-name, 24-header grammar and all proofs; ./verify.sh.
- [x] Brave/Safari/Firefox TLS fixtures, mixed-case acceptance, near-miss/forbidden zero-dispatch and limits; cargo test --workspace.
- [x] 100,000 mixed-case differential cases over 0–24 headers, zero mismatches.
- [x] Linux proofs/tests/dependency audit; ./nix-build.sh check.
- [x] Static image; ./nix-build.sh image-static.
- [x] Hardened chromium-headers:8095, OpenSSL/curl/idle checks and Brave replay; scripts/acceptance.py.
- [x] Formal-coverage unchanged config and explicit three-run union.
- [x] Protected containers unchanged; final source/format/privacy checks.

Stage 10 local commands passed: ./verify.sh 21/0, 46/0, 9/0 (overlapping groups); native cargo test --workspace, 100,000 mixed-case differential cases/zero mismatches; aarch64-linux ./nix-build.sh check, proof-log/test-log/deny-log; image-static; LAN_HOST=203.0.113.28 ./scripts/acceptance.py and delayed OpenSSL request 200.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage: 365/1032 first-party security SLoC=35.37%, explicit three-run union with unchanged config.
Evidence: logs/stage10-*.
Protected containers unchanged; chromium-headers:8095 healthy and left running.
Independent Dudley acceptance, physical browsers, second LAN device, x86_64 and upstream F* untested.
Local commit follows using configured identity; no push.

## Stage 11 — pragma-header

- [x] Nineteen-name specification and all proof groups; ./verify.sh.
- [x] Brave cache-disabled fixture at whole/1/7/1024 TLS writes, near-misses with zero handlers and all inherited tests; cargo test --workspace.
- [x] 100,000 differential cases over nineteen names, zero mismatches.
- [x] Linux proofs/tests/dependency audit; ./nix-build.sh check.
- [x] Static image; ./nix-build.sh image-static.
- [x] Hardened pragma-header:8095, inherited OpenSSL/curl/idle checks and cache-disabled replay; scripts/acceptance.py.
- [x] Formal-coverage unchanged config and explicit three-run union.
- [x] Protected containers unchanged; final source/format/privacy checks.

Stage 11 local commands passed: ./verify.sh 21/0, 46/0, 9/0 (overlapping groups); native cargo test --workspace with 100,000 differential cases/zero mismatches; aarch64-linux ./nix-build.sh check and proof-log/test-log/deny-log; image-static; LAN_HOST=203.0.113.28 ./scripts/acceptance.py and delayed 3s OpenSSL request 200.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage: 366/1035 first-party security SLoC=35.36%, explicit three-run union with unchanged config.
Evidence: logs/stage11-*.
Protected containers unchanged; pragma-header:8095 healthy and left running.
Independent Dudley acceptance, physical browsers, second LAN device, x86_64 and upstream F* untested.
Local commit follows using configured identity; no push.


## Stage 12 — header-policy (resumed after afd8d42)

- [x] Config parser, policy decision, request isolation and strict-equivalence proofs; ./verify.sh (56/0, 82/0, 9/0).
- [x] Default/strict TLS fixtures, sensitive-name zero-dispatch and boundary tests; cargo test --workspace.
- [x] 10,000 generated configs and malformed-config startup refusal table; cargo test --workspace.
- [x] 1,000 real TLS requests /100 names:59 lines <=60; scripts/acceptance.py. Sliding-window unit checks limit 0/1/60/600, <=64 stored names; cargo test --workspace.
- [x] 100,000 policy-aware differential inputs, three policies each, zero mismatches; cargo test --workspace.
- [x] Linux proofs/tests/dependency audit; ./nix-build.sh check, proof-log/test-log/deny-log (aarch64-linux).
- [x] Static image; ./nix-build.sh image-static.
- [x] Hardened header-policy:8095; scripts/acceptance.py (builtin/strict/cookie profiles and default full run), delayed OpenSSL 200.
- [x] Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage unchanged config and explicit three-receipt union; scripts/measure-coverage.py (715/1685=42.43%).
- [x] Protected containers unchanged; before/after inspect comparison. Source/format/privacy checks, cargo fmt --check, Python syntax and git diff --check.

Original strict-equivalence contradiction resolved by spec commit afd8d42. All listed local commands passed; independent acceptance remains Dudley's.


Stage 12 final local evidence: ./verify.sh **56/0 parser, 82/0 transport root, 9/0 standalone AEAD** (overlapping groups); native workspace tests and aarch64-linux Nix check; static image; live builtin/strict/Cookie-drop profiles; explicit default configuration mounted read-only at /config.
All browser fixtures including DNT passed whole/1/7/1024 writes; strict DNT 400 and duplicate reasons verified.
100,000 differential inputs across three policies, zero mismatches; 10,000 config fuzz cases; 20 malformed startup cases plus missing-file refusal.
Live TLS policy, curl/LAN-address, partial stall 400, idle 17s/zero bytes, delayed 3 s / 200, worker-cap refusal/recovery, static/shell-free/key-free/hardened checks passed.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Final coverage **715/1685 first-party security SLoC=42.43%**, explicit three-receipt union using unchanged sibling config. header-policy remains healthy/running on 8095.
Protected containers unchanged.
No push.
Evidence: logs/stage12-*.
Independent Dudley acceptance, physical browsers, second LAN device, x86_64 and upstream F* remain outstanding; this is not an end-to-end TLS proof.

## Stage 13 — spec-lemmas

- [x] Intent witnesses, attack corpus and universal statements; ./verify.sh.
- [x] Config and traversal intent checks; workspace tests.
- [x] Fifteen consistent spec/executable mutants; scripts/spec-mutants.py.
- [x] Existing differential, config fuzz and TLS acceptance; workspace tests.
- [x] Formal-coverage remeasurement and scope/privacy audit.
- [x] INTENT.md, MUTANTS.md and final local commit.

Stage 13 local acceptance: ./verify.sh 211/0 parser, 237/0 transport root, 9/0 AEAD (overlapping groups); 42 intent lemmas plus 112 supporting lemmas.
Native workspace tests and aarch64-linux ./nix-build.sh check pass, including 100,000 differential inputs/zero mismatches, 10,000 config fuzz cases and loopback TLS acceptance. python3 scripts/spec-mutants.py exits 0: 15/15 killed, 0 survivors, 0 malformed; 13 intent-proof kills and 2 default-policy fixture/config kills.
All fifteen also fail actual tests while preserving implementation/spec proofs.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage 715/6095=11.73% with unchanged sibling tool/config; 4410 ghost SLoC are not attributed by v1 (unchanged remainder 715/1685=42.43%, derived explanation only).
No runtime changes, deployment, protected-container reuse, branch switch or push.
Independent acceptance remains Dudley's.
Evidence: logs/stage13-*.

## Release pass

- M1: complete — image build/healthcheck pass; broken-proof and denied-license image builds both fail in the expected check derivation.
- M2: complete — v1.4 first-party baseline stated on both branches; main per-stage measurements pending.
- M3: complete — public docs/scripts/fixtures updated; 31 tests pass after fixture substitution; public scan passes. Final mutants/image checks remain below.
- M4: complete — MIT on both branches and all package manifests; Nix check passes with workspace license checks enabled.
- Final check: complete — ./verify.sh (fresh full-mutant baseline) reports 211/237/9 verified, zero errors; cargo test --locked --workspace passes 31/31; full mutant run kills 15/15 with zero survivors/invalid runs; Nix check, static image build, full container acceptance and public scans pass. All release containers/images, the temporary TLS directory, negative clones and the main worktree were removed. These are local results; independent acceptance remains outstanding.

## Release fix

- F1: complete — corrected the probed libfaketime path, asserted the frozen UTC clock, and disabled fake file timestamps. Clean committed-tree flake check and static image build pass with no preload errors. The 13-day database passes; the November 11 clock fails with repository staleness; broken preload fails the clock assertion. All three experiments repeat successfully after F2's fetch change.
- F2: complete — repaired historical coverage labels, restored the plan's own examples and public-scan command, fixed prose punctuation, pinned the advisory source by a real fetched NAR hash, renamed the image and aligned the acceptance helper, and fixed certificate help and uid 65532 mount instructions. Fenced Markdown code is unchanged. Main needs no changes.
- Local verification: ./verify.sh reports 211/237/9 verified, zero errors; cargo test --locked --workspace passes 31/31; the full mutant rerun kills 15/15 with zero survivors/invalid runs; Nix check, static image, complete container acceptance and public scans on both branches pass. Proof/spec/test/mutant sources remain unchanged except the acceptance helper's exact image-name literals. No independent, x86_64, LAN or physical-browser rerun is claimed.
- Final post-commit clean-tree replay and resource cleanup are logged under <tmp>/release-fix-logs/final-*.log. No push, history rewrite or other-stage branch edits.
