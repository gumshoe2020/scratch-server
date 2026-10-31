# Verified boundary notes

## 2026-10-27 — start
Read all of VERIFIED-BOUNDARY.md. Branch verified-boundary, initially clean, one commit ahead of origin. Existing scratch-server:8088 and nix-scratch:8089 observed; do not touch either. No pushes. Each milestone gets a local commit, including honest partial/blocker status where applicable.

Grammar decision: define the exact strict HTTP subset explicitly alongside M1. Permit GET/HEAD, HTTP/1.0 and HTTP/1.1, absolute paths using ASCII letters/digits plus / - _ . (no query/percent encoding), a small case-sensitive header whitelist, printable ASCII values, CRLF framing, no body or pipelining, fixed request/path/header limits. Rejection and exact field semantics must be proved against that written executable specification. TLS records necessarily reach rustls before the HTTP parser; cryptographic framing/TLS state machine remain trusted and will be stated explicitly.

## 2026-10-27 14:11 ET — M1 record
Implemented one Verus/Cargo source, exact request/field equivalence and explicit returned bounds, with total loops and <=16 recursive header levels. `./verify.sh` passed: 18 verified, 0 errors. Cargo checks the erased same source. No assume/admit/external_body added. Verus archive SHA256 matches GitHub API; toolchain and Cargo cache isolated under .tools. Specification permits duplicate whitelisted headers and no Host, intentionally; framing/body headers are never admitted. Out-of-memory and the verifier/compiler/library models remain trusted (PROOFS.md). Learned: byte-string contents are abstract to Verus, so the path's initial slash uses a direct byte comparison shared by spec and implementation; no weakening of the initial-slash property.

## 2026-10-27 14:11 ET — M2 record
Removed axum/hyper/tokio; bounded std TCP reader feeds every accumulated read to the verified parser before checking framing. Responses/HEAD/404 are ours. 48 malformed cases (47 hex fixtures plus empty EOF), three read fragment sizes, all 400 with zero handler invocations. Live port-8090 curl and --healthcheck passed; proofs rerun passed. Connections have two-second IO deadlines, at most 64 concurrent workers, <=4097 bytes of buffering.
Transport limitation: TCP cannot reject bytes that have not arrived yet. A complete valid request is dispatched immediately; coalesced trailing bytes are rejected, later bytes are discarded by closing. No keepalive or second request is processed. TLS records will necessarily be consumed by rustls first. These framing/resource behaviors remain trusted, not formally proven.
Initial live probe used urllib, whose automatic Accept-Encoding header is intentionally outside this grammar, so it received 400. Repeated successfully with curl's allowed Host/User-Agent/Accept headers. Browsers that send extra headers outside the whitelist also intentionally get 400; do not silently broaden the grammar.

## 2026-10-27 14:12 ET — M3
100,000 proptest cases passed: 50,000 valid requests compared field-for-field against httparse, and 50,000 mutated/truncated/arbitrary byte inputs classified by an independent std-only grammar recognizer. No production parser helpers/constants are used by the recognizer. Outside-subset inputs must reject regardless of httparse's broader acceptance. Explicit 256/257 path and value, 16/17 header and 4096/4097 total boundaries supplement generation. No counterexamples found; future proptest failure seeds are not gitignored. Official proptest configuration tutorial read: https://proptest-rs.github.io/proptest/proptest/tutorial/config.html . No bound on the formal proof is implied by these finite empirical tests.

## 2026-10-27 14:15 ET — M4 blocked (source coverage)
Read AWS-LC README, aws-lc-verification master README including API/CPU caveats, s2n-bignum README, and rustls 0.23.35 provider suite list. Detailed links/evidence in TLS-COVERAGE.md. No listed fully covered ARM record cipher: AES256-GCM proof is x86-specific, excludes AAD and partial blocks; no AES128-GCM or ChaCha20-Poly1305 entry. X25519's s2n-bignum proof does not close this gap. Cannot honestly satisfy “restrict ... to ones whose AWS-LC implementations are covered” by selecting algorithm names alone. Treating the spec's “unconfirmed” instruction as disclosure, not permission to declare unconfirmed coverage accepted. M4 is blocked on this stronger requirement; HTTP retained. Static musl AWS-LC build not attempted (no buildability claim). Runtime key loading/handshakes not implemented or tested. Added future-use certificate script that refuses repo destinations; syntax and refusal checked, no material generated. Continue M5, which supports HTTP when M4 does not land.

## 2026-10-27 14:25 ET — M5 and final handoff
Nix check and image-static build/load passed. Pin probe confirms nixpkgs rustc 1.99.0, cargo-deny 0.20.2, cargo-fuzz 0.13.2, no Verus/Kani attributes. Packaged release Verus with separate official Rust 1.99.1 and SHA256-checked archives; Verus launcher source confirms VERUS_USE_RUSTUP=0. Proof check in Nix passed 18 verified, 0 errors. First package attempt copied unnecessary release maintenance tools (OpenSSL missing); fixed by packaging only verifier, solver, macro and vstd artifacts, without ignoring missing libraries. Nix runs the full tests (100,000 differential cases), plus cargo-deny over the pinned RustSec 99670cfc0bc574433034b75876a4b791ccaa2efb snapshot. First deny run caught an unspecified local path version and unlicensed private workspace crates: version now explicit, only the already-publish=false workspace crates excluded from license policy (no license grant invented), third-party licenses still checked. Offline index/yank warnings remain; all four check categories passed. Timestamped advisory freshness is checked when the derivation executes; cached Nix results describe a snapshot, not ongoing live advisory status.

Hardened verified-boundary left running, healthy, port 8090. Image sha256:f0532d45fae17110b6964b71b7e4dba0d553db8df22c12c0179d6453b19055aa; 1,343,421 bytes; binary 648,992 bytes. Saved image layers contain exactly one binary; ELF inspection finds no interpreter/shared-library dependency. Export has Docker-injected .dockerenv, /etc network metadata, /dev/console and mount directories; these are not shipped image payload. Automated acceptance script passed routes, HEAD covered by Rust tests, healthcheck, shell absence, hardened settings, all 48 malformed cases. LAN_HOST=203.0.113.28 passed same-host LAN-address access; another LAN client was not tested. Final native ./verify.sh passed 18/0. Certificate script syntax/path refusal passed; no secrets generated. README gives reproducible commands; logs/evidence retained under logs/ and excluded from Git.

Read-only observation: scratch-server was found exited 137, finished 2026-10-27T18:14:56.519607095Z (14:14:56 ET), OOMKilled=false. Cause unconfirmed. This run issued no stop/start/restart/removal/reuse operation for scratch-server or nix-scratch; did not change either container. nix-scratch remained healthy. Do not restore/modify the unrelated container as part of this task.

Passed: M1 unbounded parser proof; M2 server/corpus/counter/routes; M3 100,000 cases; M5 pinned Nix checks and static image/container/LAN-address checks. Did not pass/implement: M4 TLS, source-coverage blocker in TLS-COVERAGE.md; no musl AWS-LC build, handshake, runtime key loading or actual certificate generation tested; x86 build and second-device LAN access untested. Trusted rather than proven: grammar/spec adequacy, Verus/Z3/vstd and macro erasure, compiler/toolchain, allocation/resources, socket acquisition/framing/dispatch/responses, kernel/Docker/hardware; all exact boundaries in PROOFS.md. Dudley must independently run acceptance and decide whether to revise M4 coverage requirements. No pushes, no branch changes, no EVOLUTION.md edits. Proposed learning: published crypto proofs must be matched to API, CPU, compiler and input conditions, not just algorithm names.

## 2026-10-27 14:29 ET — Stage 4 start
Read BROWSER-WHITELIST.md fully, then stage 3 proof/progress/notes. Confirmed browser-whitelist branch; initial untracked codex-run-stage 4.log belongs to the caller and will not be committed. Protected containers observed: scratch-server exited 137, nix-scratch and verified-boundary healthy. No operations on them authorized or planned. Extend only the five named headers and retain all other grammar rules. Unknown-name diagnostics inspect only bytes before a colon in CRLF header lines, filter to ASCII alphanumeric/hyphen, then cap at 64 bytes; malformed lines without a colon receive status only. Logging remains trusted and cannot affect acceptance/dispatch. No external memory writes because this task explicitly restricts changes to this directory.

## 2026-10-27 14:31 ET — Stage 4 implementation
Nine exact case-sensitive names implemented in header_prefix/prefix and returned-name bound extended from 4 to 9. No other grammar/limit changes; no new proof exemptions. Native ./verify.sh passed 18 verified/0 errors. Captured request fixtures are copied from the spec with only Host changed to port 8091 (captured Referer remains 8099) and the second User-Agent expanded from “same.” Rust tests pass both captures whole and at 1/7/1024 reads, original 48 malformed inputs plus 18 new rejections with zero handler calls, and 256-byte accepted boundaries for each added name. Unit diagnostics checks cover Cookie: x, non-ASCII/control filtering, 64-byte cap, malformed values with bare LF, and status-only failures. Keep-alive stays ignored. Differential generation/oracle both use all nine names. README and acceptance script now target only browser-whitelist:8091; no changes to protected containers.

## 2026-10-27 14:33 ET — Stage 4 checks
Native workspace tests passed, including all 100,000 differential cases (23.87s), no mismatches. Nix check passed on aarch64-linux: proof log confirms 18/0, tests log confirms all 100,000 cases (3.91s); dependency check passes with inherited pinned advisory/yank limitations. x86_64-linux was omitted and is not tested. Exact-byte HTTP fixtures marked binary in .gitattributes to preserve CRLF/framing and avoid treating the mandatory final blank line as accidental whitespace. Static image build underway; no deployment acceptance claimed yet.

## 2026-10-27 14:34 ET — Stage 4 final summary
Passed: native ./verify.sh 18/0; workspace tests including original 48 malformed plus 18 new rejection cases with zero handler calls; both Safari captures whole/1/7/1024; 100,000 differential cases, 0 mismatches; ./nix-build.sh check (aarch64-linux proofs/tests/cargo-deny); ./nix-build.sh image-static; ./scripts/acceptance.py including curl 200 and Docker Cookie-name-only logging. Final fixture replay preserves Referer exactly as captured (8099), changing only Host to 8091. After that fixture correction, boundary tests, Nix check, static image build/load and runtime acceptance were rerun successfully. Replaced only this stage's own browser-whitelist container with the final built image; left running and healthy on 8091 with required hardening. Image sha256:d025f684e2ea310dcf51f0f91fef2897aba8dc5d4959feb769ff9a8b816683af; 1,353,661 bytes; binary 651,224 bytes, static and shell-free. Protected scratch-server/nix-scratch/verified-boundary identity/configuration/mounts/PID/lifecycle unchanged against before/after Docker inspect. Caller run log retained and ignored alongside existing run-log pattern; not committed.

Did not pass/test: TLS remains blocked/out of scope per TLS-COVERAGE.md; x86_64-linux, physical Safari device and second-device LAN access were not tested. No stage 4 acceptance command failed in the final tree. Dudley still needs to run independent acceptance. Trusted rather than proven: grammar adequacy and translation, Verus/Z3/vstd/macro erasure/compiler, allocation/resources, transport/framing/dispatch/responses and rejection diagnostics, OS/Docker/hardware. Only parser safety/grammar/fields/limits are formally proved. Full evidence stays in logs/stage4-*.log plus logs/acceptance.json and exported-image evidence. No pushes, branch changes, EVOLUTION.md edits or writes outside the repository.

## 2026-10-27 14:39 ET — Stage 5 start
Read HTTPS.md fully then TLS-COVERAGE.md, PROOFS.md, BROWSER-WHITELIST.md, PROGRESS.md and NOTES.md. Branch https; initial untracked codex-run-stage 5.log belongs to caller. Protected Docker inspect snapshot saved to logs/stage5-protected-before.json. No changes to protected containers. Stage 5 permits honestly labeled unverified crypto components; it does not require a wholly verified TLS stack. Keep HTTP grammar and nine names unchanged. No external memory write: explicit task write boundary restricts writes to repo and certificate directory.

## 2026-10-27 14:43 ET — Stage 5 provider and first tests
Pinned rustls 0.23.35 without default crypto or TLS 1.2; libcrux AEAD 0.0.10 and curve25519/SHA2/HMAC/HKDF/ECDSA/P2560.0.9. Each published package README and selected API implementation was read from its crates.io archive; upstream HACL manual/repository and translation paper abstract were read. P256 and Ed25519 both publish HACL-derived verification claims, so choose P256 for compatibility. Exact target/compiler/wrapper proof correspondence stays unconfirmed; no upstream F* proof rerun here. No ring/AWS-LC active in cargo tree; optional ring is present in Cargo.lock from webpki metadata only. Initial dependency resolution failure was our wrong direct libcrux-secrets 0.0.5 pin; matching upstream 0.0.7 resolved it.
Provider compiles; TLS root/Safari and 66 malformed cases pass (zero dispatch), initially one HKDF test failed because expected hex was transcribed with a missing 2; corrected against RFC5869 vector. ./verify.sh passed 18/0 with parser untouched. Generated P256 self-signed cert only in authorized sibling directory (700, key 600). Runtime loads PKCS8; malformed/unsupported/mismatched keys fail startup. Resumption/tickets disabled, avoiding any additional ticket cipher.

## 2026-10-27 14:46 ET — Stage 5 dependency audit
Native workspace tests including TLS/Safari/66 malformed zero-dispatch cases and 100,000 differential cases passed after correcting test-client TLS close sequencing (queue final application record and close_notify before flush, avoiding post-close TCP reset). First Nix check failed because new src/tls.rs was untracked and omitted by flakes; staged it, then check reached cargo-deny. Deny caught rustls 0.23.35 advisories: upgraded to latest stable 0.23.46 from crates.io API, rather than ignore vulnerabilities; removed unmaintained rustls-pemfile and used pki-types PemObject. Primary RustSec pages 2026-0285, 2025-0134, 2026-0173 actually read. Added BSD-3-Clause/ISC permissive licenses needed by new dependencies. Narrow exception for informational unmaintained 2026-0173 only: proc-macro-error 2 is under cfg(hax) in transitive hax-lib-macros 0.3.7 via core-models; no safe upgrade, not built by production/Verus. No vulnerability exceptions. This is a dependency-maintenance trust limitation, not a cryptographic proof exemption.

## 2026-10-27 14:47 EDT — Stage 5 native verification
Updated rustls 0.23.46 native workspace tests passed:5 library tests including all TLS scenarios, 6 boundary tests, 3 differential tests (50,000+50,000 cases, 0 mismatches). Provider API source files at 0.23.46 are byte-identical to the 0.23.35 provider files already read; recorded comparison in logs/stage5-rustls-source-review.log. Target-musl cargo tree has no active ring or proc-macro-error 2 dependency. Parser source remains unchanged; proof-exemption search has zero matches. Existing malformed stream tests retain their original TCP semantics: coalesced excess bytes rejected, bytes arriving after dispatch discarded by close. Linux checks and final musl image are building.

## 2026-10-27 14:48 EDT — Stage 5 Linux build and deploy
./nix-build.sh check passed (aarch64-linux); retrieved proof/test/deny logs: 18/0,100,000 differential cases plusTLS/corpus tests, dependency checks passed with only documented informational exception/offline-yank limitation. ./nix-build.sh image-static passed and loaded https:static. Started only new container https on 8092, read-only/cap-drop ALL/no-new-privileges, image user 65532, external certificate directory mounted read-only. Docker Desktop permits mounted-key reads without weakening host 700/600 permissions. OpenSSL external LAN-address probe confirmsTLS 1.3/ChaCha20Poly1305/X25519/ECDSA P256; self-signed warning expected. Runtime acceptance still in progress.

## 2026-10-27 14:50 EDT — Stage 5 final summary
Passed: native `./verify.sh`, including final rerun (18 verified, 0 errors); workspace tests (same-provider authenticated TLS handshake, root and both Safari fixtures at 1/7/1024-byte writes, 66 malformed TLS requests with zero handler calls, original boundary tests, 100,000 differential cases with zero mismatches). `./nix-build.sh check` passed on aarch64-linux: proofs, tests and cargo-deny. `./nix-build.sh image-static` passed.

`LAN_HOST=203.0.113.28 ./scripts/acceptance.py` passed: OpenSSL TLS 1.3 / ChaCha20-Poly1305 / X25519 / ECDSA P-256; TLS 1.2, AES-only suite, P-256-only key exchange and h2-only ALPN rejected; curl -k returns 200; plaintext HTTP refused. Runtime healthcheck, both Safari replays, 48 malformed requests returning 400, and Cookie-name-only stdout passed. Export/image layers contain only the static binary (Docker injected metadata excepted), no certificate/key. Image size 3,096,498 bytes; binary 1,523,248 bytes. Image sha256:4201e75b7a801afac4306f7f85ea432c302e2bbce7c6ff5cb9ba2f5ec7555c7b. Certificate IP SAN check passed; host directory mode 700, key 600, runtime mount read-only. Container https remains running and healthy on 8092. Protected-container identity/configuration/mounts/PID/lifecycle comparison passed. No branch switch, push, EVOLUTION.md edit, protected-container mutation, or secret committed/printed/logged.

Not tested: physical Safari, a second LAN device, x86_64-linux, upstream F* proof reproduction. Dudley must independently run acceptance. Initial test/build/dependency failures were resolved; no final acceptance failure remains. Trusted rather than proven: rustls handshake/record layer, X.509/PEM/PKCS8/SEC1/signature encoding, OS randomness, provider glue, upstream wrappers/translation/compiler/target correspondence, allocation/secret-memory cleanup, toolchain, OS/Docker/hardware. Crypto claims come from the read upstream HACL*/F* sources; exact release/target proof correspondence is unconfirmed. One informational unmaintained cfg(hax)-only macro advisory exception is recorded in deny.toml; no vulnerability ignored. Only the parser is locally proven; its HTTP grammar and nine-name whitelist are unchanged.

## 2026-10-27T15:04:11-04:00 — Stage 6 start
Read stage spec and required historical documents. Confirmed https-headers branch; caller codex-run-stage 6.log retained, ignored and excluded from commit. Saved protected-container inspect snapshot. Exactly three new names; no other grammar or TLS policy change. Preserve stage 4 fixtures and add separate HTTPS fixtures: captured name order, entirely synthetic values (including Host/Referer on 8093). No external memory diary write because explicit task restricts writes to this directory. Existing sibling cert/key will only be read at runtime.

## 2026-10-27T15:04:55-04:00 — Stage 6 proof and test changes
Native ./verify.sh passed 18/0 against twelve names; exemption search found no matches. Only parser changes: three prefix alternatives in spec/executable and returned name bound 9→12. All limits/grammar unchanged. Inherited malformed TLS case Sec-Fetch-Site is now valid; replace it with specified near misses. TLS tests now include both new fixtures at whole/1/7/1024 writes, plus all three wrong-case names and empty/257-byte values, near misses and inherited corpus with zero dispatch assertions. Existing stage 4 fixtures retained. cargo fmt check initially reported expected formatting changes in new test entries; cargo fmt applied.

## 2026-10-27T15:05:47-04:00 — Stage 6 native/Linux checks
Native workspace tests passed (5 library, 6 boundary, 3 differential); 100,000 cases, 0 mismatches. TLS tests: original 48 malformed plus 28 new/inherited-name cases, all 400 and zero handlers; new near misses include Sec-Fetch-Destination and Sec-Fetch-. Native boundary test covers 29 whitelist rejection cases (plus original 48), and 256-byte valid values for all eight stage 4/6 additions. ./nix-build.sh check passed on aarch64-linux; proof-log 18/0, test-log 100,000 cases, deny-log all four categories passed. Inherited informational cfg(hax) advisory exception and offline registry-yank limitation remain; x86_64 omitted, not tested. Static build underway.

## 2026-10-27T15:06:58-04:00 — Stage 6 final summary
Passed: ./verify.sh 18/0; native cargo test --workspace including HTTPS fixtures whole/1/7/1024 TLS writes (root 200, favicon 404), 76 malformed TLS cases with zero handler calls, original 48 and 29 additional boundary rejections, 100,000 twelve-name differential cases with 0 mismatches. ./nix-build.sh check passed aarch64-linux proofs/tests/cargo-deny; retrieved proof/test/deny logs. ./nix-build.sh image-static passed and loaded https-headers:static. cargo fmt --check, git diff checks, fixture captured-name-order/CRLF check, certificate IP SAN check passed.

LAN_HOST=203.0.113.28 ./scripts/acceptance.py passed: TLS 1.3/ChaCha20Poly1305/X25519/ECDSA P256; TLS 1.2, AES-only, P256-only key exchange, h2-only and plain HTTP refused. Curl HTTPS and synthetic-value HTTPS fixture replay 200; favicon 404; 53 live malformed requests 400; Cookie-name-only diagnostic privacy. Static ELF no interpreter/DT_NEEDED, shell-free image, saved layers contain only binary and no keys/certificates; exported runtime has only binary plus Docker-injected metadata. Image sha256:fe1e9a4dadd02972947de3304f8fd067daceab66806a86ffb8703bc4f96c7b13; 3096506 bytes, binary 1523392 bytes. Hardened https-headers remains healthy/running on 8093 (read-only/cap-drop ALL/no-new-privileges/uid 65532, read-only original certificate mount). Final protected-container comparison passed: identity/config/mounts/PID/lifecycle unchanged for scratch-server, nix-scratch, verified-boundary, browser-whitelist, https. No changes to sibling certificate/key or TLS provider policy. No branch switch, push, EVOLUTION.md edit or writes outside repo.

No final acceptance failure remains. Not tested: physical Safari, second LAN device, x86_64-linux, upstream F* proof reproduction; independent Dudley acceptance remains outstanding. Trusted rather than proven: grammar adequacy/translation, Verus/Z3/vstd/ghost erasure/compiler, allocation/resources, transport/framing/dispatch/responses and diagnostic code, rustls record/handshake, X.509/PEM/PKCS8/SEC1/DER, OS randomness, provider glue, upstream wrappers and release/target/translation correspondence, secret cleanup, OS/Docker/hardware. Only parser is locally proven. TLS-PROVIDER.md limitations and inherited informational cfg(hax) advisory exception/offline-yank limitation remain unchanged. Evidence: logs/stage6-*. Caller run log retained/ignored. Local stage commit follows; no push.

## 2026-10-27T15:11:57.847620-04:00 — Stage 7 start
Read authoritative spec and required background. Confirmed idle-connections branch; protected six-container snapshot saved. No memory diary write: explicit write boundary is this directory only. Caller run log retained/ignored. Interpret short deadline literally as 2 seconds between reads (not a total request deadline); the pre-request 15-second budget is absolute from accept, including TLS handshake. At cap refuse new accepted sockets immediately without worker queue; idle clients can hold all 64 slots for up to 15 seconds. EOF without bytes retains inherited malformed-corpus 400 behavior; only idle timeout closes silently.

## 2026-10-27T15:14:17.409183-04:00 — Stage 7 native checks
./verify.sh passed 18 verified/0 errors; parser/spec and TLS provider implementation unchanged. Native cargo test --workspace passed 7 library, 6 boundary, 3 differential tests (100,000 cases, zero mismatches). TLS fixtures/corpus now run through production deadline adapter. New real-time tests cover TCP/TLS delayed 3s, idle 16s zero application bytes, partial 3 s / 400/zero handlers; TLS handshake delayed 3s still closes at total 15s; no-handshake and incomplete TLS record trickled over 12s both close near 15s with zero wire bytes. Diagnostic privacy tests retain sanitization and 64-byte cap with explicit grammar/unknown_header classes. An overbroad edit initially removed healthcheck socket timeouts; caught in diff review and restored before image build. Final native cargo check follows; no change to healthcheck behavior.

## 2026-10-27T15:16:48.346773-04:00 — Stage 7 final summary
Passed: native ./verify.sh 18/0 and exemption scan (zero matches); cargo test --workspace (7 library, 6 boundary, 3 differential; 100,000 cases, zero mismatches); final cargo check --workspace and cargo fmt --all -- --check. Parser/spec/manifests unchanged; src/tls.rs implementation before test module is byte-identical. New real socket tests exercise production adapter over TCP and TLS: delayed 3 s / 200, idle past 15s zero application response, partial stall 400/zero dispatch, handshake delay consuming the same 15s budget, absent/trickled handshake zero wire response near 15s. The 2s limit remains between socket reads, not a total request-time guarantee; sufficiently frequent trickles can persist until the 4096-byte cap, as in the inherited policy. Application buffering stays 4097 bytes.

Final ./nix-build.sh check passed aarch64-linux proofs/tests/cargo-deny; proof-log 18/0, test-log 100,000 cases plus deadline/corpus/fixture tests, deny-log all four categories passed. One early deny-log retrieval ran before its derivation finished and had no log; final retrieval succeeded. Inherited informational cfg(hax) advisory exception and offline registry-yank limitation remain. ./nix-build.sh image-static passed and loaded idle-connections:static. Image sha256:7bd28e498e421e03fa0fc28891cb50facf1fb09efe2b04a13185650c54173a8c, size 3096509 bytes, binary 1523120 bytes. LAN_HOST=203.0.113.28 ./scripts/acceptance.py passed static ELF without interpreter/DT_NEEDED, shell-free/key-free image layers/export, read-only/cap-drop ALL/no-new-privileges/uid 65532, original certificate mounted read-only, routes/healthcheck/Safari fixture/curl replay, 52 live malformed rejections, classified grammar/size/slow/unknown_header/idle logging with value privacy, 64 occupied slots refuse excess and recover. Live corpus omits empty-open request because it now correctly idles silently; empty EOF still returns 400 with zero dispatch in both Rust corpus paths.

External policy checks passed TLS 1.3/ChaCha20Poly1305/X25519/ECDSA P256; TLS 1.2, AES-only, P256-only key exchange, h2-only and plain HTTP refused. Requested exact delayed OpenSSL pipeline on 203.0.113.28:8094 returned 200. idle-connections remains healthy/running on 8094. Final Python before/after Docker comparison passed all six protected IDs/configurations/mounts/PIDs/lifecycles unchanged. No branch switch, push, EVOLUTION.md edit, certificate/key generation/change, secret output or writes outside this directory. Caller run log retained/ignored. Local stage commit follows.

No final acceptance failure remains. Not tested: physical Safari, second LAN device, x86_64-linux, upstream F* proof reproduction; Dudley's independent acceptance remains outstanding. Trusted rather than proven: transport deadline/framing/resource enforcement, diagnostics, dispatch/responses, rustls handshake/record layer, certificate encoding/validation, OS randomness, provider glue/upstream wrapper and release/target correspondence, secret cleanup, grammar/spec adequacy, Verus/Z3/vstd/ghost erasure/compiler, allocation/resources, OS/Docker/hardware. Only the unchanged HTTP parser has a local formal proof; TLS-PROVIDER.md limitations remain unchanged.

## 2026-10-27T16:37:23.233774-04:00 — Stage 8 start
Read authoritative spec and required history. Confirmed proven-transport branch; caller log excluded. Protected four-container snapshot retained. No memory write outside repository. Empty Bytes events terminate as Grammar (EOF convention); interrupted syscalls are shell retries, not core events. Terminal machines return Done on further events, avoiding repeat dispatch or false silent-close claims. Stage 7 empty EOF remains Grammar.

## 2026-10-27T16:40:47.305932-04:00 — Stage 8 core checkpoint
./verify.sh passed 18/0 parser and 43/0 core (includes parser and AEAD module); cargo check --workspace passed.
Core contracts cover arbitrary input/event contents, bounded 4097-byte buffering, whole-buffer parser acceptance, one dispatch, terminal absorption, deadlines/EOF/size, decreasing rank and arbitrary-trace 4098-event bound.
Empty Bytes becomes Grammar.
Response contracts fix status/header bytes, exact decimal Content-Length and HEAD omission; decimal_value proves the decimal length, all variable digits 48..57.
Response bodies limited to 4096 bytes, enforced at shell boundary next.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Baseline formal-coverage reproduced 737 first-party security SLoC, 17.37% proven using original config and source-bound receipt.
No crypto proof claim yet: helpers verified, not wired.

## 2026-10-27T16:42:15.838189-04:00 — Stage 8 shell wired
Production serve_with_started now only maps reads/errors to core events and executes actions; both plaintext and TLS use it. Responses use the verified formatter; shell rejects bodies over 4096 before calling it. Header diagnostics and dynamic root body generation remain trusted. New event-script shell tests passed (cargo test --test transport): fragmentation/interruption, successful dispatch then no more reads, idle/partial deadlines, EOF, trailing bytes, 4097 one-byte oversize events; direct core tests check terminal absorption and exact classes; all response lengths 0..4096 and all statuses test HEAD/body isolation. ./verify.sh passed 18/0 and 43/0 after wiring. Nix proof source includes all crates; image/runtime script targets only proven-transport:8095. Full inherited workspace tests are running.

## 2026-10-27T16:44:26.225458-04:00 — Stage 8 TLS glue checkpoint
./verify.sh passed 18/0 parser, 43/0 transport root (including parser/helpers), 9/0 standalone AEAD; counts overlap. cargo test --workspace passed all inherited fixtures/corpus/deadline tests and 100,000 differential cases, plus 4 new event/formatter/AEAD tests. Production record adapters now use proven nonce, overflow-checked sealing size, ciphertext/tag size and conditional release gate. Test-only code unchanged in meaning. A broad literal replacement briefly affected a test allocation; compiler caught it and original test was restored. Nonce tests match unchanged rustls Nonce and big-endian construction across carry/high-bit boundaries. Error gate preserves ciphertext on failure. Crypto algorithms/policy unchanged.

Remaining provider trait methods, upstream calls, AAD/conversions and signature/HKDF/X25519 checks cannot be wholly proved without upstream models/contracts; no exemptions or invented contracts added.
Exact boundary listed in PROOFS.md.
Formal-coverage v1 accepts one root/log only, so scripts/measure-coverage.py runs three independent complete-file verifier receipts and the original tool/config, preserving raw reports and an explicit union of proven IDs from identical inventories.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Baseline 128/737=17.37%; after union 330/978=33.74%.
Individual raw runs attribute 128,149 and 53 proven SLoC respectively; tool does not itself accept their union.
Dependency sizes remain unknown (no upstream proof/size claim).
Nix/runtime checks follow.

## 2026-10-27T16:46:38.804798-04:00 — Stage 8 Linux checks
./nix-build.sh check passed aarch64-linux; proof-log 18/0, 43/0, 9/0, test-log 7 library+6 boundary+3 differential+4 transport tests, 100,000 cases/zero mismatches; deny-log advisories/bans/licenses/sources passed. Inherited informational cfg(hax) advisory exception and offline yank limitation unchanged; x86_64 omitted. ./nix-build.sh image-static passed/loaded proven-transport:static. Started only new proven-transport on 8095 with read-only root/cap-drop ALL/no-new-privileges/uid 65532 and original TLS bind read-only. Final runtime acceptance underway. Sibling formal-coverage cargo build passed with its isolated .cargo-home; reran baseline and final reports. Fixed reporting-script aliasing so its individual-report summary retains 128/149/53, while union remains 330/978. A source-comparison check initially assumed EVOLUTION.md existed on this branch; it is absent. Corrected check confirms prohibited files untouched, parser byte-identical, branch unchanged and no Rust proof-exemption tokens.

## 2026-10-27T16:49:29.893404-04:00 — Stage 8 formatter generalization
Final review removed the extra 4096 response-body precondition instead of retaining a narrower property than necessary. decimal now proves correctness/no-header-control digits for every usize length; response accepts arbitrary body slices and returns None only when the complete GET buffer cannot fit usize, while HEAD formats just headers. Shell maps only that error to IO failure. ./verify.sh passed 18/0, 43/0, 9/0. Added runtime response checks 4097/10, 000/100,000 bytes beyond exhaustive 0..4096. Earlier image/runtime acceptance passed but is superseded; final native/Nix/image/coverage/runtime reruns follow because executable source changed.

## 2026-10-27T16:52:45.495914-04:00 — Stage 8 final summary
Now locally proven (all inputs, no bounded checking): the unchanged twelve-name HTTP parser; connection-core buffer<=4097 and arithmetic/index safety; whole-buffer parser-gated Dispatch at most once; terminal absorption; timeout/idle/EOF/oversize classification; decreasing rank and an arbitrary-trace bound of 4098 events; exact three-status response format, decimal Content-Length, exclusion of CR/LF from variable header digits and HEAD header equality/body omission for arbitrary body lengths (only unrepresentable combined buffers return None). AEAD helpers prove 12-byte IV XOR96-bit big-endian sequence nonce construction, checked payload+1+16/tag subtraction and conditional authenticated plaintext release with unchanged destination on reported failure. Production shell uses those helpers. Final ./verify.sh passes 18/0 parser, 43/0 transport including parser/helpers, 9/0 standalone AEAD (overlapping counts, not 70 distinct properties); source scan finds no proof exemptions.

Final native cargo test --workspace and ./nix-build.sh check pass:7 library, 6 boundary, 3 differential (100,000 cases, zero mismatches), 4 new transport/format/AEAD tests. Includes inherited HTTP/HTTPS Safari fixtures, malformed 400 with zero handlers, delayed/idle/slow and absolute handshake-budget tests; new scripted shell sequences, terminal absorption, 4097 one-byte events, nonce equivalence and release failure preservation. Responses tested every length 0..4096 plus 4097/10, 000/100,000. aarch64-linux proof/test/deny logs retrieved; all four dependency categories pass with inherited informational cfg(hax) exception/offline yank limitation. Final ./nix-build.sh image-static passes. Replaced only this stage's own initial proven-transport container after its final rebuild; protected containers untouched.

Final LAN_HOST=203.0.113.28 ./scripts/acceptance.py passes on 8095: TLS 1.3/ChaCha20Poly1305/X25519/ECDSA P256; TLS 1.2, AES-only, P256-only group, h2-only and plaintext refused; curl/Safari replay, routes, healthcheck, 52 malformed requests 400, name-only diagnostics/value privacy; delayed 3 s / 200, partial stall 400, idle 17s closes with zero application bytes, 64-worker refusal/recovery. Separate delayed 3s OpenSSL pipeline returns 200 on final image. Static ELF has no interpreter/DT_NEEDED; image layers/export are shell-free and contain no key/certificate. Final image sha256:c8dc2af42083162f1c4ea598a09bdc2401dd765db7daf246ed253a32aaf318ae, size 3096509 bytes, binary 1521976 bytes. proven-transport remains healthy/running on 8095 with read-only root, cap-drop ALL, no-new-privileges, uid 65532 and original TLS directory mounted read-only. Final four-container identity/config/mount/PID/lifecycle comparison passes for scratch-server, nix-scratch, parser-http, parser-https. Parser source, TLS provider/suite policy and inherited TLS tests are byte-identical to stage 7; EVOLUTION.md/ATTACK-SURFACE.md untouched; branch never changed, no push. Caller log retained/ignored. No key printed/logged/committed, certificate/key generation or writes; sibling formal-coverage only built in place as authorized, no source edits or commits. No external memory write due task boundary.

Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Before/after formal-coverage: baseline 128 proven of 737 first-party security-relevant SLoC=17.37%; final 337 of 981=34.35% (+209 proven SLoC,+16.98 percentage points).
The sibling tool was built with CARGO_HOME="$PWD/.cargo-home" cargo build and its original examples/scratch-server config was used unchanged. v1 accepts one complete-file receipt only: three actual source-bound Verus runs yield raw reports with 128/156/53 proven SLoC on identical 981-SLoC inventories. scripts/measure-coverage.py explicitly unions those proven unit IDs (logs/stage8-coverage-union.json, summary.json); no combined verifier receipt or tool support is fabricated.
Unknown dependency sizes stay unknown and upstream primitive claims stay unconfirmed.
Earlier 330/978 figures above are superseded by arbitrary-length formatter generalization.

Still unverified: IO/action shell wiring and event delivery, real-clock deadlines/availability, threads/64-slot admission, route/body generation, handler counter, diagnostics and writes; provider trait code including payload gathering, key conversions, AAD, rustls payload slicing/truncation/unpadding, trait encrypted_payload_len's upstream length bound, X25519 checks, HKDF/hash/HMAC length assumptions, PKCS8/SEC1/X.509/DER/signatures; rustls handshake/record protocol, crypto correctness/failure reporting, OS randomness, upstream release/target/F* correspondence, secret cleanup, allocation/resources, grammar/spec adequacy, Verus/Z3/vstd/ghost erasure/compiler, OS/Docker/hardware. Full provider slice/length proof was not achieved without upstream models/contracts; exact limits appear in PROOFS.md rather than invented axioms or weaker claims. No end-to-end verified TLS or wall-clock liveness claimed. Physical Safari, a second LAN device, x86_64 and upstream F* reruns were not tested. No final local acceptance failure remains; independent acceptance by Dudley remains outstanding. Four local checkpoint commits (core, shell, glue, final), no push; logs/stage8-* retain evidence.

## 2026-10-27T18:22:15.750708-04:00 — Stage 9 start
Read authoritative spec and required records; confirmed firefox-headers branch. Only Sec-Fetch-User will be added. Protected-container snapshot saved; port 8095 free. Caller log excluded. No external memory diary write because task permits writes only in this repository. Synthetic Firefox fixture values will never be printed or logged.

## 2026-10-27T18:22:39.413875-04:00 — Stage 9 proofs
./verify.sh passed 18/0 parser, 43/0 transport root, 9/0 standalone AEAD (overlapping counts). Parser diff limited to the new spec/executable prefix and name bound 12→13. Existing transport core and production TLS implementation unchanged. Firefox root/favicon fixtures retain captured name order, all values synthetic; TLS fixture loop includes whole/1/7/1024 writes. New name rejection coverage includes wrong case, both near misses, empty/257-byte values and valid 256-byte boundary.

## 2026-10-27T18:23:28.427635-04:00 — Stage 9 formal-coverage
Sibling formal-coverage cargo build passed using its isolated .cargo-home; no source/config changes. python3 scripts/measure-coverage.py passed using the unchanged examples/scratch-server config and stage 8 three-run union method.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Current first-party security-relevant inventory 983 SLoC; 338 proven=34.38%.
Individual reports attribute 129/156/53 proven SLoC on identical 983-SLoC inventories, union by exact unit identity.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Stage 8 final 337/981=34.35%; original stage 7 baseline 128/737=17.37%.
Reports/receipts/union/summary retained under logs/stage9-coverage-*. v1 accepts one receipt, so union is explicitly derived rather than a fabricated multi-root tool result.
Dependency sizes remain unknown; trust limits unchanged.

## 2026-10-27T18:23:45.270371-04:00 — Stage 9 native acceptance
Native cargo test --workspace passed 7 library, 6 boundary, 3 differential, 4 transport tests; 50,000 valid plus 50,000 mutated/arbitrary cases over 13 names, zero mismatches. Includes Firefox root 200/favicon 404 at whole/1/7/1024 TLS writes, inherited Safari HTTP/HTTPS fixtures, 81 malformed TLS cases with zero handler calls, 34 boundary whitelist rejection cases at all four chunk sizes, and inherited transport/idle/slow/handshake-budget tests. Fixture name-order/CRLF comparison and no-exemption/source-scope checks passed. Production transport core and TLS implementation unchanged.

## 2026-10-27T18:24:30.639503-04:00 — Stage 9 Linux checks
./nix-build.sh check passed aarch64-linux. Retrieved proof-log/test-log/deny-log: 18/0, 43/0, 9/0; all workspace tests including 100,000 differential cases; advisories, bans, licenses, sources all passed. Existing informational cfg(hax) exception and offline registry-yank limitation unchanged. x86_64 omitted and untested. Static image build underway.

## 2026-10-27T18:25:16.706185-04:00 — Stage 9 image/runtime start
./nix-build.sh image-static passed and loaded firefox-headers:static. Started only new firefox-headers on 8095 with read-only root, cap-drop ALL, no-new-privileges, image uid 65532 and original sibling TLS directory mounted read-only. No protected container touched. Live acceptance running.

## 2026-10-27T18:26:24.505064-04:00 — Stage 9 final acceptance
All local acceptance passed: ./verify.sh 18/0 parser, 43/0 transport root, 9/0 standalone AEAD (overlapping counts); native cargo test --workspace and aarch64-linux ./nix-build.sh check with all fixtures, corpus, deadline/core tests and 100,000 thirteen-name differential cases, zero mismatches. ./nix-build.sh image-static passed. LAN_HOST=203.0.113.28 ./scripts/acceptance.py passed: Firefox and Safari synthetic HTTPS fixture replay 200/favicon 404, curl replay 200, TLS 1.3/ChaCha20Poly1305/X25519/ECDSA P256; TLS 1.2, AES-only, P256-only group, h2-only and plaintext refused; 55 live malformed 400s; name-only diagnostic privacy; delayed 3 s / 200, partial stall 400, idle 17s closes with zero application bytes; 64-worker refusal/recovery. Separate delayed 3s OpenSSL TLS 1.3 request returned 200. Static ELF without interpreter/DT_NEEDED, shell-free/key-free image layers and export passed.

Image sha256:25bb13ad4f9c192947afd0f39fa671e9d9896d6bdff4109f614409ed9cab7e21, size 3096508 bytes, binary 1521952 bytes. firefox-headers remains healthy/running on 8095 with read-only root, cap-drop ALL, no-new-privileges, uid 65532 and original TLS mount read-only. Four protected containers retain identity/config/mount/PID/lifecycle (before/after inspect comparison passed). Final source-scope, proof-exemption, fixture order/CRLF, format, Python syntax and diff checks passed; production TLS/core/shell/manifests unchanged. Only grammar change is Sec-Fetch-User, exactly 13 names; PROOFS.md updated. No protected-file edits, branch changes, push, key/cert writes or captured header-value output/storage. All new fixture values are synthetic and marked. Caller log retained/ignored. Local commit follows using configured identity.

Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage final 338/983=34.38% first-party security-relevant SLoC, explicit union of 129/156/53 proven SLoC from three independent source-bound Verus receipts and unchanged sibling config.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Stage 8 was 337/981=34.35%; original baseline 128/737=17.37%.
Tool source/config unchanged; only authorized in-place build.
Evidence retained in logs/stage9-coverage-*.
No fabricated combined receipt; unknown dependency sizes stay unknown.

Trusted boundary is unchanged from stage 8/PROOFS.md: IO/action shell wiring, event delivery, real-clock deadlines, threads/admission, handlers/routes/diagnostics/writes; rustls protocol and provider adapters/upstream crypto contracts, encoding, randomness, allocation/resources, secret cleanup; grammar/spec adequacy, Verus/Z3/vstd/ghost erasure/compiler, OS/Docker/hardware. No end-to-end TLS proof. Existing cfg(hax) informational exception and offline-yank limitation unchanged. No final local acceptance failures remain. Dudley must independently run acceptance; physical Firefox/Safari, second LAN device, x86_64 and upstream F* reproduction were not tested. No external diary write due explicit repository-only write boundary.

## 2026-10-27T18:55:22.356100-04:00 — Stage 10 start
Read authoritative spec and required history fully. Branch chromium-headers; protected-container snapshot saved; 8095 free. Exactly ASCII-insensitive names, five additions and 24-header cap. Brave root list contains 16 names despite 15 label; preserve listed order. Caller log excluded. No external diary write due repository-only write boundary. TLS/key directory read-only; no values will be printed or logged.

## 2026-10-27T18:56:47.198722-04:00 — Stage 10 implementation/proofs
./verify.sh passed 21/0 parser, 46/0 transport root, 9/0 standalone AEAD; groups overlap. Added verified ASCII folding only for header prefixes, exactly five specified names, and cap 24. Exact method/version/path/value/framing behavior retained. Diagnostics whitelist now matches all 18 names ignoring ASCII case; sanitized rejected names retain sent case. Corrected inherited malformed lowercase-host case to near-miss hosts and 17-header case to 25; former accepted cases now explicitly tested. Brave fixtures preserve 16/14 listed-name orders, synthetic values only. Production transport core and TLS implementation remain byte-identical. Native 100,000-case suite and Linux checks underway.

## 2026-10-27T18:57:16.551687-04:00 — Stage 10 native/coverage
Native cargo test --workspace passed 8 library, 7 boundary, 3 differential, 4 transport tests; 100,000 mixed-case differential cases over 0–24 headers, zero mismatches.
Includes Brave/Safari/Firefox root 200/favicon 404 at whole/1/7/1024 TLS writes, mixed-case 18-name acceptance, 24 accepted/25 rejected, near-miss/forbidden zero-dispatch and inherited idle/slow/core tests. ./verify.sh 21/0, 46/0, 9/0.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage build and measurement passed using unchanged sibling config: 365/1032=35.37% first-party security SLoC; individual reports 156/156/53 proven SLoC, explicit union by identical unit IDs.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Stage 9 was 338/983=34.38%; baseline 128/737=17.37%.
Reports/receipts retained logs/stage10-coverage-*.
No combined receipt fabricated; dependency sizes unknown.
Initial Nix test derivation failed because new fixtures were untracked when source snapshot was made; staged fixtures and rerunning.

## 2026-10-27T18:58:41.143731-04:00 — Stage 10 Linux/image
./nix-build.sh check passed on aarch64-linux after staging fixtures; ./nix-build.sh image-static passed and loaded chromium-headers:static. Started only chromium-headers on 8095, read-only root, cap-drop ALL, no-new-privileges, uid 65532, original TLS directory mounted read-only. Production sources unchanged since native proofs/tests/coverage. Linux evidence and live acceptance follow.

## 2026-10-27T18:59:55.521360-04:00 — Stage 10 final acceptance
All local acceptance passed: ./verify.sh 21/0 parser, 46/0 transport root, 9/0 standalone AEAD (overlapping groups); native workspace tests; ./nix-build.sh check on aarch64-linux, proof-log/test-log/deny-log; ./nix-build.sh image-static; LAN_HOST=203.0.113.28 ./scripts/acceptance.py; separate delayed 3s OpenSSL TLS 1.3 request 200. Native/Linux 100,000 differential cases cover mixed-case 18-name whitelist and 0–24 headers with zero mismatches. Brave root 200/favicon 404 and all Safari/Firefox fixtures pass whole/1/7/1024 writes; new mixed-case whitelist and 24/25 boundaries pass over TLS and plaintext; near-miss/forbidden names reject with zero handlers. Live 55 malformed requests 400; exact sent-case sanitized diagnostics and value privacy pass. TLS 1.3/ChaCha20Poly1305/X25519/ECDSA P256 passes; TLS 1.2, AES-only, P256-only key exchange, h2-only and plaintext refused. Delayed 3 s / 200, partial stall 400, idle 17s closes with zero application bytes, 64-worker refusal/recovery pass.

Image sha256:1f27970ba84491c90ebb894782025ee3e5c215db6fec51501aae4a4ed193aed1, size 3096509 bytes, binary 1522416 bytes. Static ELF without interpreter/DT_NEEDED, shell-free/key-free layers/export; chromium-headers remains healthy/running on 8095, read-only root, cap-drop ALL, no-new-privileges, uid 65532, original sibling TLS bind read-only. Protected-container comparison passed for scratch-server, nix-scratch, parser-http, parser-https: identity/config/mount/PID/lifecycle unchanged. Source audit confirms branch chromium-headers, protected files untouched, no proof exemptions, unchanged production transport core, TLS, IO transport, main and manifests. Exactly three grammar changes: ASCII-insensitive header names, five specified additions, 24-header cap. PROOFS.md and README updated. Formatter/Python syntax/diff checks passed. Evidence logs/stage10-*. Caller log ignored and retained. No push, branch switch, key/cert writes or captured values stored/printed. No external diary write due explicit repository-only write boundary. Local commit follows under configured Dudley identity.

Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage final 365/1032=35.37% first-party security-relevant SLoC, explicit union of 156/156/53 proven SLoC from three source-identical reports and independent source-bound Verus receipts; unchanged sibling config and tool source.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Stage 9 was 338/983=34.38%; original baseline 128/737=17.37%.
No fabricated combined receipt; unknown dependency sizes stay unknown.

Trusted boundary remains stage 8/PROOFS.md: IO/action shell wiring, event delivery, real-clock deadlines, threads/admission, handlers/routes/diagnostics/writes; rustls protocol/provider adapters and upstream crypto contracts, encoding, randomness, allocation/resources, secret cleanup; grammar/spec adequacy, Verus/Z3/vstd/ghost erasure/compiler, OS/Docker/hardware. No end-to-end TLS proof. Inherited cfg(hax) informational advisory exception/offline-yank limitation unchanged. No final local acceptance failures remain. Dudley must independently run acceptance; physical Brave/Firefox/Safari, second LAN device, x86_64 and upstream F* reruns were not tested.

## 2026-10-27T19:03:52.054747-04:00 — Stage 11 start
Read PRAGMA-HEADER.md fully first and all required references. Confirmed pragma-header branch; protected four-container snapshot saved; 8095 free. Only Pragma added to the grammar and diagnostic whitelist; nineteen names, 24-header cap, all other grammar and production transport/TLS unchanged. New cache-disabled fixture preserves stage 10 Brave root name order with Pragma before Cache-Control; all values synthetic, never captured. Caller run log retained/ignored. Certificate/key read only; no external diary write because explicit task boundary permits writes only in this repository (plus authorized sibling tool build).

## 2026-10-27T19:04:08.119027-04:00 — Stage 11 proofs
./verify.sh passed 21/0 parser, 46/0 transport root, 9/0 standalone AEAD (overlapping obligations). Parser diff limited to Pragma spec/executable prefix and returned-name bound 18→19. Transport core and production TLS untouched. Initial test compilation caught a control character introduced by a Python replacement string in new near-miss test entries; corrected before rerunning tests. No proof exemptions.

## 2026-10-27T19:05:02.765805-04:00 — Stage 11 coverage/source audit
Sibling formal-coverage build and python3 scripts/measure-coverage.py passed using unchanged examples/scratch-server config and stage 10 three-run union method.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Final 366/1035 first-party security-relevant SLoC=35.36%; individual reports 157/156/53 proven SLoC on identical 1035-SLoC inventories.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Stage 10 was 365/1032=35.37%; historical baseline 128/737=17.37%.
Reports/receipts/explicit unit union retained in logs/stage11-coverage-*.
No combined verifier receipt fabricated; dependency sizes remain unknown.
Source audit passed: exact Pragma-only parser refinement, diagnostic addition only in production shell, unchanged transport core and production TLS/manifests, no proof exemptions, protected files untouched.
Fixture name-order/17-header/CRLF assertions, Python syntax, format and diff checks passed.
A new test-list type mismatch caught by compilation was corrected before final workspace run.

## 2026-10-27T19:05:51.107311-04:00 — Stage 11 native/Linux acceptance
Native cargo test --workspace and aarch64-linux ./nix-build.sh check passed. Retrieved proof-log/test-log/deny-log: 21/0 parser, 46/0 transport, 9/0 AEAD; 8 library, 7 boundary, 3 differential and 4 transport tests; all four dependency categories passed. 50,000 valid and 50,000 mutated/arbitrary cases across mixed-case nineteen-name whitelist and 0–24 headers, zero mismatches. Brave cache-disabled root 200 plus inherited Brave/Safari/Firefox root 200/favicon 404 pass whole/1/7/1024 TLS writes. Pragma case variants and Pragm/Pragmas zero-handler tests pass; empty/257-byte invalid values rejected and 256-byte boundary accepted. Inherited corpus, deadline/idle/slow/handshake/core tests pass. Existing informational cfg(hax) advisory exception and offline registry-yank limitation unchanged. x86_64 not tested. Static image build continues.

## 2026-10-27T19:06:40.645580-04:00 — Stage 11 image/runtime start
./nix-build.sh image-static passed and loaded pragma-header:static, including musl workspace tests. Started only new pragma-header on 8095 with read-only root, cap-drop ALL, no-new-privileges, image uid 65532 and original sibling TLS directory mounted read-only. Protected containers untouched. LAN_HOST=203.0.113.28 ./scripts/acceptance.py running.

## 2026-10-27T19:07:41.181518-04:00 — Stage 11 final acceptance
All local acceptance passed: ./verify.sh 21/0 parser, 46/0 transport root, 9/0 standalone AEAD (overlapping groups); native workspace tests with 100,000 mixed-case nineteen-name differential cases/zero mismatches; aarch64-linux ./nix-build.sh check, proof-log/test-log/deny-log; ./nix-build.sh image-static; LAN_HOST=203.0.113.28 ./scripts/acceptance.py; separate delayed 3s OpenSSL TLS 1.3 request 200. Brave cache-disabled synthetic fixture 200 at whole/1/7/1024 writes, plus all existing fixtures pass. Pragma case variants accepted; Pragmas/Pragm rejected with zero handlers at all four sizes. Live 57 malformed 400s, name-only case-preserving diagnostic privacy, TLS 1.3/ChaCha20Poly1305/X25519/ECDSA P256 pass; TLS 1.2, AES-only, P256-only group, h2-only and plaintext refused. Delayed 3 s / 200, partial stall 400, idle 17s closed with zero application bytes, 64-worker refusal/recovery pass.

Image sha256:aa5739979f0e40359b703e0666a28c38a310a740b1c198e42304a0cec060afb1, size 3096506 bytes, binary 1522512 bytes. Static ELF without interpreter/DT_NEEDED, shell-free/key-free image layers/export pass. pragma-header healthy/running on 8095, read-only root, cap-drop ALL, no-new-privileges, uid 65532, original sibling TLS directory mounted read-only. Protected-container comparison passes all four identities/configs/mounts/PIDs/lifecycles unchanged. Exact grammar change is Pragma only, nineteen names; all other grammar, limits, transport core and production TLS unchanged. PROOFS.md lists 19 names and updated parser references; README documents this stage. No branch switch, push, protected-file edits, key/cert writes or captured header-value output/storage. New fixture values are synthetic, never captured; caller log retained/ignored. No external diary write due explicit repository-only write boundary. Local stage commit follows using configured identity.

Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Final formal-coverage 366/1035=35.36% first-party security-relevant SLoC, explicit union of 157/156/53 proven SLoC from three source-identical reports and independent source-bound Verus receipts; unchanged sibling config/tool source.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Stage 10 was 365/1032=35.37%; historical baseline 128/737=17.37%.
No combined receipt fabricated; dependency sizes remain unknown.
Evidence retained under logs/stage11-*.

Trusted boundary remains stage 8/PROOFS.md: IO/action shell wiring, event delivery, real-clock deadlines, threads/admission, handlers/routes/diagnostics/writes; rustls protocol/provider adapters and upstream crypto contracts, encoding, randomness, allocation/resources, secret cleanup; grammar/spec adequacy, Verus/Z3/vstd/ghost erasure/compiler, OS/Docker/hardware. No end-to-end TLS proof. Inherited informational cfg(hax) exception/offline-yank limitation unchanged. No final local acceptance failures remain. Dudley must independently run acceptance; physical Brave/Firefox/Safari, second LAN device, x86_64 and upstream F* reruns were not tested.


## 2026-10-27T19:24:05-04:00 — Stage 12 proof blocker: incompatible accepted sets
Read HEADER-POLICY.md fully first, then all required references. Confirmed branch
header-policy; no checkout, push or production source changes. The authoritative
spec simultaneously requires (a) always rejecting duplicate Host or Connection,
for every policy, and (b) strict-mode accepted-set equality to stage 11's
request_spec, with unchanged stage-11 tests/oracle. These sets cannot be equal:
stage 11 accepts a GET / HTTP/1.1 request containing exactly two well-formed Host
lines, and likewise two Connection lines. Both names are known, and mixed case
does not change their identity. Stage 12 must reject each witness even in strict
mode. No header values are needed to describe the contradiction.

Added tests/differential.rs:: stage11_accepts_duplicate_host_and_connection as a
reproducible diagnostic against the unchanged parser and independent stage-11
oracle/httparse. It asserts both duplicate-name witnesses accept and retain two
ordered header entries, without printing request/header bytes. The existing
stage-11 boundary test also builds 16/17/23/24 repeated Host lines and expects
acceptance, so running every stage-11 test unchanged is independently incompatible
with the new duplicate rule. This is a false equivalence obligation, not a solver
limitation. Per the explicit stop rule, do not implement policy or weaken either
requirement. Andrew must resolve the spec conflict. A possible revision for his
approval: strict accepted set equals the stage-11 set restricted to no duplicate
Host/Connection; update affected boundary fixtures to repeat another quiet name.
That revision has NOT been made or assumed authorized.

./verify.sh passed on unchanged production sources: parser 21 verified/0 errors, transport root 46/0, standalone AEAD 9/0 (groups overlap).
These establish existing stage-11 properties only; no Stage 12 policy proof is claimed. cargo fmt --all -- --check passed.
Formal coverage remeasured using scripts/measure-coverage.py with only its output prefix replaced in memory by stage 12-blocked-coverage, preserving the script and sibling tool/config unchanged.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Three source-bound receipts and unit union: 366/1035=35.36% first-party security-relevant SLoC; individual proven SLoC 157/156/53, dependency sizes unknown.
Evidence: logs/stage12-blocked-*.

No policy parser/decision/request isolation/log limiter implemented. Nix check,
image-static, Stage 12 config fuzzing, default/strict TLS acceptance and deployment
not run. No container operations or TLS/key reads/writes were needed; all four
protected containers and TLS/transport sources are untouched. No writes outside
this repository or external diary entry, respecting the task's explicit boundary.
Caller run log retained and ignored consistently with previous stage logs.

## 2026-10-27T19:24:16-04:00 — Stage 12 blocker checks complete
CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" cargo test
--test differential passed all 4 tests: duplicate Host/Connection counterexamples,
inherited grammar boundaries, 50,000 valid and 50,000 mutated/arbitrary inputs,
zero mismatches against the stage-11 oracle. git diff --check passed. Only notes,
progress, the diagnostic test and caller-log ignore entry changed. Local blocker
commit uses the configured repository identity; no Stage 12 completion claimed.

## 2026-10-27T19:25:21.145329-04:00 — Stage 12 resumed
Read amended HEADER-POLICY.md and CODEX_PROMPT.md. Proof 3 now intersects stage 11 with no duplicate Host/Connection; blocker resolved. Implement policy as data that has no hard-reject or use overrides; keep legacy grammar as ghost/reference plus private strict parser. Request exposes method/path only. Minute logging uses monotonic fixed 60-second windows shared across workers; names deduplicate ignoring ASCII case, storage 64 and a full table suppresses further names rather than evicting. Config whitespace allows spaces/tabs and CRLF; comments strip at first #, LF terminates a line, trailing LF adds no extra line. No external diary writes under repository-only boundary. Caller log retained.

## 2026-10-27T19:32:40.457647-04:00 — Stage 12 policy/proof checkpoint
./verify.sh passed 56/0 parser, 82/0 transport root, 9/0 standalone AEAD (overlap).
Policy representation has no hard-reject override or Use; Request only projects method/path.
Strict mode retains the unchanged private stage 11 grammar and applies the verified duplicate/policy filter; other policies use token grammar. accepted_safety, strict_equivalence and structural legacy-name induction pass.
Only required transport-core changes: carry policy, invariant/Dispatch contract parameter, policy-aware parser call; formatting/AEAD/TLS implementation unchanged.
Native workspace tests passed, including default sensitive-name TLS400/zero calls, Cookie-drop 200, browser/DNT fixtures whole/1/7/1024, strict inherited tests, 10,000 generated configs and 20 malformed startup refusal cases plus missing file.
The first Linux build omitted config fixtures from the Nix fileset; added config and rerunning.
A directive-length typo was caught by the explicit-default test and fixed.
Logging now uses sliding 60-second windows, a stronger bound than the initial fixed-window choice: suppress when 64 names tracked, no live-name eviction, shared Mutex across workers.
Case-insensitive dedup, configured limit 0 supported.
Initial coverage was rejected because a logger method name matched the tool's proof-exemption keyword; renamed to should_log (no proof bypass existed).
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Final remeasurement with unchanged sibling tool/config yields 715/1685=42.43%, explicit union of 502/160/53 proven SLoC.
Receipts/logs retained in logs/stage12-*.

## 2026-10-27T19:34:46.677326-04:00 — Stage 12 Linux checks
./nix-build.sh check passed aarch64-linux; proof-log confirms 56/0, 82/0, 9/0.
Linux test-log confirms 10 library, 7 boundary, 4 differential, 3 policy, 4 transport
passed (100,000 inputs, three policies each, zero mismatches). cargo-deny
advisories/bans/licenses/sources passed; inherited cfg(hax) informational exception
and offline-yank limitation unchanged. x86_64 omitted and untested. TLS provider
implementation before cfg(test), src/transport.rs and aead.rs byte-identical to
afd8d42; protected documents unchanged. Static image build underway. New default
live corpus excludes original cases 33(hosts) and 39(X-Test), now valid unknowns;
strict native corpus retains all 48 including empty EOF. Header-count fixtures
repeat Accept so they isolate 24/25 boundary from duplicate-Host rejection.
Brave+DNT preserves Sec-GPC exactly once (already in the Brave fixture), using
synthetic DNT/Sec-GPC values as required. Native default TLS fixtures and all 25
sensitive names pass whole/1/7/1024 writes, with zero handler calls on rejects.


## 2026-10-27T19:38:09-04:00 — Stage 12 final local acceptance
All local acceptance commands passed: ./verify.sh 56/0 parser, 82/0 transport root,
9/0 standalone AEAD (overlapping groups); native workspace tests; aarch64-linux
./nix-build.sh check and retrieved proof/test/deny logs; ./nix-build.sh image-static;
LAN_HOST=203.0.113.28 ./scripts/acceptance.py; profile checks builtin/strict/cookie;
separate delayed 3s OpenSSL TLS 1.3 request 200. Linux and native tests include 10
library, 7 boundary, 4 differential, 3 policy, 4 transport tests. 100,000 differential
inputs compare default/strict/Cookie-drop policies against the independent grammar
and httparse (300,000 policy comparisons), zero mismatches. 10,000 generated
configs either refuse or preserve hard-reject. 20 malformed-config startup cases
plus missing-file refusal fail before TLS loading or port binding. Initial config
typo/Nix fileset failures were resolved; no final local acceptance failure remains.

Default Safari/Firefox/Brave/cache-disabled/Brave+DNT fixtures pass whole and
1/7/1024 TLS writes; DNT logs exactly once across those requests. Sec-GPC is quiet.
All 25 hard-reject/guarded names mixed-case reject at all chunk sizes with zero
handler calls in native TLS tests; Cookie-drop accepts. Live strict mounted config
rejects DNT with reason=unknown_header at all chunk sizes; Host/Connection
duplicates reject with reason=duplicate in every live profile. 24 Accept headers
accept, 25 reject with reason=too_many_headers. Final default live 1000 TLS requests,
100 distinct names, 8 workers, <60 seconds yielded 59 unknown-note lines (the prior
DNT used the other default budget slot); bound 60, no repeated names. Unit tests
exercise limits 0/1/60/600 and 1000 iterations per limit, <=64 retained names, case
folding, suppression at capacity and expiry at 60 seconds. Name storage is bounded
by 64 entries of at most 64 request bytes; no rolling live-name eviction.

TLS 1.3/ChaCha20Poly1305/X25519/ECDSA P256, curl via same-host LAN address, all
routes/healthcheck, 48 live malformed 400s, partial stall 400, idle 17s with zero
application bytes, delayed 3 s / 200 and 64-worker refusal/recovery passed. TLS 1.2,
AES-only, P256-only key exchange, h2-only and plaintext refused. Full image/export
checks confirm static ELF without interpreter/DT_NEEDED, shell-free/key-free
image layers. Image sha256:a6983d135991ed32285ed4260a26c7584be651add47c6459216ba6b2082b5fd9, size 3114938 bytes,
binary 1530576 bytes. header-policy remains healthy/running on 8095:
read-only root, cap-drop ALL, no-new-privileges, uid/gid 65532, original TLS directory
mounted read-only and config/header-policy.conf mounted read-only at /config with
SCRATCH_HEADER_POLICY=/config. Only this stage's own header-policy container was
replaced between profiles; all four protected container IDs/configs/mounts/PIDs/
lifecycles match the before snapshot. No protected files or branches changed.
No push, key/cert writes, captured-value output or external memory writes.

Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage final 715/1685=42.43% first-party security-relevant SLoC; explicit union of 502/160/53 proven SLoC from three accepted source-bound receipts on identical inventories and unchanged sibling tool/config.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Stage 11 was 366/1035= 35.36%; historical baseline 128/737=17.37%.
Current Rust source hashes match the receipts.
No combined receipt fabricated; dependency sizes remain unknown.
Evidence retained in logs/stage12-*; original blocked-stage evidence also retained.
The initial native log has sparse/NUL regions from two overlapping writes; both processes exited 0, and final Linux test-log is complete/unambiguous.
Coverage tool source/config unchanged.
Cargo formatter, Python syntax, scope/privacy token scan, fixture order/CRLF and diff checks passed; local final checkpoint follows using configured Dudley identity.
Caller run logs retained/ignored.

Trusted boundary remains policy-file IO/startup wiring and diagnostic/log-budget
wiring, socket/event delivery and real-clock scheduling, worker admission,
handler/routes/response writes, rustls protocol/provider adapters/upstream crypto
contracts, encoding, OS randomness, allocation/resources and secret cleanup;
grammar/spec adequacy, Verus/Z3/vstd/ghost erasure/compiler, OS/Docker/hardware.
Production TLS before cfg(test), IO transport and AEAD helper source are unchanged.
Core changes only carry policy and connect its parser/spec; no TLS policy or
transport deadlines/formatting changes. No end-to-end TLS or wall-clock liveness
proof. Inherited informational cfg(hax) exception/offline-yank limits unchanged.
Dudley must independently run acceptance. Physical browsers, a second LAN device,
x86_64 and upstream F* reproduction were not tested.

## 2026-10-27T19:46:20.592341-04:00 — Stage 13 start
Read SPEC-LEMMAS.md fully first, then HEADER-POLICY.md, PROOFS.md, PROGRESS.md and NOTES.md. Branch spec-lemmas confirmed. Caller log retained and ignored. No deployment or protected-container reuse; TLS acceptance will use the existing native loopback tests rather than scripts/acceptance.py, which targets the protected header-policy container. Repository-only write boundary also excludes external diary writes. Intent proofs will live in a separate ghost module; executable production sources remain unchanged.

## 2026-10-27T19:55:36.689609-04:00 — Stage 13 executable checks and proof strategy
Native cargo test --workspace passed, including 100,000 differential inputs under three policies, 10,000 generated configs, existing loopback TLS tests and three new intent tests. No protected-container reuse or certificate/key access needed. Traversal tests cover GET/HEAD, strict/default and 1/7/1024 read chunks; code review confirms exact route matching sends all other paths straight to the fixed 404 reply without filesystem access (only the exact root route probes fixed binary paths).
Reduced browser witnesses retain every header name and order from each source fixture, replacing each value by one synthetic printable byte. Pinned Verus compute panics on byte literals and on the quantified header model; use ordinary SMT proofs and reveal_byteslit, not compute. Separate per-header construction helpers keep solver queries small. The original literal_bytes model helper exposes existing literal contents using standard verifier machinery; it adds no axioms. Universal path/value/count helpers and accepted-policy statements have passed targeted verifier runs; full verify remains pending.

## 2026-10-27T20:10:48.798044-04:00 — Stage 13 implementation checkpoint
New executable intent checks passed with the workspace suite. Intent catalogue and isolated mutant harness are present; full proofs and mutant outcomes are pending, so no stage completion or formal-coverage result is claimed. Helper splitting and explicit sequence extensionality address solver cost; a generated fixture-class casing mistake was corrected before verification (fixture names themselves were preserved). No production executable code changed. Local checkpoint records work in progress using repository identity.

## 2026-10-27T20:10:48.798114-04:00 — Stage 13 proof decomposition and mutants
All fifteen mutated spec/executable pairs passed ./verify.sh with intent excluded in development consistency runs. Empty-value mutation guards both first/last value-byte accesses when empty and updates the matching header-bound annotations; no intent lemma is edited in a mutant copy. Expect maps to guarded class 28 (the existing Cookie permission bit), sufficient to model the forbidden hard-to-guarded transition under a valid policy with all guarded bits enabled. Bare-LF mutation admits actual LF header line terminators and final blank lines, retaining CRLF request lines; added an explicit LF-header attack test. These small choices retain all fifteen specified fault classes.
Separate proof modules permit Verus to verify independent facts in parallel. The 25-header witness now has one suffix lemma per header budget, preserving the same rejection conclusion. Generic framing proof splits clean folded matching from the known-prefix alternatives. Corrected generated proof bookkeeping (case-insensitive fixture class IDs, explicit sequence equality, colon-space facts only for nonzero raw prefixes); no production spec or executable semantics changed. Earlier aggregate runs were interrupted while these proof obligations were repaired; final results remain pending. No proof weakening or mutation deletion.

## 2026-10-27T20:17:46.686752-04:00 — Stage 13 proof and coverage checkpoint
./verify.sh passed with the pinned tool: parser 211 verified/0 errors, transport root 237/0, standalone AEAD 9/0.
Evidence: logs/stage13-mutants/baseline-proofs.log from the completed proof phase before an interrupted redundant test run, independently reproduced by all three accepted formal-coverage receipts.
Current Rust hashes match those receipts.
There are 42 intent statements and 112 supporting lemmas, catalogued individually. cargo test --test intent passed all 3 tests, including the added bare-LF-header witness; no production executable changes.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage using unchanged sibling tool/config reports 715/6095=11.73%, explicitly unioning three accepted source-bound reports.
All 4410 added security SLoC are in the ghost intent.rs module, which v1 does not attribute as proven (unit statuses tested/unverified).
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): The unchanged remainder is 715/1685=42.43%, an explanatory subtraction only.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Report the raw 11.73% rather than changing configuration or claiming the tool supports these ghost modules.
This is a reporting limitation, not a runtime coverage regression or a spec/code bug.
Tool source/config remain read-only and unchanged.
Mutation logs now whitelist outcomes, source locations and test names before writing: no request literals, proptest counterexample bytes or assertion payloads are logged.
Earlier diagnostic logs were similarly filtered; formal-coverage logs/receipts were left intact.
The full end-to-end mutant run performs its own baseline verification and workspace tests, then all 15 consistent mutants with focused intent checks and full workspace tests.
A resource-limited intent query is inconclusive and cannot count as a kill; a genuine test failure can still kill that mutant.
Final mutation and Linux outcomes remain pending.

## 2026-10-27T20:19:09.429050-04:00 — Stage 13 Linux checks
./nix-build.sh check passed on aarch64-linux; retrieved proof-log/test-log/deny-log. Proof counts 211/0 parser, 237/0 transport, 9/0 AEAD match native. Full workspace tests, including the new intent suite, policy-aware 100,000 differential inputs, 10,000 config cases and loopback TLS acceptance, pass. Dependency advisories/bans/licenses/sources pass under the inherited informational cfg(hax)/offline-yank limitations. x86_64 remains untested. No image build or deployment; only temporary unnamed Nix check containers were used. An earlier in-progress Nix check was cancelled before this successful final-source rerun; protected named containers were never operated on.
The method mutant minimally changes GET matching to ASCII-insensitive (including the specified lowercase-get attack); HEAD remains unchanged. This isolates the required fault without adding other accepted methods. Aggregate mutation results remain pending.

## 2026-10-27T20:40:25.384182-04:00 — Stage 13 final local acceptance
python3 scripts/spec-mutants.py completed end to end with exit 0: 15/15 killed, no survivors, no malformed/invalid runs.
Each mutation preserved all preexisting ./verify.sh groups with intent excluded, then restored the unchanged intent module for the focused proof and ran all workspace suites with --no-fail-fast.
Thirteen focused intent checks failed; unknown-reject and Cookie-drop passed their fixed-policy proof checks but were killed by actual built-in fixture/config tests, as required.
All fifteen also failed actual tests.
No solver exhaustion was counted.
MUTANTS.md records every fault, what killed it and the empty survivor set; none removed or weakened.
The source/proof/tooling code is the version committed in 52585b9; subsequent changes are documentation only.
Fresh baseline ./verify.sh passed 211 verified/0 errors parser, 237/0 transport, 9/0 AEAD; baseline cargo test --workspace passed 10 library, 7 boundary, 4 differential, 3 intent, 3 policy, 4 transport tests. 100,000 differential inputs compare three policies (300,000 comparisons), zero mismatches; 10,000 config fuzz cases, malformed startup refusal, existing loopback TLS/browser/fragmentation/corpus/deadline tests and new config/traversal/attack tests all pass.
Independent aarch64-linux Nix proofs/tests/dependency checks pass with matching counts.
No x86_64, physical browser, LAN deployment or upstream crypto proof rerun is claimed.
Historical first-party measurement (formal-coverage v1, 2026-10-27; superseded by v1.4): Formal-coverage remains 715/6095=11.73% in the unchanged tool/config, with 4410 security SLoC in ghost intent.rs unattributed by v1; unchanged remainder 715/1685=42.43% is explanatory only.
Three accepted source-bound receipts, individual reports and explicit unit union retained.
Final source hashes match receipts; exact fixture-name/order reductions, no forbidden proof tokens, unchanged src/transport-core, no executable parser function changes and untouched EVOLUTION.md/ATTACK-SURFACE.md audited.
Only the ghost module declaration was added to existing http-subset lib.rs.
Header values/private keys were not included in mutation logs or commits.
No TLS sibling access, outside-repository writes, external diary write, protected-container operation, branch change or push.
Final documentation and local commit follow using configured Dudley identity.
Independent acceptance by Dudley remains outstanding; these are local command results, not an independent acceptance claim.

## 2026-10-27T20:41:20.872767-04:00 — Stage 13 commit and final audit passed
Final local commit completed under configured Dudley identity. All 15 results, 154 catalogue rows, both native/Linux 211/237/9 proof summaries, intact source/log hashes for all three coverage receipts and protected-source/branch checks passed the final audit. git diff --check passed. Stage 13 checklist now records only commands that completed successfully. No push or deployment. Independent acceptance remains Dudley's.

## 2026-10-29 — Release M1 started
Re-probed branch spec-lemmas at 5b6fcc5 (stage tip 2fa5eb7), 19 main-only commits, and existing Docker names without operating on them. Existing untracked codex-run-release.log is operator output and ignored. Main worktree created using the requested TMPDIR path. Both image derivations now depend on the exact checks exposed by flake check. Runner digest taken from locally installed nixos/nix RepoDigests. Advisory archive remains pinned; The original libfaketime change intended to evaluate freshness at the pinned commit timestamp; independent acceptance proved its preload path ineffective, so it still used the real clock (corrected in Release fix F1 below). Refresh archive revision, sha256, FETCH_HEAD timestamp and FAKETIME together, then rerun dependency checks and review advisories. Snapshot checks do not claim current advisory status.

## 2026-10-29 — Release M1 acceptance
`./nix-build.sh image-static` succeeded and loaded only release-scratch-server:static. The hardened release-health container, using a freshly generated localhost certificate, passed the binary's --healthcheck and Docker reports healthy. Fresh native verification reports 211/237/9 verified with zero errors; cargo test --locked --workspace passes all 31 tests.

Negative runs used throwaway clones under target/release-checks, with the release flake copied in. In proof, replaced one existing AEAD proof assertion with assert(false); no runtime behavior changed. In deny, removed MIT from deny.toml's allow list. Both `nix build .#image-static` commands exited 1, naming the expected failed proof or dependency derivation. Commands (run from the repository root):

```sh
docker run --rm --name release-negative-proof \
  -v "$PWD/target/release-checks/proof":/src -w /src \
  -v scratch-server-nix-store:/nix \
  -e GIT_CONFIG_COUNT=1 -e GIT_CONFIG_KEY_0=safe.directory -e GIT_CONFIG_VALUE_0=/src \
  nixos/nix@sha256:f159a58f390677f41e8e900a74b6e2f9c7f27b364122ae6acf7a652ce6c551c0 \
  nix --extra-experimental-features 'nix-command flakes' build .#image-static \
  --no-update-lock-file > logs/release-negative-proof.log 2>&1
```

Tail output:

```text
       >       (Note: triggers are used by the underlying SMT theorem prover to instantiate quantifiers;
       >       the theorem prover instantiates a quantifier whenever some expression matches the
       >       pattern specified by one of the quantifier's triggers.)
       >    --> crates/transport-core/src/../../http-subset/src/lib.rs:582:25
       >     |
       > 582 |     no_duplicates(h) && forall|i: int| 0 <= i < h.len() ==> drops(p, h[i].name)
       >     |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
       >
       > verification results:: 236 verified, 1 errors
       > error: aborting due to 1 previous error
       >
       For full logs, run:
         nix log /nix/store/jkc1j31sam1rdjbgzj0bjblhj1jirchi-verified-boundary-proofs.drv
building '/nix/store/bvxx8shqwrfbbag9bdibycq8qgf8hqyq-release-scratch-server-base.json.drv'...
error: Cannot build '/nix/store/cnb26gv8a6si9ci7fxkf5r6s7bncx58k-release-scratch-server-customisation-layer.drv'.
       Reason: 1 dependency failed.
       Output paths:
         /nix/store/kaa8z1s3gxvkds83yhwfa14rp82bm1v3-release-scratch-server-customisation-layer
error: Cannot build '/nix/store/nmpz7zf5dxi60j3b28m7wr3j23fn5w4y-release-scratch-server-conf.json.drv'.
       Reason: 1 dependency failed.
       Output paths:
         /nix/store/51iv166kzdp3iwnr60ah635ckd61ljnr-release-scratch-server-conf.json
error: Cannot build '/nix/store/y8zaqay74mrjf6pgwzp9m26kvsd096wc-stream-release-scratch-server.drv'.
       Reason: 1 dependency failed.
       Output paths:
         /nix/store/5x07b0ddsf2bn36xr51jm5qcpdz922rh-stream-release-scratch-server
error: Cannot build '/nix/store/4192p9p55b9bq9s8afr5j7p0fid3yc2r-release-scratch-server.tar.gz.drv'.
       Reason: 1 dependency failed.
       Output paths:
         /nix/store/k12jmhjq4f78adsczs9xgq1f92g8ywv9-release-scratch-server.tar.gz
```

```sh
docker run --rm --name release-negative-deny \
  -v "$PWD/target/release-checks/deny":/src -w /src \
  -v scratch-server-nix-store:/nix \
  -e GIT_CONFIG_COUNT=1 -e GIT_CONFIG_KEY_0=safe.directory -e GIT_CONFIG_VALUE_0=/src \
  nixos/nix@sha256:f159a58f390677f41e8e900a74b6e2f9c7f27b364122ae6acf7a652ce6c551c0 \
  nix --extra-experimental-features 'nix-command flakes' build .#image-static \
  --no-update-lock-file > logs/release-negative-deny.log 2>&1
```

Tail output:

```text
       >                       │   │   ├── libcrux-p256 v0.0.9 (*)
       >                       │   │   ├── libcrux-sha2 v0.0.9
       >                       │   │   │   ├── libcrux-ecdsa v0.0.9 (*)
       >                       │   │   │   ├── libcrux-hmac v0.0.9 (*)
       >                       │   │   │   ├── libcrux-p256 v0.0.9 (*)
       >                       │   │   │   └── scratch-server v0.1.0 (*)
       >                       │   │   └── libcrux-sha3 v0.0.11 (*)
       >                       │   └── scratch-server v0.1.0 (*)
       >                       └── libcrux-sha3 v0.0.11 (*)
       >
       > advisories ok, bans ok, licenses FAILED, sources ok
       For full logs, run:
         nix log /nix/store/fwn6achzpfm50xfhcg808rwgkvkib2ld-verified-boundary-dependencies-0.1.0.drv
building '/nix/store/qndm1pzgsc96wyigm78b10jvxiiw0wrn-release-scratch-server-base.json.drv'...
error: Cannot build '/nix/store/gj3m6gly6d8sqykcdrg24l2iyksf5k3p-release-scratch-server-customisation-layer.drv'.
       Reason: 1 dependency failed.
       Output paths:
         /nix/store/m6ss6ays7ldf862n27mm6w49y5zqaqnb-release-scratch-server-customisation-layer
error: Cannot build '/nix/store/py0xpabk5nmj4saw0m7ljq7wv6sffcg1-release-scratch-server-conf.json.drv'.
       Reason: 1 dependency failed.
       Output paths:
         /nix/store/ndban6k3hlfdvy8bhxa066rxhynnijm2-release-scratch-server-conf.json
error: Cannot build '/nix/store/58a089fycih4rg47qvkk7kiac1walhdv-stream-release-scratch-server.drv'.
       Reason: 1 dependency failed.
       Output paths:
         /nix/store/x14c3pd2bkk1k3z4lbcfpkxr8jlvnh7n-stream-release-scratch-server
error: Cannot build '/nix/store/4lx0gvbscdyvb96gfg94gsl1xwvx4g0p-release-scratch-server.tar.gz.drv'.
       Reason: 1 dependency failed.
       Output paths:
         /nix/store/xhzjxmwlqq9w5f5h9g4rd9srpswxj5d3-release-scratch-server.tar.gz
```

## 2026-10-29 — Release M2
Current coverage uses the authoritative v1.4 baseline at 2fa5eb7 and package source SLoC counting rule. Historical v1 build records are explicitly dated and superseded; no new formal-coverage measurement is claimed. Main describes stage 13 separately from its stage-1 code and leaves every per-stage coverage cell pending.

## 2026-10-29 — Release M3 preparation
The coverage-document helper stopped on main's already-updated baseline after M1. Its uncommitted duplicate main edit was restored immediately. M2 committed the historical first-party annotations; the public README/PROOFS rewrite here completes the v1.4 current statement. No commits were amended or rewritten.
README now explains the threat model, proof/test/trust boundaries, build/run/verify and actual TLS/architecture/reproducibility limits. Scripts accept container/port/host, TLS directory and tool checkout arguments. The certificate host defaults to localhost. Every tracked LAN fixture address was replaced with the equal-length TEST-NET-3 address; no grammar, limits, expected outcomes, proof, intent lemma or mutant changed. Build records moved to dev, links updated and absolute home paths replaced by placeholders. Typography changes are confined to Markdown prose outside fenced code and inline code spans. Private names/project references were scrubbed using the external denylist, which remains untracked.

## 2026-10-29 — Release script acceptance
The parameterized acceptance script passed against release-health at localhost:18095 with the new private TLS directory. It exercised all existing checks: 48 malformed requests, browser fixtures at four chunk sizes, TLS restrictions, healthcheck, static ELF/no-shell filesystem, deadlines, capacity, and 1,000 logging probes (59 lines <= 60). No LAN or physical browser rerun is claimed. After the equal-length fixture substitution, workspace tests and the complete mutant runner are rerunning. Missing formal-coverage path fails clearly with argparse exit 2; no external tool checkout was accessed.

## 2026-10-29 — Release M4 started
Added MIT LICENSE (Copyright (c) 2026 Andrew VanLoo) and license = MIT to all three packages. Removed cargo-deny's private-package license exclusion so our own crates are now checked alongside third-party crates; advisory exception, licenses allow list, bans and source policy remain unchanged. Final Nix check and image build will validate the licensed tree; no Rust source, proof, test or mutant edits.

## 2026-10-29 — Release resume after API interruption
The API capacity failure terminated the host sessions for final verification, tests, mutants, Nix check and image build. Their log tails ended without final success summaries, and process probes found no surviving host runners; those runs are not counted as passes. Saved their interrupted logs. The existing versions container and release-health remain alive and are being allowed to finish; no duplicate versions job was launched. M4 remains uncommitted. The resumed operator log is ignored alongside the original operator log. Main's release commits and worktree cleanup were already complete.

## 2026-10-29T13:19:10-04:00 — Release M4 checks passed
The resumed ./nix-build.sh check and image-static both exited 0. Nix proof-log contains 211/237/9 verified, zero errors; dependency evidence ends with advisories/bans/licenses/sources all OK, now including the MIT-licensed workspace crates. The fresh native mutant baseline also ran ./verify.sh successfully with 211/237/9 and zero errors. Native cargo test --locked --workspace exited 0 with all 31 tests passing. The final image's --healthcheck and complete parameterized acceptance.py run against release-final-health on localhost:18096 passed: 48 malformed requests, browser fixtures/fragmentation, TLS refusal matrix, hardened/static/shell-free checks, deadlines, capacity and 1,000 logging probes (59 <= 60 lines). Public scanner exits 0 with zero hits. No x86_64, LAN, physical browser or independent acceptance rerun is claimed. Full mutant rerun remains pending. Evidence is in logs/release-final-*.log, logs/nix-proof-log.log, logs/nix-deny-log.log and logs/stage13-mutants/baseline-proofs.log.

## 2026-10-29T13:26:33-04:00 — Release final mutant checkpoint
The uninterrupted resumed runner has completed its clean baseline and killed the first five mutants. Every completed mutant passed refinement verification before intent/test rejection; none is malformed. Final aggregate result is still pending. Removed release-health and both superseded release image IDs, plus the throwaway negative-check clones. Only release-final-health and its current image remain for final cleanup; the main worktree is already removed.

## 2026-10-29T13:39:20-04:00 — Release final acceptance and cleanup
The complete resumed python3 scripts/spec-mutants.py exited 0: 15/15 killed, zero survivors, zero malformed/invalid runs. Every mutation passed all preexisting refinement proofs before its intent/test rejection; all fifteen failed actual workspace tests. The full native baseline ./verify.sh reports 211/237/9 verified and zero errors. Independent locked workspace tests pass all 31 tests. The final aarch64-linux Nix check, proof/dependency evidence, static image build, healthcheck and full container acceptance pass. Source audit confirms all 15 Rust/proof/runner files match 5b6fcc5 exactly, and all HTTP fixture diffs are exclusively the equal-length TEST-NET-3 substitution. LICENSE matches main and all three manifests declare MIT. Public scan is zero hits on both branches. No independent, x86_64, physical-browser or second-device acceptance is claimed.
All created release containers and the three loaded release image IDs were removed without force-removing images; no unrelated container or image was operated on. Removed the generated TLS key/certificate and directory, throwaway negative clones and main worktree. Docker release-name inventories are empty and git lists only the original worktree. Retained build logs and machine-readable mutant receipts under logs. No push, history rewrite, identity change or other-stage branch edit occurred. M4 commit follows; a cached clean-tree Nix check/image replay will additionally be recorded in logs/release-clean-tree-*.log, followed by removal of the reloaded image.

## 2026-10-29T14:46:28-04:00 — Release fix F1
Read the independent release acceptance report fully and retained PLAN-RELEASE as the governing spec. A real read-only store probe found libfaketime's library under lib/libfaketime.so.1, not lib/faketime/. Corrected the preload and asserted the exact frozen UTC date before cargo-deny; failure emits one clear clock-assertion line and exits nonzero. The archive and its hash, FETCH_HEAD timestamp and frozen clock timestamp must be refreshed together. The earlier M1 clock claim was false and is corrected above and in README.

The first future-clock experiment unexpectedly passed: a direct stat probe showed libfaketime also replaced FETCH_HEAD's metadata time. Added NO_FAKE_STAT=1 so the database timestamp remains real; DONT_FAKE_MONOTONIC=1 is retained. Final throwaway-clone experiments: FETCH_HEAD at 2026-10-14 12:00 UTC passes (exit 0); FAKETIME at 2026-11-11 10:02:42 fails (exit 1, "repository is stale", last update 2026-10-27T10:02:42Z); deliberately missing preload fails (exit 1, "advisory clock assertion failed: expected frozen UTC timestamp"). No staleness policy was relaxed.

Commands: in each throwaway clone under TMPDIR, run nix flake check --no-update-lock-file or nix build .#checks.aarch64-linux.dependencies --no-link --no-update-lock-file, then nix log .#checks.aarch64-linux.dependencies. The candidate flake check and ./nix-build.sh image-static both pass; clean committed-tree replay follows. Logs are retained at <tmp>/release-fix-logs/f1-*.log. A temporary Docker wrapper maps the runner name and Nix volume to release-fix names and rewrites only archive RepoTags before docker load; the tested nix-build.sh itself is unchanged. The isolated release-fix-nix-store was seeded from a read-only cache mount; no existing volume was written. Only release-fix containers/images/volumes are created.

Native ./verify.sh passes 211/237/9 verified, zero errors; cargo test --locked --workspace passes 31/31. Public scan passes on spec-lemmas and main using <notes>/public-denylist.txt; main needs no doc changes and its temporary worktree was removed. Proof/spec/test/mutant sources are untouched. Full mutant rerun is in progress. Existing untracked operator log is left alone. No push, history rewrite, identity change or other-stage branch edit.

## 2026-10-29T14:54:10-04:00 — Release fix F2
L1: moved historical first-party formal-coverage v1 annotations to coverage sentence starts and removed them from proof/test-count sentences. Rejoined only paragraphs damaged by the mechanical labels; all historical annotation lines contain a coverage figure. Restored the plan's original old-number examples and added one context note identifying their version/date, rather than rewriting its governing instructions. Fixed its garbled public-scan command to use <notes>/public-denylist.txt. The M2 filtered grep has no unlabelled historical figures outside the plan's own examples; main has no old-number matches. L2: repaired prose comma/semicolon/colon spacing in NOTES/PROGRESS; fenced-block byte comparison passes. Markdown searches used explicit paths.

L3: replaced the compressed codeload hash with fetchFromGitHub at the identical RustSec revision. A real fake-hash fetch reported NAR hash sha256-y2Y5TPInS05cpQWXH6wwfuJv8i6HG2kXMxOKcQko22w=; the subsequent dependency build verifies that hash. L4: renamed the image to scratch-server in the flake and README, and aligned the acceptance helper's default container and exact tag assertion. The assertion remains mandatory; no proof/spec/test/mutant obligation is removed or relaxed. L5: argparse now handles certificate-helper --help (exit 0, usage, no traceback); README supplies chown/chmod commands for uid 65532, notes Linux helper privileges and the need to wait for healthy status. The generated localhost certificate was exercised with 700/600 mount permissions and uid/gid 65532 ownership. Ignored the existing operator transcript without changing its contents.

F1's committed tree passes clean-clone nix flake check and ./nix-build.sh image-static with no dirty-tree warning or preload errors. F2 candidate flake check and image-static pass; all three clock experiments also repeat successfully with fetchFromGitHub: 13-day FETCH_HEAD passes, November 11 clock fails as stale, broken preload fails the clock assertion. Successful check/image/derivation logs contain no "cannot be preloaded" messages. All Linux runs use the isolated release-fix Nix volume, with build output redirected to files under <tmp>/release-fix-logs; successful derivation logs were retrieved separately.

The complete container acceptance passes (48 malformed requests, all browser fixtures, TLS, static/no-shell/hardening, routes, healthcheck, idle/slow/capacity, 1,000 probes with 59 lines within the 60-line bound). The first invocation ran before Docker marked the container healthy and was retried after observing healthy status. The image rename exposed the old literal expected tag in acceptance.py, which was corrected. For the fenced Docker run only, its exact expected tag was translated in memory to release-fix-scratch-server:static, matching the archive's test-only RepoTags translation. All assertions remain intact. The saved running image's config digest matches the F2 archive's config digest. No LAN, physical browser or x86_64 build is claimed. Full mutant rerun remains in progress before the F2 commit.

## 2026-10-29T15:08:12-04:00 — Release fix final validation
The complete python3 scripts/spec-mutants.py rerun exits 0: 15/15 killed, zero survivors and zero invalid runs. Every mutation passes the preexisting refinement proofs before intent/test rejection. Its fresh baseline again verifies 211/237/9 with zero errors and passes workspace tests; the separate cargo test --locked --workspace run passes 31/31. Protected Rust/proof/fixture/verifier/mutant files are byte-identical to the pre-fix tip. No assumptions, admissions, unsafe code or proof escape hatches were added. Certificate --help, actual certificate generation, Python syntax, documentation path/fence/coverage-label audits, diff whitespace checks and public scan all pass. Main has no required edits; its scan passes and no main commit is needed.

F2 is ready to commit. The final post-commit clean-clone check/image replay and removal of the isolated Nix volume and remaining test image are recorded in <tmp>/release-fix-logs/final-*.log. The runtime container, TLS files and superseded test images are already removed. Test-only archive/runner/tag adaptations preserve the published build and all acceptance assertions while honoring the release-fix Docker fence. No push, history rewrite, other-stage branch change, identity change or unrelated Docker resource operation occurred. These are local results for independent acceptance to recheck; no new independent verdict is claimed.


# Stages 14–16


## 2026-10-30 09:03 EDT — Stage 14 blocked: malformed diagnostic name spans

Read PROVEN-DISPATCH.md fully first, then PROVEN-TRANSPORT.md, HEADER-POLICY.md,
PROOFS.md and MUTANTS.md. PROGRESS.md and NOTES.md did not exist, so created them.
Branch remains `proven-dispatch`; no branch creation, push, deployment, container
operation, formal-coverage operation, or protected-file modification occurred.

The authoritative spec's property 3 requires every yielded name to contain no CR,
LF or colon. Its equivalence requirement simultaneously requires the same name
sequences as the old iterator for all generated malformed blocks, explicitly
including bare CR and LF. These cannot both hold. The existing walker splits on
CRLF only and takes every byte before the first colon, including bare CR or LF.

Minimal construction (no header value recorded here): a normal request line,
CRLF, the name bytes `X`, a bare CR or bare LF, `Name`, a colon, an otherwise valid
synthetic field value, CRLF, CRLF. The old walk yields the five-letter name with
the embedded line-break byte. With strict policy it produces exactly
`status=400 reason=unknown_header name=XName`. Skipping that invalid name instead
produces `status=400 reason=grammar`; truncating or filtering the span cannot
preserve the required name sequence either. The parser rejects both requests;
this is a diagnostic-walk contract bug, not evidence of parser acceptance or log
injection. The existing sanitizer removes the embedded line break.

Following the explicit stop/commit-evidence instruction, did not implement or
repair production behavior. Preserved the four requested implementations verbatim
in test-only src/dispatch_oracle.rs before any deletion (none was deleted).
Two regression tests reproduce both bare-CR and bare-LF counterexamples, compare
against the unchanged current implementation, check strict diagnostic output and
check parser rejection. A third test checks the preserved sanitizer on all 256
byte values and compares both existing limiters for four limits (4,008 calls).
Assertions deliberately omit raw input payloads from failure messages.

Suggested spec resolution, requiring Andrew's decision: permit arbitrary bytes
other than colon in diagnostic name spans, and keep the strict no-control-byte
property on sanitizer output. This preserves all runtime behavior. Alternatively,
authorize a diagnostic behavior change and relax malformed-name equivalence.
No spec, proof, existing test, or existing behavior was weakened here.

### Function inventory at blocker

No production function moved or added; additional proven runtime SLoC: **0**.
All original src/lib.rs functions remain in the unverified shell with identical
source to 5b880fb (apart from a cfg(test) module declaration). No app.rs exists.
SLoC below counts nonblank, noncomment source lines, including braces, matching
these four function counts in PROVEN-DISPATCH.md; it is not a coverage measurement.

| Function | Old location | Added location | Status | SLoC |
| --- | --- | --- | --- | ---: |
| UnknownLog::should_log | src/lib.rs:41 | src/dispatch_oracle.rs:10 | test-only oracle; production shell unchanged | 15 |
| sanitized | src/lib.rs:57 | src/dispatch_oracle.rs:26 | test-only oracle; production shell unchanged | 8 |
| header_names | src/lib.rs:66 | src/dispatch_oracle.rs:35 | test-only oracle; production shell unchanged | 20 |
| rejection_line | src/lib.rs:139 | src/dispatch_oracle.rs:55 | test-only oracle; production shell unchanged | 31 |
| stage14_blocker_header_spans_contain_bare_line_breaks | new | src/dispatch_oracle.rs:89 | test only, not proven | 11 |
| stage14_blocker_malformed_name_changes_rejection_diagnostic | new | src/dispatch_oracle.rs:103 | test only, not proven | 13 |
| stage14_preserved_oracle_sanitizer_and_limiter | new | src/dispatch_oracle.rs:119 | test only, not proven | 19 |

Full 100,000-case new equivalence suite, random names, all 15 spec mutants, Nix
checks/image, and live checks on 8096 are not run because implementation stopped
at the mandatory blocker. No image/container was built or replaced. Historical
stage-13 results are not claimed as stage-14 results. Existing PROOFS.md remains
unchanged because there are no new proven properties.

## 2026-10-30 09:04 EDT — Blocker tests and baseline workspace validation

`CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" cargo test
--workspace` passed (exit 0): all 31 existing tests plus 3 new evidence/oracle
tests, **34 passed, 0 failed**. The inherited 100,000 differential cases remain
in this run. New equivalence evidence comprises 2 malformed blocks checked for
both name sequence and strict diagnostic/parser result, 256 single-byte sanitizer
inputs, and 4,008 limiter calls across limits 0, 1, 60, 600. These compare the
preserved old code with unchanged production code, not a new proven core.
Receipt: logs/stage14-tests.log. First-party Rust prohibited-token scan across
tracked src/crates/tests and the new oracle: zero matches (receipt
logs/stage14-prohibited-scan.log); no banned construct was added.

## 2026-10-30 09:04 EDT — Baseline proofs pass; commit blocker evidence

`./verify.sh` completed with exit 0: **211 verified / 0 errors** parser,
**237 / 0** transport, **9 / 0** standalone AEAD. Counts overlap because the
transport root includes parser and AEAD source. Receipt: logs/stage14-verify.log.
These are unchanged baseline proofs, not completion of any stage-14 property.
`git diff --check` passed. The new universal CR/LF-free span property cannot be
proven while preserving these old iterator sequences; implementation stopped
under PROVEN-DISPATCH.md's mandatory bug/equivalence stop rule. Spec-mutant result
this run: **not run / 15**, live 8096 checks: **not run**, Nix: **not run**.
No private key or header value was printed or retained in these notes. The
pre-existing untracked codex-run-stage14.log is left untouched and uncommitted.
No external diary write was made because this task forbids writes outside this
repository (except TMPDIR and Docker); this NOTES.md retains the audit record.

## 2026-10-30 09:17 EDT — Resolution A and core implementation

Resumed after Andrew approved amendment 0cb7016: diagnostic names exclude colon
and CRLF pairs but preserve bare CR/LF. Retained the original evidence tests,
renamed them compatibility tests and updated their comments. The historical
blocker entries above are retained as an audit trail and are resolved.

Pure decisions live in crates/transport-core/src/app.rs under the existing root.
The module uses its own verus! block, like aead.rs; the declaration is at module
scope, so no tooling or coverage configuration change is required. An arbitrary
cursor causes the name walker to rescan from the request line. This prevents a
caller from skipping the first empty line and preserves exact old iteration for
normal cursors. At most 25 names are needed for rejection diagnosis; production
accepted requests have at most 24 names. The small rescans are a deliberate
simplicity tradeoff.

Limiter state has a fixed limit for its lifetime, matching ServerState's immutable
policy; valid(limit) carries that invariant. Timestamp rollback retains future
entries, matching Instant::duration_since's saturating zero age. Comparison occurs
before subtraction, so no underflow. Raw ASCII lowercase storage matches the old
String implementation for production names, which are parser-validated ASCII
tokens; non-ASCII limiter names cannot occur on the accepted-request logging path.
Sanitizer and diagnostic equivalence still cover arbitrary byte inputs.

## 2026-10-30 09:20 EDT — Core checkpoint verified

`./verify.sh` passed: parser **211/0**, transport **285/0**, standalone AEAD
**9/0** (overlapping roots). The transport root adds 48 verification results,
including an induction over arbitrarily long limiter traces proving every
half-open 60,000 ms window contains at most min(limit,64) successful log calls.
The unknown-line and response formatter consume the same route_status function;
status_agrees connects its numeric log code to the fixed wire status line.
Plan carries exactly one handler increment for Dispatch and zero for every other
action. task_response has no bytes for ReadMore/CloseSilently/Done and a fixed
400/non-HEAD/bad-request body for every rejection, independent of diagnostics.
Receipt: logs/stage14-core-verify.log. No earlier proof or contract was weakened.

## 2026-10-30 09:22 EDT — Shell rewired and equivalence checkpoint

src/lib.rs now owns only IO/state construction, socket event mapping, the clock,
mutex, stdout, dynamic filesystem page body, atomic effect and response writes.
Routing, log filtering/formatting, rejection reasons, response/body/status and the
post-machine counter delta are delegated to app.rs. The counter effect remains
after unknown-header logging and before root-body IO, matching the old ordering.
The rejection stdout error remains ignored before writing the fixed response.
No TLS, main, transport, parser or policy source was changed.

As required by the spec, the shell converts elapsed Instant time since state
creation to u64 milliseconds (saturating at u64::MAX). The core's 60,000 ms window
uses that requested clock resolution. Equivalence time tests use exact millisecond
instants, including the 59,000/60,000 boundary; sub-millisecond scheduling is not
modeled by the requested millisecond API. Wall-clock delivery remains trusted.

The first new equivalence run found a test-loader error: malformed.hex uses
space-separated octets. Corrected the loader to split whitespace. It did not
expose an old/new implementation difference and required no behavior change.
The corrected 100,000 generated blocks give **0 name-sequence differences** and
**0 diagnostic differences across 300,000 comparisons** (builtin, strict and
all-guarded-drop policies). All required malformed categories are generated.
Sanitizer: 256 byte values plus 10,000 random names up to 300 bytes, 0 differences.
Limiter: 4,008 inherited/boundary comparisons plus 80,000 generated sliding-window
comparisons, 0 differences; additional rollback and u64::MAX checks pass.
Receipts: logs/stage14-equivalence-debug.log and logs/stage14-tests-final.log.

## 2026-10-30 09:22 EDT — Full workspace acceptance

`cargo test --workspace` (repo-local pinned Cargo/Rustup homes) passed: **39 tests,
0 failures**, including all 31 inherited tests unchanged in meaning, the retained
3 evidence/oracle tests, 3 expanded equivalence tests and 2 response-plan/shell
routing tests. Existing limiter test interface changed from Instant offsets to
equivalent u64 millisecond offsets; its limits, counts and boundary assertions
are unchanged. New integration tests exercise GET/HEAD headers and handler counts
for all three routes, all no-response actions and every rejection reason.
Full new equivalence counts are recorded in the previous entry. No runtime bug
was found by the equivalence tests. No claim of independent acceptance is made.

## 2026-10-30 09:27 EDT — Acceptance campaigns and diagnostic span inventory

Started the existing spec-mutant runner and Nix check. Their top-level output
is redirected to stage14-prefixed logs; their internal receipt paths remain the
existing tooling defaults. The required nix-build.sh entry point uses a transient
release-nix build container and the existing Nix cache volume; the only runtime
container created/replaced for this stage is proven-dispatch on 8096. Protected
container identities/pids/start times were recorded read-only before deployment.

TooManyHeaders also carries the 25th name span, as specified, while its line
remains the fixed name-free prefix. This strengthens diagnostic span validity
without changing any output. Focused re-verification: 48/0. Final full verification
is running against that source; all runtime Rust is now held fixed during the
mutant campaign.

## 2026-10-30 09:30 EDT — Final proof acceptance

Final ./verify.sh passed with exit 0: parser **211/0**, transport **285/0**,
standalone AEAD **9/0**, including the final diagnostic span representation.
Receipt: logs/stage14-verify-final.log. PROOFS.md lists every property with
file/line references and the remaining unverified shell. Repeated first-party
Rust prohibited-token scan: **zero matches** (logs/stage14-prohibited-scan.log).
No earlier executable parser/policy/grammar, proof or test was weakened.

## 2026-10-30 09:35 EDT — Deployment checkpoint and live acceptance

Final ./nix-build.sh check and ./nix-build.sh image-static passed (exit 0).
Native and aarch64-linux Nix verification agree: **211/0, 285/0, 9/0**. Nix
workspace tests: **39/0**. Receipts: logs/stage14-nix-final-check.log,
logs/stage14-nix-image.log, logs/stage14-nix-proof-summary.log and
logs/stage14-nix-test-summary.log (full tool-internal logs remain under logs/nix-*).
Only aarch64-linux was exercised; x86_64-linux was not run.

Deployed container proven-dispatch on **8096**, image sha256:609e691b43f05504928a20ca0e0a04d0dc89709e52685cbde95584dbb2271107
Image size **3,104,699 bytes**, ELF **1,529,848 bytes**. Export/layer
inspection finds exactly the application binary: no dynamic interpreter or
DT_NEEDED, no shell and no private-key file. User 65532:65532, read-only rootfs,
cap-drop ALL, no-new-privileges, read-only TLS and policy mounts all pass. The
existing TLS directory is reused solely through a read-only Docker bind mount;
no host TLS file was written, copied or logged.

python3 scripts/stage14-live.py passed using the existing acceptance.py checks:
TLS 1.3/ChaCha20/X25519/ECDSA success, TLS1.2/AES/P-256-key-exchange/h2 refusals,
plaintext refusal, curl, root/health/404, healthcheck, delayed
3-second request 200, partial request 400 and 17-second idle 0 bytes, capacity
limit, **48 malformed requests**, **32 TLS fixture replays** (8 fixtures x 4
fragmentations), four browser curl replays, duplicate/header-cap/privacy tests.
The 1,000-request/100-name log probe emits **59 new lines**, within the remaining
default 60-line sliding budget after the earlier DNT line. All pass.

Additional **14 rejected-request cases** against header-policy on 8095 and
proven-dispatch on 8096 produce byte-identical complete response buffers and
stdout suffixes, including bare CR/LF names and diagnostic precedence. The
comparison stores only diagnostic names/reasons, never header values. Receipts:
logs/stage14-live.log, logs/stage14-live/stage12-acceptance.json and
logs/stage14-live/rejection-equivalence.json. Full acceptance artifacts are
namespaced under logs/stage14-live. No physical-browser or separate LAN-device
check was performed; browser fixture replays are the required empirical checks.

Protected-container identity/pid/start-time snapshots compare equal with cmp:
scratch-server, nix-scratch, parser-http, parser-https and header-policy were not
restarted, stopped, removed or reconfigured. The only requests sent to 8095 were
the explicitly required rejection comparisons. No push or formal-coverage run.
The mutant campaign continues in temporary copies with runtime Rust held fixed.

## 2026-10-30 09:49 EDT — Stage 14 acceptance complete

The unchanged python3 scripts/spec-mutants.py completed with exit 0: **15/15
killed, 0 survivors, 0 invalid**. Every mutant passed the pre-intent refinement
phase, now including the application proofs. Thirteen failed focused intent
lemmas; all fifteen failed real workspace tests. The runner's working-tree
Rust/config/fixture hash checks passed. Original internal receipts are under
logs/stage13-mutants; archived stage-14 receipts are logs/stage14-mutants with
results.json and per-mutant logs. Top-level log: logs/stage14-mutants.log.
MUTANTS.md records this revalidation without replacing historical evidence.

All acceptance checkboxes are complete. Final counts: **211/285/9 verified
with zero errors** per overlapping root, **39 workspace tests**, **15/15 killed**.
Equivalence: **100,000 header sequences**, **300,000 diagnostic comparisons**,
**256 byte values + 10,000 random sanitizer names**, **84,008 paired limiter
calls**, plus rollback/max-timestamp cases. The live 8096 results and complete
function/SLoC tables above are part of this final audit record.

Final protected-container snapshot again matches the pre-deployment snapshot
exactly, and proven-dispatch is healthy. The prohibited-token Rust scan still
has zero matches. Scope diff against 0cb7016 is empty for TLS, main, transport,
parser/policy, EVOLUTION.md and ATTACK-SURFACE.md. No push, branch switch or
formal-coverage invocation occurred. Commit identity remains repository-configured
Dudley. All writes remain in this repository or Docker; no external diary write.

Unverified by design: physical socket/stdout/atomic/mutex effects and their
wiring, Instant sampling and requested millisecond conversion, filesystem page
content, TLS adapters except inherited AEAD helpers, upstream crypto/OS/toolchain
and allocation success. Verus models the pure decisions, exact bytes and
transition contracts; it has no model of those OS/upstream effects. These were
tested where listed, not claimed proven. There is no new formal-coverage result
or percentage: Dudley runs that independently. Physical browsers, a separate LAN
device and x86_64 were not exercised. The stage is ready for Dudley's independent
acceptance; these are Codex's executed checks, not an independent signoff.

## 2026-10-30 09:51 EDT — Final function/SLoC inventory

Counts are physical nonblank, noncomment source lines. Runtime counts exclude
ghost contracts, loop specifications, proof blocks and assertions; inline code
counts once per physical line. Ghost-only functions have zero runtime SLoC and
a separate source count. This is a local source inventory, not formal-coverage;
Dudley supplies the compiled-files measurement. Old locations refer to 5b880fb.


### crates/transport-core/src/app.rs

| Function | Old location/origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `same` | new | crates/transport-core/src/app.rs:9 | proven core | 11 | 0 |
| `route` | src/lib.rs:108 | crates/transport-core/src/app.rs:25 | proven core | 5 | 0 |
| `route_status_spec` | new | crates/transport-core/src/app.rs:36 | proven ghost | 0 | 3 |
| `route_status` | src/lib.rs:108 | crates/transport-core/src/app.rs:39 | proven core | 3 | 0 |
| `code` | new | crates/transport-core/src/app.rs:42 | proven ghost | 0 | 3 |
| `status_code` | src/lib.rs:193 | crates/transport-core/src/app.rs:45 | proven core | 3 | 0 |
| `status_agrees` | new | crates/transport-core/src/app.rs:49 | proven ghost | 0 | 5 |
| `plan` | src/lib.rs:193 | crates/transport-core/src/app.rs:57 | proven core | 11 | 0 |
| `task_response` | src/lib.rs:108 | crates/transport-core/src/app.rs:77 | proven core | 13 | 0 |
| `safe` | new | crates/transport-core/src/app.rs:117 | proven ghost | 0 | 3 |
| `is_safe` | new | crates/transport-core/src/app.rs:120 | proven core | 3 | 0 |
| `filtered` | new | crates/transport-core/src/app.rs:124 | proven ghost | 0 | 6 |
| `sanitize_spec` | new | crates/transport-core/src/app.rs:130 | proven ghost | 0 | 3 |
| `sanitize` | src/lib.rs:57 | crates/transport-core/src/app.rs:133 | proven core | 11 | 0 |
| `crlf` | new | crates/transport-core/src/app.rs:155 | proven ghost | 0 | 3 |
| `line_end` | new | crates/transport-core/src/app.rs:158 | proven ghost | 0 | 6 |
| `scan_line` | new | crates/transport-core/src/app.rs:164 | proven core | 10 | 0 |
| `colon_end` | new | crates/transport-core/src/app.rs:181 | proven ghost | 0 | 6 |
| `scan_colon` | new | crates/transport-core/src/app.rs:187 | proven core | 10 | 0 |
| `stop` | new | crates/transport-core/src/app.rs:206 | proven ghost | 0 | 7 |
| `line_bounds` | new | crates/transport-core/src/app.rs:213 | proven ghost | 0 | 8 |
| `stop_bounds` | new | crates/transport-core/src/app.rs:221 | proven ghost | 0 | 9 |
| `next_from` | new | crates/transport-core/src/app.rs:230 | proven ghost | 0 | 12 |
| `next_spec` | new | crates/transport-core/src/app.rs:242 | proven ghost | 0 | 4 |
| `next_header_name` | src/lib.rs:66 | crates/transport-core/src/app.rs:248 | proven core | 15 | 0 |
| `diagnostic_valid` | new | crates/transport-core/src/app.rs:284 | proven ghost | 0 | 7 |
| `decision` | new | crates/transport-core/src/app.rs:291 | proven ghost | 0 | 9 |
| `diagnose_from` | new | crates/transport-core/src/app.rs:300 | proven ghost | 0 | 17 |
| `diagnose` | src/lib.rs:139 | crates/transport-core/src/app.rs:317 | proven core | 26 | 0 |
| `prefix` | new | crates/transport-core/src/app.rs:348 | proven ghost | 0 | 10 |
| `name_span` | new | crates/transport-core/src/app.rs:358 | proven ghost | 0 | 7 |
| `diagnostic_spec` | new | crates/transport-core/src/app.rs:365 | proven ghost | 0 | 5 |
| `diagnostic_line` | src/lib.rs:139 | crates/transport-core/src/app.rs:370 | proven core | 22 | 0 |
| `folded` | new | crates/transport-core/src/app.rs:406 | proven ghost | 0 | 3 |
| `lowercase` | new | crates/transport-core/src/app.rs:409 | proven core | 12 | 0 |
| `live` | new | crates/transport-core/src/app.rs:428 | proven ghost | 0 | 3 |
| `retained` | new | crates/transport-core/src/app.rs:431 | proven ghost | 0 | 6 |
| `prune` | new | crates/transport-core/src/app.rs:437 | proven core | 7 | 0 |
| `cap` | new | crates/transport-core/src/app.rs:450 | proven ghost | 0 | 1 |
| `present` | new | crates/transport-core/src/app.rs:451 | proven ghost | 0 | 3 |
| `UnknownLog::valid` | new | crates/transport-core/src/app.rs:456 | proven ghost | 0 | 5 |
| `UnknownLog::new` | new | crates/transport-core/src/app.rs:461 | proven core | 2 | 0 |
| `UnknownLog::should_log` | src/lib.rs:41 | crates/transport-core/src/app.rs:464 | proven core | 15 | 0 |
| `needs_root` | src/lib.rs:108 | crates/transport-core/src/app.rs:494 | proven core | 3 | 0 |
| `rejection_line` | src/lib.rs:171 | crates/transport-core/src/app.rs:497 | proven core | 8 | 0 |
| `next_unknown` | src/lib.rs:86 | crates/transport-core/src/app.rs:510 | proven core | 14 | 0 |
| `unknown_line` | src/lib.rs:86 | crates/transport-core/src/app.rs:531 | proven core | 12 | 0 |
| `recent` | new | crates/transport-core/src/app.rs:555 | proven ghost | 0 | 7 |
| `recent_bounds` | new | crates/transport-core/src/app.rs:562 | proven ghost | 0 | 6 |
| `retain_recent` | new | crates/transport-core/src/app.rs:569 | proven ghost | 0 | 12 |
| `emitted_count` | new | crates/transport-core/src/app.rs:581 | proven ghost | 0 | 5 |
| `log_transition` | new | crates/transport-core/src/app.rs:586 | proven ghost | 0 | 7 |
| `window_accounting` | new | crates/transport-core/src/app.rs:593 | proven ghost | 0 | 15 |
| `sliding_window_bound` | new | crates/transport-core/src/app.rs:611 | proven ghost | 0 | 12 |


### src/lib.rs

| Function | Old location/origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `ServerState::default` | src/lib.rs:23 | src/lib.rs:25 | unverified shell | 3 | 0 |
| `ServerState::with_policy` | src/lib.rs:28 | src/lib.rs:30 | unverified shell | 8 | 0 |
| `log_unknown` | src/lib.rs:86 | src/lib.rs:39 | unverified shell | 17 | 0 |
| `reply` | src/lib.rs:101 | src/lib.rs:56 | unverified shell | 6 | 0 |
| `root_body` | src/lib.rs:108 | src/lib.rs:62 | unverified shell | 16 | 0 |
| `serve_connection` | src/lib.rs:189 | src/lib.rs:80 | unverified shell | 3 | 0 |
| `serve_with_started` | src/lib.rs:193 | src/lib.rs:83 | unverified shell | 57 | 0 |
| `sanitized` | src/lib.rs:57 | src/lib.rs:143 | test adapter | 3 | 0 |
| `header_names` | src/lib.rs:66 | src/lib.rs:147 | test adapter | 8 | 0 |
| `rejection_line` | src/lib.rs:139 | src/lib.rs:156 | test adapter | 3 | 0 |
| `unknown_log_is_bounded_in_every_sliding_minute` | src/lib.rs:255 | src/lib.rs:164 | test only | 16 | 0 |
| `diagnostic_classes_and_privacy` | src/lib.rs:274 | src/lib.rs:181 | test only | 42 | 0 |


### src/dispatch_oracle.rs

| Function | Old location/origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `should_log` | src/lib.rs:41 | src/dispatch_oracle.rs:12 | test only | 15 | 0 |
| `sanitized` | src/lib.rs:57 | src/dispatch_oracle.rs:28 | test only | 8 | 0 |
| `header_names` | src/lib.rs:66 | src/dispatch_oracle.rs:37 | test only | 20 | 0 |
| `rejection_line` | src/lib.rs:139 | src/dispatch_oracle.rs:57 | test only | 31 | 0 |
| `stage14_compatibility_header_spans_contain_bare_line_breaks` | new | src/dispatch_oracle.rs:90 | test only | 11 | 0 |
| `stage14_compatibility_malformed_name_changes_rejection_diagnostic` | new | src/dispatch_oracle.rs:104 | test only | 17 | 0 |
| `stage14_preserved_oracle_sanitizer_and_limiter` | new | src/dispatch_oracle.rs:124 | test only | 23 | 0 |
| `next_random` | new | src/dispatch_oracle.rs:148 | test only | 6 | 0 |
| `stage14_header_and_diagnostic_equivalence_100000_blocks` | new | src/dispatch_oracle.rs:156 | test only | 100 | 0 |
| `stage14_sanitizer_equivalence_random_names` | new | src/dispatch_oracle.rs:258 | test only | 14 | 0 |
| `stage14_limiter_equivalence_sliding_windows` | new | src/dispatch_oracle.rs:274 | test only | 48 | 0 |


### tests/dispatch.rs

| Function | Old location/origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `read` | new | tests/dispatch.rs:17 | test only | 3 | 0 |
| `write` | new | tests/dispatch.rs:22 | test only | 4 | 0 |
| `flush` | new | tests/dispatch.rs:26 | test only | 3 | 0 |
| `shell_routes_get_and_head_with_exactly_one_handler_call` | new | tests/dispatch.rs:31 | test only | 34 | 0 |
| `plans_never_respond_or_increment_except_as_specified` | new | tests/dispatch.rs:66 | test only | 33 | 0 |


Executable function-body/signature inventory totals: **216 proven core SLoC**,
**110 unverified shell SLoC** in src/lib.rs, excluding test adapters,
types and imports. There were no prior app.rs functions. Pure dispatch/reject
logic now lives in task_response; only dynamic root-body IO stays in root_body.
The old production UnknownLog, sanitizer, diagnostic iterator and rejection_line
implementations are removed from the shell and retained only as test oracles.
Ghost helpers carry no shipped runtime SLoC. These source totals do not assert
a formal-coverage numerator, denominator or percentage.

## 2026-10-30 09:51 EDT — Inventory audit correction

Final review corrected the local counting script to recognize pub(crate),
multiline function signatures and the cfg(test) boundary. The original inventory
misclassified two tests as shell functions and omitted serve_with_started. The
corrected tables above include every function and report **216 proven runtime
core SLoC, 110 unverified runtime shell SLoC** (function signatures/bodies only).
The proven-core count is unchanged. The superseded draft is retained under
logs/stage14-inventory-first-draft.md for audit. Runtime source, proof counts,
workspace/Nix results, mutant receipts and deployed image are unchanged.

## 2026-10-30 11:02 EDT — Stage 15 started

Read the authoritative PROVEN-RECORD.md first and all required context. Branch proven-record, initial working tree clean. Module declaration follows aead.rs/app.rs at Rust module scope with its own verus! block. Only permitted behavior change: encrypted_payload_len saturates above usize::MAX - 17; production rustls lengths are bounded to 16,384. No external diary write because this task confines writes to this repository, TMPDIR and Docker.

## 2026-10-30 11:06 EDT — Stage 15 function/SLoC inventory

Physical nonblank/noncomment source lines, including signatures and braces. Runtime SLoC excludes ghost contracts, loop specifications, proof blocks and assertions; ghost-only functions have zero runtime SLoC. This is a source inventory, not formal-coverage. Old source is 13c8124. Repeated method names are numbered in source order. All TLS production functions remain unverified shell; their calls into the core do not prove the surrounding trait wiring.

| TLS function | Old line | New line | Before SLoC | After SLoC | Status |
| --- | ---: | ---: | ---: | ---: | --- |
| `provider#1` | 21 | 21 | 12 | 12 | shell |
| `fill#1` | 48 | 48 | 3 | 3 | shell |
| `try_next_u32#1` | 54 | 54 | 5 | 5 | shell |
| `try_next_u64#1` | 59 | 59 | 5 | 5 | shell |
| `try_fill_bytes#1` | 64 | 64 | 3 | 3 | shell |
| `start#1` | 74 | 74 | 3 | 3 | shell |
| `hash#1` | 77 | 77 | 3 | 3 | shell |
| `output_len#1` | 80 | 80 | 3 | 3 | shell |
| `algorithm#1` | 83 | 83 | 3 | 3 | shell |
| `fork_finish#1` | 88 | 88 | 5 | 5 | shell |
| `fork#1` | 93 | 93 | 3 | 3 | shell |
| `finish#1` | 96 | 96 | 3 | 3 | shell |
| `update#1` | 99 | 99 | 3 | 3 | shell |
| `extract_from_zero_ikm#1` | 107 | 107 | 3 | 3 | shell |
| `extract_from_secret#1` | 110 | 110 | 11 | 11 | shell |
| `expander_for_okm#1` | 122 | 122 | 3 | 3 | shell |
| `hmac_sign#1` | 125 | 125 | 5 | 5 | shell |
| `expand_slice#1` | 132 | 132 | 9 | 9 | shell |
| `expand_block#1` | 141 | 141 | 6 | 6 | shell |
| `hash_len#1` | 147 | 147 | 3 | 3 | shell |
| `start#2` | 159 | 159 | 8 | 8 | shell |
| `name#1` | 167 | 167 | 3 | 3 | shell |
| `complete#1` | 172 | 172 | 7 | 7 | shell |
| `pub_key#1` | 179 | 179 | 3 | 3 | shell |
| `group#1` | 182 | 182 | 3 | 3 | shell |
| `encrypter#1` | 193 | 193 | 3 | 3 | shell |
| `decrypter#1` | 196 | 196 | 3 | 3 | shell |
| `key_len#1` | 199 | 199 | 3 | 3 | shell |
| `extract_keys#1` | 202 | 202 | 7 | 7 | shell |
| `encrypt#1` | 211 | 211 | 31 | 33 | shell |
| `encrypted_payload_len#1` | 242 | 244 | 3 | 3 | shell |
| `decrypt#1` | 247 | 249 | 26 | 26 | shell |
| `unpad_message#1` | new | 279 | 0 | 10 | shell |
| `public_key_alg_id#1` | 279 | 293 | 3 | 3 | shell |
| `signature_alg_id#1` | 282 | 296 | 3 | 3 | shell |
| `verify_signature#1` | 285 | 299 | 21 | 21 | shell |
| `fmt#1` | 316 | 330 | 3 | 3 | shell |
| `load_private_key#1` | 321 | 335 | 31 | 31 | shell |
| `fmt#2` | 355 | 369 | 3 | 3 | shell |
| `choose_scheme#1` | 360 | 374 | 5 | 5 | shell |
| `algorithm#2` | 365 | 379 | 3 | 3 | shell |
| `public_key#1` | 368 | 382 | 3 | 3 | shell |
| `scheme#1` | 373 | 387 | 3 | 3 | shell |
| `sign#1` | 376 | 390 | 13 | 13 | shell |
| `load_config#1` | 391 | 405 | 8 | 8 | shell |
| `server_config#1` | 400 | 414 | 12 | 12 | shell |
| `der_bytes#1` | 430 | 444 | 3 | 3 | test only |
| `algorithm#3` | 433 | 447 | 3 | 3 | test only |
| `sign#2` | 438 | 452 | 8 | 8 | test only |
| `configs#1` | 447 | 461 | 37 | 37 | test only |
| `exchange#1` | 484 | 498 | 18 | 18 | test only |
| `exchange_policy#1` | 502 | 516 | 55 | 55 | test only |
| `idle_and_partial_deadlines_tcp_and_tls#1` | 558 | 572 | 89 | 89 | test only |
| `idle_handshake_budget_is_absolute#1` | 655 | 669 | 39 | 39 | test only |
| `tls_routes_and_safari#1` | 697 | 711 | 51 | 51 | test only |
| `default_policy_browser_fixtures_and_sensitive_names#1` | 749 | 763 | 87 | 87 | test only |
| `tls_case_and_count_boundaries#1` | 837 | 851 | 70 | 70 | test only |
| `malformed_tls_never_dispatches#1` | 908 | 922 | 59 | 59 | test only |
| `aead_rejects_tampered_record_and_x25519_rejects_low_order#1` | 969 | 983 | 16 | 16 | test only |
| `hkdf_rfc5869_case1_and_hash_fork#1` | 986 | 1000 | 30 | 30 | test only |

| Core function | Origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `all_zero` | new | record.rs:10 | proven ghost | 0 | 3 |
| `unpad` | rustls into_tls13_unpadded_message via src/tls.rs:272 | record.rs:13 | proven core | 16 | 0 |
| `aad` | rustls make_tls13_aad via src/tls.rs:230,263 | record.rs:46 | proven core | 6 | 0 |
| `inner_plaintext` | src/tls.rs:217–219 | record.rs:57 | proven core | 10 | 0 |
| `encrypted_payload_len` | src/tls.rs:242 | record.rs:75 | proven core | 4 | 0 |
| `x25519_peer` | src/tls.rs:174 | record.rs:81 | proven core | 9 | 0 |
| `p256_point_shape` | src/tls.rs:294 | record.rs:95 | proven core | 2 | 0 |
| `offers` | src/tls.rs:360 | record.rs:98 | proven core | 10 | 0 |

New shell helper `unpad_message` is included in the TLS table. It maps core errors, sets rustls type/version and slices through the public into_plain_message_range API. No upstream private API is used.

| Added test function | Location | Status | SLoC |
| --- | --- | --- | ---: |
| `random` | src/record_tests.rs:4 | test only | 6 |
| `compare_unpad` | src/record_tests.rs:11 | test only | 13 |
| `stage15_unpad_equivalence_100000` | src/record_tests.rs:26 | test only | 16 |
| `stage15_aad_equivalence_80001` | src/record_tests.rs:44 | test only | 10 |
| `stage15_inner_and_length_equivalence` | src/record_tests.rs:56 | test only | 33 |
| `stage15_key_shape_and_offers_equivalence` | src/record_tests.rs:92 | test only | 22 |

Seven new executable core functions total **57 runtime SLoC**; all_zero is ghost-only. The extra shell payload gather is needed because rustls OutboundChunks is not a plain slice. Both gathered payload and constructed inner plaintext are Zeroizing-owned; allocation success and temporary Vec ownership transfer are trusted. All existing production TLS functions remain in place. No compiled-files coverage percentage is claimed.

## 2026-10-30 11:06 EDT — Stage 15 core checkpoint

./verify.sh passed: parser 211/0, transport 298/0, standalone AEAD 9/0 (overlapping roots). New record module adds 13 results. Receipt logs/stage15-core-verify.log. Focused record verification also passes 13/0. The initial bit-vector assertion needed to state the expression directly rather than refer to local aliases; no contract was weakened. PROOFS.md records all six properties and remaining shell limitations. Runtime source inventory: seven proven helpers, 57 SLoC.

## 2026-10-30 11:07 EDT — Stage 15 shell checkpoint

TLS shell delegates unpadding/AAD/inner construction/length/key-share shape/point shape/scheme membership to record.rs. Errors are mapped to the identical rustls variants and version TLSv1_3; into_plain_message_range borrows the original payload. libcrux calls and all-zero ECDH rejection are retained. All four new equivalence tests passed; full inherited workspace suite is still completing. Final full verifier receipt logs/stage15-core-verify-final.log confirms 211/298/9, zero errors.

## 2026-10-30 11:08 EDT — Stage 15 equivalence and workspace acceptance

Full cargo test --workspace passed, 43 tests/0 failures: all 39 inherited tests unchanged in meaning, plus four new tests. New equivalence counts: 100,000 unpadding cases (including all types, zero buffers, CCS with/without padding, required boundary lengths, long zero runs and random bytes); 80,001 AAD cases (70,001 consecutive lengths plus 10,000 random usize); 10,000 inner constructions and actual encrypted records, covering every content type, empty/full/random payloads and fragmented OutboundChunks; 20,019 length checks (10,000 bounded payload lengths, 10,000 random representable lengths, 19 near-MAX boundary/saturation cases); 25,856 peer copies and 25,856 point-shape comparisons (lengths 0–100 x every prefix); 10,000 scheme lists plus 10,000 arbitrary-wanted membership checks. Zero oracle differences. Receipt logs/stage15-tests.log; assertions do not dump record bytes.

Final verifier 211/298/9, zero errors; first-party tracked Rust scan for all five prohibited tokens: zero matches (logs/stage15-prohibited-scan.log). Started unchanged spec-mutant runner in temporary copies and ./nix-build.sh check; runtime Rust is held fixed during the campaign. The prescribed Nix wrapper uses transient release-nix build containers and existing Nix cache volume. Only runtime container created/replaced will be proven-record on 8097.

## 2026-10-30 11:14 EDT — Stage 15 deployment and live acceptance

./nix-build.sh check and ./nix-build.sh image-static passed, exit 0. aarch64-linux Verus counts match native: 211/298/9, zero errors; 43 Nix workspace tests pass. Only aarch64-linux exercised, not x86_64. Receipts logs/stage15-nix-check.log, stage15-nix-image.log, stage15-nix-proof-full.log and stage15-nix-test-full.log.

Deployed proven-record on 8097: image sha256:234a8b3036b68ca1b784e4d8e62576aaa21115522d6e5f5d2012d911ddd3ee89, image 3,104,699 bytes, ELF 1,529,560 bytes. Existing TLS directory is bound read-only, never copied or written. User 65532:65532, read-only rootfs, cap-drop ALL, no-new-privileges and read-only policy mount verified. Export and saved-image layer inspection find only the application binary, no shell/key, no ELF interpreter or DT_NEEDED.

python3 scripts/stage15-live.py passed, reusing acceptance.py unchanged: TLS1.3/ChaCha20/X25519/ECDSA P-256 success; TLS1.2/AES/P-256 key exchange/h2 and plaintext HTTP refused; curl/routes/healthcheck pass; delayed 3-second request 200; partial timeout 400; 17-second idle closes with 0 bytes. All 48 malformed requests, 32 TLS browser fixture replays (8 x 4 fragmentation sizes), four curl browser replays, capacity and privacy checks pass. The 1,000-request/100-name log probe produces 59 new lines within the remaining 60-line sliding budget. Eighteen scripted requests (root/health/404/HEAD plus 14 rejections) give byte-identical full responses and stdout suffixes against proven-dispatch on 8096. No physical browser or separate LAN device check performed. Receipts logs/stage15-live.log and logs/stage15-live/*.json.

Protected-container identity/pid/start-time snapshots compare equal with cmp: scratch-server, nix-scratch, parser-http, parser-https, header-policy, proven-dispatch. Only proven-record was created. No push, branch switch, formal-coverage invocation or modification of EVOLUTION.md/ATTACK-SURFACE.md. Mutant campaign remains active; first mutant killed, remaining results pending.

The added live-runner functions repeat stage 14 tooling for 8097/8096; they are not shipped runtime code. Physical nonblank/noncomment function SLoC:

| Function | Origin | New location | Status | SLoC |
| --- | --- | --- | --- | ---: |
| `stdout` | scripts/stage14-live.py | scripts/stage15-live.py:22 | acceptance tooling, unproven | 2 |
| `exchange` | scripts/stage14-live.py | scripts/stage15-live.py:26 | acceptance tooling, unproven | 9 |
| `compare` | scripts/stage14-live.py | scripts/stage15-live.py:37 | acceptance tooling, unproven | 33 |

## 2026-10-30 11:32 EDT — Stage 15 final acceptance record

The unchanged spec-mutant runner completed with exit 0: **15/15 killed,
0 survivors, 0 invalid**. All fifteen pre-intent refinement phases passed,
including the new record proofs. Thirteen focused intent lemmas killed faults;
all fifteen actual workspace test runs failed as expected for the deliberately
incorrect programs. Temporary-copy and working-tree hash checks passed. Archived
receipts: logs/stage15-mutants/results.json and per-mutant logs; top-level
logs/stage15-mutants.log. MUTANTS.md preserves the earlier stage evidence.

All stage 15 acceptance command checkboxes are complete. Verified per root:
**211 parser / 298 transport / 9 standalone AEAD**, zero errors (overlapping
roots), including **13 new record results**. Workspace and aarch64-linux Nix
both pass **43 tests**, preserving all 39 inherited tests. No prohibited token
matches in tracked first-party Rust. Nix check/static-image and live 8097 checks
pass, with 18 byte-identical response/stdout comparisons against 8096. Full new
equivalence counts and function inventories are above; no old-code/rustls bug
was exposed. Saturation above usize::MAX - 17 remains the sole authorized
behavior difference.

Local source measurement: seven new proven executable helpers, **57 SLoC**;
one ghost helper all_zero, **0 runtime SLoC / 3 ghost source SLoC**. TLS has
45 existing production functions, all retained in the shell: encrypt changes
31→33 SLoC, each of the other 44 is unchanged in count. Added shell helper
unpad_message has **10 SLoC**. Thus TLS production function SLoC is **305→317**.
The full per-function before/after table above includes all 60 resulting TLS
functions, including unchanged test functions; the record and added test/tool
function tables identify origin, new location, status and SLoC. These counts
are physical source inventories, not compiled-files coverage. Dudley measures
formal-coverage independently; it was never run or modified here.

Final audit logs/stage15-final-audit.log confirms: proven-record healthy;
all six protected container identities/pids/start times unchanged; branch still
proven-record; no diff in out-of-scope sources against initial a6a0bf8; no push;
no EVOLUTION.md/ATTACK-SURFACE.md edit; no prohibited Rust tokens; whitespace
check passed. Commits use configured Dudley identity. Writes stayed inside this
repository or Docker; no external diary entry was made.

Unproven by design: rustls type/error/range wiring, trait integration, payload
gathering, allocation/Zeroizing effects, upstream crypto/DER and TLS state
machine, RNG, private-key loading, hash/HMAC/HKDF/signing, config/IO, compiler,
OS and hardware. No sound local models/contracts exist for those upstream/OS
effects under the ban on opaque proof escapes. All six requested pure properties
are proven; selected shell effects are tested, not claimed proven. No end-to-end
TLS, secrecy/timing, resource-success or upstream release/target proof is claimed.
Physical browsers, separate LAN device and x86_64 were not exercised. These are
Codex's executed checks; Dudley's independent acceptance is still required.

## 2026-10-30 13:36 EDT — Stage 15 plaintext capacity fix started

Andrew requested one allocation fix on `proven-record`, starting at 37fb951.
`inner_plaintext` now reserves `payload.len() + 1` before copying bytes. The
existing `sealed_len` guard excludes overflow; behavior and the `ensures` clause
are unchanged. Added one capacity regression for lengths 0, 1, 255 and 16,384.
Checks are running with receipts under `logs/stage15-capacity-`.

The mutant runner hardcodes `logs/stage13-mutants`. Run its original source via
Python with only that output-directory literal redirected in memory to
`logs/stage15-capacity-mutants`; keep `__file__` at the original script path.
No runner file, mutation logic, proof phase, or test command changes. This keeps
all campaign receipts under the requested prefix and preserves older logs.
No containers, deployment, external memory writes, or files outside this repo
are in scope for this fix.

## 2026-10-30 14:01 EDT — Stage 15 plaintext capacity fix verified

`./verify.sh` exited 0: **211 parser / 298 transport / 9 standalone AEAD**,
zero errors, unchanged counts. `Vec::with_capacity(payload.len() + 1)` required
no proof adjustment; the existing guard proves the addition safe and the existing
loop and `ensures` clause remain unchanged. Reserving the final length before
any plaintext push prevents growth reallocations during construction.

Stage 15's five prohibited word tokens have **zero matches** across all 18
tracked first-party Rust files. `cargo test --workspace` exited 0 with **44
passed / 0 failed**, including all 43 inherited tests and the new exact-capacity
test at payload lengths 0, 1, 255 and 16,384. The test checks returned capacity;
it does not instrument the allocator or claim an end-to-end secrecy proof.

The full spec-mutant campaign exited 0: **15/15 killed, 0 survivors, 0 invalid**;
all refinement phases pass and every mutated workspace test run fails with
actual test failures. The runner's working-tree source hash checks pass.
Receipts: `logs/stage15-capacity-verify.log`, `-prohibited-scan.log`, `-tests.log`,
`-mutants.log`, and `logs/stage15-capacity-mutants/` (including results.json).
Only the documented log-directory redirect was used; no algorithm changes.

Final scope: one allocation line, one nine-line test addition, these notes,
PROGRESS.md and receipts. No deployment/image/container actions, branch changes,
push, protected-document edits or writes outside this repository. These are
Codex's executed checks; independent acceptance remains Dudley's. Nothing
unexpected surfaced; the test count increases by one as requested.

## 2026-10-30 14:16 EDT — Stage 15 capacity fix, owner check

Andrew reports the redeployed proven-record on 8097 (image sha256:809255c5…, commit 6860b91) works from his side. This is the first human check of this build; Dudley's automated live acceptance passed at 14:15.

## 2026-10-30 15:48 EDT — Stage 16 blocked by admission equivalence

Read PROVEN-IO.md fully first, then PROVEN-RECORD.md, PROVEN-DISPATCH.md,
IDLE-CONNECTIONS.md, PROOFS.md, MUTANTS.md, PROGRESS.md and NOTES.md. Started
on proven-io at 5ca8b97 with a clean working tree. No production code changed.

The required property-3 oracle exposes an overflow in the old admission closure
at src/main.rs:95–96 (unchanged from 6860b91):
`(n < 64).then_some(n + 1)`. Rust eagerly evaluates the then_some argument even
when n < 64 is false. At usize::MAX the old closure panics under debug overflow
checks, whereas the specified function must return None. Counts 0..=200 agree
exactly. This is an old arithmetic bug at an explicitly required oracle input,
not evidence of a reachable network denial of service: active starts at zero,
successful admission caps it at 64, and production releases disable overflow
checks. Release-mode testing confirms that the unused argument wraps and the
old closure returns None. No live connections or containers were used.

Following the mandatory stop/commit-evidence rule in PROVEN-IO.md (also user
non-negotiable 6), stopped before implementing io.rs, rewiring the shell, changing
the deadline or fixing the old closure. The evidence test preserves the closure
verbatim and compares it with a test-only expression of the specified admission
rule. Its MAX test asserts the observed debug panic and the matching release
result; a passing evidence test does not mean universal admission equivalence.

Both commands passed with two tests each:
`cargo test --test stage16_admission -- --nocapture` and
`cargo test --release --test stage16_admission`, using repository-local
CARGO_HOME/RUSTUP_HOME. Receipts: logs/stage16-admission-debug.log (contains the
caught overflow panic/location) and logs/stage16-admission-release.log. There
are 201 equal comparisons plus one differing debug boundary, repeated in release
with all 202 equal. No deadline/classification/port equivalence cases ran.

Suggested resolution for Andrew: authorize the guarded addition required by
property 3 as an additional permitted difference for unreachable counts under
overflow-checking builds, and explicitly permit resuming despite this evidence.
Shipped release outputs already agree at MAX. No production repair, spec
amendment, proof weakening or existing-test weakening was made here.

### Stage 16 inventory at blocker

Physical nonblank/noncomment whole-file source lines (including imports, types,
braces and existing test code; not a formal-coverage measurement):

| File | Before (5ca8b97) | After |
| --- | ---: | ---: |
| src/main.rs | 150 | 150 |
| src/transport.rs | 64 | 64 |
| src/lib.rs | 218 | 218 |

All three files are byte-identical to the starting commit. No production function
moved or was added; additional proven runtime SLoC is zero. No io.rs exists.
Existing production shell functions retain their previous status.

| Function | Old location/origin | New location | Status | Function SLoC |
| --- | --- | --- | --- | ---: |
| old_admit | src/main.rs:95–96 closure body | tests/stage16_admission.rs:4 | test-only old oracle, unproven | 3 |
| specified_admit | new, property 3 expression | tests/stage16_admission.rs:9 | test only, unproven | 7 |
| stage16_admission_equivalence_0_through_200 | new | tests/stage16_admission.rs:18 | test only, unproven | 5 |
| stage16_admission_max_exposes_old_overflow | new | tests/stage16_admission.rs:25 | test only, unproven | 10 |

Inventory receipt: logs/stage16-blocker-inventory.json. Test function SLoC counts
physical nonblank/noncomment signatures and bodies, including test assertions.

Stage acceptance remains incomplete: no new properties proven, TRUSTED-BASE.md
not created (no stage-16 subtotals), deadline timings plain/TLS not measured,
spec mutants not run (no stage-16 killed/15 result), Nix check/image not run,
8098 live checks and 8098-vs-8097 trickle test not run, no deployment. These are
deliberately deferred under the stop rule, not implied by prior-stage results.
No external diary write: this task restricts writes to the repository, TMPDIR
and Docker. This NOTES.md preserves the audit record for Dudley.

## 2026-10-30 15:49 EDT — Stage 16 blocker workspace validation

Full `cargo test --workspace` exited 0: **46 passed / 0 failed**, all 44
inherited tests unchanged plus the two admission evidence tests. The spec's
43-test baseline predates the stage-15 capacity regression; 44 is the actual
inherited count at 5ca8b97. Receipt: logs/stage16-blocker-workspace.log.
Prohibited-token scan passed with **zero matches across 19 first-party Rust
files**, including the new evidence test (logs/stage16-prohibited-scan.log).
`git diff --check` passed. Production source, proofs, authoritative spec and
protected documents remain byte-identical to 5ca8b97. No Docker commands,
network/live probes, formal-coverage, branch switches, push or external writes
were performed. Baseline full Verus is still completing; no new proof claim.

## 2026-10-30 15:50 EDT — Stage 16 baseline proofs and blocker checkpoint

`./verify.sh` exited 0: **211 parser / 298 transport / 9 standalone AEAD
verified, zero errors** (overlapping roots). Receipt:
logs/stage16-baseline-verify.log. These are unchanged baseline proofs; **zero
stage-16 properties** have been added or proven. The admission boundary evidence
requires an explicit resolution before implementation resumes. PROOFS.md stays
unchanged because no new proof exists. Checked first-party source/doc scope
against 5ca8b97 and git diff --check again; only NOTES.md, PROGRESS.md and the
new test file are changed. Commit uses the configured Dudley identity.

## 2026-10-30 15:59 EDT — Stage 16 resumed after admission approval

HEAD 401bbe2 contains Andrew's admission amendment. Implemented io.rs under the
existing transport root: deadline arithmetic and per-read bound, read
classification/start flag, guarded admission, saturating release/round trip,
port syntax and numeric completeness, shared crypto sizes. Focused I/O Verus
passes 19/0; full ./verify.sh is running. Shell converts nanosecond clock samples
with saturation, records the first decrypted byte once, maps error kinds while
retaining the original error, and uses the proven admission/release decisions
inside fetch_update. No other behavior change is intended.

Small interpretation choices: module declaration is at Rust module scope, with
its own verus! block, matching the existing aead/app/record pattern. The spec's
banned token `admit` is also its required API name; scans will report all such
identifier occurrences separately from forbidden proof primitives. No Verus
admit primitive is present. Key-shape APIs now use the shared size constants in
both types and contracts; no prior guarantee was weakened.

Oracle development found two harness count mistakes, not production differences:
the 15-symbol port alphabet yields 12,204,241 strings, and the OS plus stable
ErrorKind constructors expose 44 distinct variants in the pinned toolchain.
FilesystemLoop/InProgress are unstable names but stable raw-errno conversion
constructs them, plus InputOutputError/TooManyOpenFiles/Uncategorized. Tests cover
all stable constructors plus errno -1..=4096 and assert all 44 observed kinds.
Port equivalence has zero differences across the exhaustive set. Existing
43-test wording predates the capacity regression: inherited production baseline
is 44 tests; the two blocker tests are retained and updated for the approved fix.

The unmodified live suite fills all 64 workers, explicitly prohibited for this
stage. Added an opt-in --skip-capacity option (default behavior unchanged) and
--image-tag to acceptance.py; stage16-live.py sets those and compares 8098 to
8097 using the existing 18-request set. It will report capacity=false, and use
only three concurrent idle probes or two comparative trickle connections.
Admission at the cap is covered by the proven core and local atomic tests, not
by a live cap-filling check. This is an explicit user constraint, not a test
silently weakened. flake.nix uses the image name proven-io so nix-build.sh never
loads/overwrites scratch-server:static; wrapper supports NIX_LOG_PREFIX for
stage16-prefixed internal logs. No protected container will be reconfigured.

## 2026-10-30 16:03 EDT — Stage 16 core and shell checkpoints verified

Full ./verify.sh exited 0: **211 parser / 317 transport / 9 standalone AEAD**,
zero errors; io adds 19 results. Receipt logs/stage16-core-verify-final.log.
Prohibited-construct scan across 22 first-party Rust files finds zero forbidden
proof/escape constructs; the required admission API identifier is reported
separately with every location in logs/stage16-prohibited-scan-final.log.

Full cargo test --workspace -- --nocapture exited 0: **52 passed / 0 failed**,
including all 44 inherited tests, the two updated blocker tests, and six more
stage-16 tests. The release admission test command also passes all three tests
(logs/stage16-admission-release-final.log). No other old-code difference found.
Logs: logs/stage16-tests.log. Deadline plain/TLS timings: trickle 400 at
10.003/10.003 s (both logged reason=slow); eight-second complete request 200 at
8.038/8.041 s; after 3 s idle, request-to-response 0.004/0.001 s, both 200;
idle closure observed at 15.002/14.943 s since post-handshake client start, zero
bytes, asserted again after full 17 s. The server idle clock includes handshake.
At most two new local slow connections run concurrently. Inherited tests remain
unchanged in meaning.

Equivalence counts: 100,000 deadline pairs, compared with request deadline both
disabled and enabled; 39 stable error constructors plus 4,098 raw errno errors
and Ok(0/1/1024), all 44 distinct ErrorKind values observed, plus Data(MAX);
201 exact old/new admission counts and MAX approved exception in both profiles;
202 atomic admit/release inputs including MAX. Ports: 12,204,241 exhaustive byte
strings + 10,000 random strings + 7 explicit boundaries = **12,214,248** parser
comparisons, plus unset/non-UTF-8/default/error shell tests. No payload or header
value appears in assertion output. The only caught panic is historical MAX
admission evidence, now approved and documented.

## 2026-10-30 16:01 EDT — Stage 16 function/SLoC inventory

Old location is 401bbe2 (production source unchanged from the original 5ca8b97). Physical nonblank/noncomment source lines include signatures/braces. Core runtime excludes contracts, loop specifications, assertions and proof blocks. Ghost functions have zero runtime SLoC. This source audit does not run formal-coverage.

| File | Whole file before | After | Production source before | After | Production function SLoC before | After |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| src/main.rs | 150 | 171 | 150 | 149 | 137 | 136 |
| src/transport.rs | 64 | 70 | 64 | 70 | 44 | 49 |
| src/lib.rs | 218 | 226 | 135 | 141 | 110 | 116 |
| src/tls.rs | 1005 | 1006 | 408 | 408 | 317 | 317 |

| Function | Old origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| request_end | new | crates/transport-core/src/io.rs:13 | proven ghost | 0 | 4 |
| timeout_spec | new | crates/transport-core/src/io.rs:17 | proven ghost | 0 | 9 |
| read_timeout | src/transport.rs:23 inline timeout calculation | crates/transport-core/src/io.rs:26 | proven core | 12 | 0 |
| deadline_bound | new | crates/transport-core/src/io.rs:51 | proven ghost | 0 | 6 |
| classify | src/lib.rs:93 inline read match | crates/transport-core/src/io.rs:62 | proven core | 10 | 0 |
| admission_result | new | crates/transport-core/src/io.rs:78 | proven ghost | 0 | 3 |
| release_result | new | crates/transport-core/src/io.rs:81 | proven ghost | 0 | 3 |
| admit | src/main.rs:95 closure | crates/transport-core/src/io.rs:84 | proven core | 4 | 0 |
| release | src/main.rs:113 fetch_sub decrement | crates/transport-core/src/io.rs:90 | proven core | 4 | 0 |
| admission_round_trip | new | crates/transport-core/src/io.rs:95 | proven ghost | 0 | 5 |
| port_start | new | crates/transport-core/src/io.rs:101 | proven ghost | 0 | 3 |
| digit_range | new | crates/transport-core/src/io.rs:104 | proven ghost | 0 | 3 |
| port_form | new | crates/transport-core/src/io.rs:107 | proven ghost | 0 | 5 |
| decimal_prefix | new | crates/transport-core/src/io.rs:112 | proven ghost | 0 | 11 |
| parse_port | src/main.rs:16 port parse | crates/transport-core/src/io.rs:123 | proven core | 22 | 0 |
| sizes_agree | new | crates/transport-core/src/io.rs:180 | proven ghost | 0 | 3 |
| port#1 | src/main.rs:16 | src/main.rs:16 | shell | 5 | 0 |
| run#1 | src/main.rs:36 | src/main.rs:35 | shell | 79 | 0 |
| read#1 | src/transport.rs:22 | src/transport.rs:23 | shell | 8 | 0 |
| serve_tcp#1 | src/transport.rs:47 | src/transport.rs:44 | shell | 31 | 0 |
| elapsed_ns#1 | new | src/transport.rs:77 | shell | 4 | 0 |
| serve_with_started#1 | src/lib.rs:83 | src/lib.rs:83 | shell | 52 | 0 |
| read_result#1 | new | src/lib.rs:137 | shell | 11 | 0 |
| output_len#1 | src/tls.rs:80 | src/tls.rs:80 | shell | 3 | 0 |
| hash_len#1 | src/tls.rs:147 | src/tls.rs:147 | shell | 3 | 0 |
| x25519_peer | crates/transport-core/src/record.rs:81 | crates/transport-core/src/record.rs:82 | proven core, shared constants only | 9 | 0 |
| p256_point_shape | crates/transport-core/src/record.rs:95 | crates/transport-core/src/record.rs:96 | proven core, shared constants only | 2 | 0 |

| Test/helper function | Old origin | New location | Status | SLoC |
| --- | --- | --- | --- | ---: |
| old_classify | new | src/io_tests.rs:5 | test only, unproven | 10 |
| stage16_classification_all_error_kinds | new | src/io_tests.rs:16 | test only, unproven | 24 |
| random | new | src/io_tests.rs:42 | test only, unproven | 6 |
| old_timeout | new | src/io_tests.rs:50 | test only, unproven | 7 |
| stage16_deadline_equivalence_100000 | new | src/io_tests.rs:58 | test only, unproven | 29 |
| compare_port | new | src/io_tests.rs:89 | test only, unproven | 4 |
| stage16_port_exhaustive_and_random | new | src/io_tests.rs:95 | test only, unproven | 27 |
| stage16_request_deadlines_plain_and_tls | new | src/io_deadline_tests.rs:3 | test only, unproven | 84 |
| old_admit | blocker evidence, updated | tests/stage16_admission.rs:4 | test only, unproven | 3 |
| stage16_admission_equivalence_0_through_200 | blocker evidence, updated | tests/stage16_admission.rs:9 | test only, unproven | 5 |
| stage16_admission_max_exposes_old_overflow | blocker evidence, updated | tests/stage16_admission.rs:16 | test only, unproven | 10 |
| stage16_atomic_admission_and_release | new | tests/stage16_admission.rs:32 | test only, unproven | 18 |
| stage16_port_unset_non_utf8_and_error | new | src/main.rs:161 | test only, unproven | 18 |

scripts/stage16-live.py repeats stdout/exchange/compare from stage15-live.py and adds trickle/trickle_compare (unproven acceptance tooling). acceptance.py main/idle_checks add image-tag selection and an explicit opt-in capacity-probe omission; default checks remain intact. nix-build.sh adds only log-prefix selection; flake.nix changes only the image name. Audit generator logs/stage16-inventory.py is unshipped measurement tooling.

New IO executable core: **52 runtime SLoC**. TRUSTED-BASE.md totals **63 unproven production functions / 618 SLoC**, with per-function reasons, test coverage and explicit gaps. Every TLS production function remains shell. Runtime constants are not function bodies.

Stage 16 acceptance-tool function inventory (physical nonblank/noncomment lines; all unproven, not shipped):

| Function | Origin | New location | SLoC |
| --- | --- | --- | ---: |
| stdout | scripts/stage15-live.py | scripts/stage16-live.py:25 | 2 |
| exchange | scripts/stage15-live.py | scripts/stage16-live.py:29 | 9 |
| compare | scripts/stage15-live.py | scripts/stage16-live.py:40 | 33 |
| trickle | new | scripts/stage16-live.py:75 | 33 |
| trickle_compare | new | scripts/stage16-live.py:110 | 5 |

## 2026-10-30 16:09 EDT — Stage 16 Nix and deployment checkpoint

NIX_LOG_PREFIX=stage16-nix ./nix-build.sh check and image-static both exited 0. ARM Linux proofs repeat 211/317/9, zero errors; Nix workspace repeats 52 tests, zero failures. x86_64 was not run. Receipts: logs/stage16-nix-check.log, logs/stage16-nix-image.log, logs/stage16-nix-proof-log.log and logs/stage16-nix-test-log.log. The runner's only change is log-path selection.

Deployed only proven-io on 8098 from proven-io:static, sha256:5d1184971b5b66eafc1a76c8f6dad7abf5f5ad6bb6fd7572dac38331c238edab, 3,104,694 image bytes. Verified image tag, user 65532:65532, read-only rootfs, cap-drop ALL, no-new-privileges, read-only TLS and policy mounts, and healthy state. Existing TLS files are mounted only; no private-key content was read/copied/logged. scratch-server:static image ID remains byte-identical to the pre-load snapshot. Metadata receipt: logs/stage16-deployment-inspect.json.

The full live suite is running with the explicit capacity-probe omission. The mutant baseline passed full proofs/workspace tests; all 15 faults are queued in temporary copies. Runtime Rust remains frozen. No push, branch switch or protected-container mutation occurred.

## 2026-10-30 16:11 EDT — Stage 16 live acceptance passed

python3 scripts/stage16-live.py exited 0. ARM static image inspection found exactly the application binary in exported filesystem/image layers, no shell/key, no ELF interpreter or DT_NEEDED. Image 3,104,694 bytes; ELF 1,529,760 bytes. Hardened user/mounts/capabilities/no-new-privileges and healthcheck all pass.

8098 passes TLS1.3/ChaCha20/X25519/ECDSA P256 negotiation; TLS1.2/AES/P256 key exchange/h2/plain HTTP refusal; routes/curl/healthcheck; 3-second delayed 200, partial timeout 400, 17-second idle zero bytes; 48 malformed cases; 32 TLS fixture replays (8 fixtures x 4 fragmentation sizes); four curl browser replays; duplicates/header cap/privacy and 1,000-request/100-name log probe (59 new lines within the remaining 60-line budget). Capacity is explicitly false/not run: no 64-slot live probe was attempted.

All 18 scripted complete response buffers and stdout suffixes match proven-record on 8097 byte-for-byte. Comparative 35-byte trickle (one byte/TLS record every 1.5 seconds, only two slow live connections): 8098 gives **400 reason=slow at 10.058 s**, after 7 bytes; 8097 gives **200 at 51.023 s**, after all 35 bytes. Receipts: logs/stage16-live.log, logs/stage16-live/stage12-acceptance.json, rejection-equivalence.json and trickle-equivalence.json.

Protected snapshot comparison passed for scratch-server, nix-scratch, parser-http, parser-https, header-policy, proven-dispatch and proven-record: IDs/pids/start times unchanged. scratch-server:static image ID unchanged. Out-of-scope documents and PROVEN-IO.md unchanged from approved 401bbe2. No physical browser, separate LAN client or x86_64 check was performed. The only pending campaign is all 15 spec mutants; two killed so far.

TRUSTED-BASE.md audit refined per-method descriptions so pure accessors name rustls trait wiring as their reason, rather than attributing a libcrux call to a constant-returning body. Totals remain 63 functions / 618 SLoC. Unused secret extraction and fixed-string Debug formatters explicitly have no direct test coverage; this is a disclosed gap, not a claimed proof/test.

## 2026-10-30 16:31 EDT — Stage 16 final acceptance record

The full spec-mutant campaign exited 0: **15/15 killed, zero survivors, zero
invalid**. Every pre-intent refinement phase passes, including I/O and record
proofs. 13 focused intent lemmas kill faults; all fifteen actual workspace
test runs fail as required. Original source/fixture/config hash checks pass.
Temporary copies only; no working-tree mutation. Receipts:
logs/stage16-mutants.log, logs/stage16-mutants/results.json and per-mutant logs.
The sole runner adaptation is the previously documented in-memory output-path
redirect; no fault, proof, test command or kill criterion changed.

Final audit exited 0 (logs/stage16-final-audit.log): native, Nix ARM and mutant
baseline proofs **211/317/9, zero errors**; all three workspace baselines
**52 passed / zero failed**; all 44 inherited tests retained. Five new executable
I/O helpers total **52 proven runtime SLoC**; eleven ghost helpers have zero
runtime SLoC. Full origin/location/status/per-function counts, including test
and acceptance-tool functions, are in the stage-16 inventory above. Two new
shell helpers: elapsed_ns **4 SLoC**, read_result **11 SLoC**. Rewired wrappers
stay shell; crypto key-shape helpers stay proven using shared constants.

Whole-file main/transport/lib SLoC: **150→171 / 64→70 / 218→226**. Production
source excluding cfg(test): **150→149 / 64→70 / 135→141**. Function-body shell
subtotals in TRUSTED-BASE.md: main **136**, transport **49**, application **116**,
TLS **317** = **618 SLoC / 63 functions**. TLS production functions remain shell.
These are source inventories, not compiled-files coverage or percentages.

New equivalence: 100,000 deadline pairs with both deadline modes; all 44
ErrorKind values through 39 constructors and 4,098 errno cases plus Ok(0/1/1024),
plus Data(MAX); 201 exact admission comparisons plus MAX approved exception,
and 202 atomic admission/release cases; **12,214,248 port comparisons** plus
environment unset/non-UTF-8/default/error checks. No other old-code bug surfaced.
Plain/TLS trickle timings **10.003/10.003 s**, both 400 reason=slow; eight-second
requests **8.038/8.041 s**, both 200; after three idle seconds, response latency
**0.004/0.001 s**, both 200; zero-byte idle closures observed **15.002/14.943 s**
from post-handshake test start, checked again at full 17 seconds.

Nix check/image-static pass. Deployed healthy proven-io:static on 8098, image
sha256:5d1184971b5b66eafc1a76c8f6dad7abf5f5ad6bb6fd7572dac38331c238edab.
Image **3,104,694 bytes**, ELF **1,529,760 bytes**, static/shell-free/key-free,
non-root/read-only/no caps/no-new-privileges, read-only TLS and policy. Full
allowed live suite passes: protocol/suite/group/ALPN policy, browser fixtures,
48 malformed cases, privacy/log limiter, idle/slow, healthcheck and routes.
Eighteen full response/stdout comparisons are byte-identical to 8097. Live
trickle: **8098 400 reason=slow at 10.058 s** vs **8097 200 at 51.023 s**.
Only two comparative slow connections; no cap-filling attempt.

Final protected IDs/pids/start times and scratch-server:static image ID are
unchanged. Branch remains proven-io; no push, branch switch, protected-document
edit, formal-coverage run/edit, external memory write or secret logging. Rust
source is unchanged from the shell checkpoint 322f895. First-party scan across
22 Rust files: **zero forbidden proof/escape constructs**, six occurrences of
the required admit API identifier (listed separately, never the Verus primitive).

Unproven by design: Instant/OS timeout delivery, callbacks/Cell ownership,
atomics, threads, mutex/stdout/socket/filesystem effects, all rustls traits,
upstream TLS/crypto/DER, allocation/resource success, cleanup/secrecy/timing,
compiler/OS/hardware and upstream release/target proof correspondence. No sound
local models exist for those effects under the ban on opaque proof escapes.
All five requested pure properties are proven; no end-to-end TLS or wall-clock
liveness proof is claimed. Unused traffic-secret extraction and fixed-string
Debug formatters have no direct test; failure injection is not exhaustive.
Physical browsers, a separate LAN client, x86_64 and independent acceptance
were not run. Dudley performs coverage and independent acceptance separately.
The live capacity-fill check is deliberately omitted under Andrew's explicit
constraint, with capacity=false in the receipt; the cap is proven/unit-tested.

### Stage 17 — 2026-10-31: setup and measurement choice

Read RUSTLS-SURFACE.md and required stage context. Branch rustls-surface; initial
untracked codex-run-stage17.log belongs to the calling run and will not be committed.
No production access change is needed: tls::provider, tls::server_config and
serve_connection are public. Use native aarch64-apple-darwin release codegen for M1,
with debug=2 and strip=false only: the existing nix-build.sh invokes docker run,
which conflicts with the explicit no-container rule, and musl cross-linking is not
available in this host toolchain. This is the spec's permitted native fallback,
not a measurement of the Linux image. No containers or formal-coverage touched.
Memory diary writes are omitted because this stage explicitly forbids writes
outside the repository and named toolchain/temp exceptions.

### Stage 17 — 2026-10-31 06:46 EDT: M1 measured on the real musl binary (supersedes the native-fallback choice above)

The run continued under Claude after the Codex run stopped. Andrew clarified that ephemeral
`docker run --rm` build containers like nix-build.sh's are allowed (only long-lived
containers are forbidden; no existing container touched). So M1 measures the shipped
target instead of the native fallback: `scripts/rustls-surface-build.sh` builds the flake's
own `server-static` (aarch64-unknown-linux-musl, release profile, same flake.lock toolchain
and flags) via `nix build --impure --expr` with `overrideAttrs` adding only
CARGO_PROFILE_RELEASE_DEBUG=2, CARGO_PROFILE_RELEASE_STRIP=false, dontStrip and
doCheck=false, and copies rustc's dep-info `.d` files out. flake.nix and nix-build.sh are
unchanged. Build took about 3 minutes from the existing Nix store volume.
Codegen check against the shipped binary (extracted from image-static.tar.gz): identical
.text size (706072 bytes) and address, identical .rodata size; not byte-identical, but
99.67% of instruction words match as a multiset once PC-relative immediates (b/bl/adr/adrp)
are masked, i.e. the same code with a different function layout. Recorded as the codegen
caveat for M1 item 3.
Counting rules (scripts/rustls-surface.py): sources are the .crate archives from the cargo
cache, verified against the Cargo.lock checksums and unpacked to $TMPDIR; counter tokei
15.0.0 "code" lines. Crate total excludes test/bench-only files inside src/ (test.rs,
*_test.rs, test_macros.rs, *bench.rs); inline #[cfg(test)] modules inside other files
cannot be separated by tokei and remain in the total. Compiled = files in the crate's
dep-info. In-binary = distinct (file, line) pairs from DWARF 4 line-table sequences that
start in an executable section, with relative directories resolved against each CU's
DW_AT_comp_dir. Functions = defined t/T/w/W symbols in executable sections whose outermost
llvm-symbolizer frame lies in the crate.
Finding for item 5: the rustls client role is in the binary (751 in-binary lines) because
`--healthcheck` connects to our own endpoint with a rustls client. It is not on the accept
path and not peer-reachable from the network, so it is reported as "not unused" rather
than as dead code. No logging feature (no `log` crate compiled) and no TLS 1.2 files.
Attacker-facing module list (item 4) is a fixed file list in the script; extension codecs
live inside msgs/handshake.rs so they are counted with the handshake codecs.

### Stage 17 — 2026-10-31 07:00 EDT: fuzz harness, seeds, false OOM

`fuzz/` is a cargo-fuzz crate with its own `[workspace]` and its own Cargo.lock (started as
a copy of the server's, so rustls 0.23.46 / webpki 0.103.16 / pki-types 1.15.1 are the same
pins); the server workspace and Cargo.lock are untouched (cksum checked after every build).
Nightly toolchain 1.101.0-nightly (92b5b8c87 2026-10-29) and cargo-fuzz 0.13.2 live in
~/.rustup and ~/.cargo; homebrew cargo is first on PATH, so the nightly bin directory is put
first explicitly. All three targets load the production config through
`scratch_server::tls::load_config` from a throwaway P-256 key made by
scripts/make-lan-cert.py into $TMPDIR/stage17-tls (never in the repo). No production
visibility change was needed.
- `server_first_flight`: byte 0 seeds chunking (0 = one read), rest via read_tls.
- `server_after_handshake`: full in-memory handshake with a rustls client on our provider,
  then raw or sealed ops; sealed records use the client's extracted application traffic
  key/IV with our own ChaCha20-Poly1305 encrypter (the key stays in process memory; nothing
  logs it). Inner content type 0 lets the payload carry arbitrary padding. Limitation: after
  a KeyUpdate the harness keeps the old key, so later sealed records fail to decrypt.
  A `STAGE17_DEBUG` env var prints rustls error variants (no plaintext) for seed checks;
  the seeds give the expected errors (DecryptError after KeyUpdate, IllegalTlsInnerPlaintext,
  PeerSentOversizedRecord, ...) and app data is accepted.
- `proven_shell_pipeline`: `serve_connection` over `rustls::StreamOwned` and an in-memory
  socket, then the serve_tcp close_notify tail. serve_tcp itself takes a TcpStream for its
  deadline wrapper, so DeadlineSocket and the timeout paths are not exercised (choice logged).
  Raw mode feeds wire bytes; client mode embeds a rustls client in the socket so HTTP bytes
  reach the proven machine after a real handshake.
Seeds: scripts/stage17-capture-hellos.py ran the native release server on 127.0.0.1:18443
(binds 0.0.0.0 for under a minute) behind a recording relay on 18444 and captured the
ClientHello records of openssl s_client (1587 B), Homebrew curl/OpenSSL (1590 B), system
curl (319 B) and our rustls client (207 B); all four completed with HTTP 200. Only the
hello records are stored. `cargo run --example seed_corpus write` derives 8/22/18 seeds.
False OOM, not a finding: a 20 s trial reported an OOM at rss 301 MB after 2 s. Replaying
that input alone 1/3000/20000 times gives max RSS 48/215/441 MB with ASan defaults but 48 MB
after 20000 runs with ASAN_OPTIONS=quarantine_size_mb=0: ASan's 256 MB free-quarantine
counts toward RSS. The fuzz script sets quarantine_size_mb=16 (72-75 MB steady state for
all three targets), keeping -rss_limit_mb=256 and use-after-free detection.
Fuzz settings: -fork=2 per target, three targets concurrently (6 workers, within 8),
-max_total_time=1800, -timeout=1, -rss_limit_mb=256, -ignore_{crashes,timeouts,ooms}=0.
Inputs are not fully deterministic (server randomness and X25519/ECDSA nonces come from
getrandom through our provider); accepted, since the provider must not change.

### Stage 17 — 2026-10-31 07:35 EDT: fuzz results, coverage, acceptance

Runs 06:54–07:24 EDT, the three targets at once, 2 forked workers each (6 of the 8 allowed),
alongside the Bertie spike. All exited 0 after about 1815–1820 s of fuzzing, with
oom/timeout/crash 0/0/0 throughout and no artifacts, so there are no findings and
fuzz/findings/ was not created. Executions (last fork-mode status line) and on-disk corpus:
server_first_flight 29,848,149 / 2410 files; server_after_handshake 595,019 / 476;
proven_shell_pipeline 671,188 / 1780. The handshake per input costs throughput in the last two
(about 175–210 exec/s for two workers).
Coverage: scripts/stage17-coverage.py builds coverage binaries with `cargo fuzz coverage`.
cargo-fuzz's batched replay logged fewer inputs than the corpus holds, so the script replays
every corpus and seed file in a single -runs=0 pass and merges with llvm-profdata. The totals
matched the batched result exactly. Reached = lcov DA lines with count>0, intersected with
the M1 in-binary (file, line) set by crate-relative path (same source, different codegen:
the coverage build is not LTO/opt-z, so "reached (any line)" exceeds the in-binary count).
Result: 1972/4152 rustls, 271/693 webpki, 164/250 pki-types. Two harnesses run a rustls
client, so the script also reports the figure outside client-role files (1639/3252) and for
server_first_flight alone (1252), which has no client. Client-role code inside shared
files (codecs used by both sides) cannot be separated by file.
Largest unreached in-binary rustls files (unreached/in-binary): enums.rs 448/453,
client/tls13.rs 257/402, msgs/handshake.rs 253/580, server/tls13.rs 224/351, client/ech.rs
149/149, client/hs.rs 118/238, error.rs 110/114, tls13/key_schedule.rs 84/239,
msgs/enums.rs 61/66, msgs/persist.rs 52/52. enums.rs and msgs/enums.rs are almost entirely
`enum_builder!` macro invocations: DWARF puts the expanded code on the invocation lines,
but llvm-cov attributes macro-expanded regions to the macro, so those ~510 lines read as
unreached whatever the fuzzer does. This is a measurement artifact, and the true reached
figure is higher. The remaining unreached lines were not analysed one by one; that is left
for Dudley's review and is not a finding.
Acceptance: verify.sh 211/317/9 verified, 0 errors (logs/stage17-verify.log). cargo test
--workspace 52 passed (logs/stage17-cargo-test.log). Cargo.lock cksum unchanged after every
build. `git diff 9d6adbb` touches nothing under src/, crates/, config/, flake.nix,
flake.lock, Cargo.toml, Cargo.lock, nix-build.sh, verify.sh, EVOLUTION.md or
ATTACK-SURFACE.md. No tracked file contains a private key; the throwaway key stays in
$TMPDIR/stage17-tls. The only containers started were the two ephemeral --rm measurement
builds, and no images were created or left behind. Not pushed.
