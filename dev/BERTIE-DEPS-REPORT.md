# Stage 19c — Bertie dependency update

> D1–D4 are historical 19d results, superseded by E below (2026-11-21).
> E1 repairs extraction and passes all seven strict F* checks; E2 replaces the
> production client probe. Native/release/live checks and all twenty mutants pass locally. Independent
> acceptance remains outstanding.

Local checks by Codex; independent acceptance remains outstanding. B1 is partial:
minimum patched libcrux-kem cannot preserve Bertie's exposed legacy Kyber draft
API. No behavior/features/proofs/checks have been removed to cross that boundary.
No push, identity override or modification of existing Bertie worktrees.

## Branch and versions

New worktree `~/src/bertie-deps`, branch `deps/libcrux-advisories`,
base `8ff624adda8b74c8c0edbec29b7c92492164233f`. Configured author: Andrew VanLoo.
Partial B1 revisions: `fd496381be78fc53e5a43fc969ee450670174ebb`, then
`03374d49c29f5dad55252584fa362ade0ea3010e` (final pin).

| Dependency | Before | After | Disposition |
|---|---|---|---|
| libcrux-kem | 0.0.2 | 0.0.2 | BLOCKED: minimum fixed 0.0.10 removes kyber feature / X25519Kyber768Draft00 |
| libcrux-sha3 (old KEM path) | 0.0.2 | 0.0.2 | BLOCKED: exact transitive pin from old KEM |
| libcrux-ecdh | 0.0.2 | 0.0.2 | BLOCKED: exact transitive pin from old KEM |
| libcrux-chacha20poly1305 | 0.0.2 | 0.0.9 | Smallest fixed release compatible with full server graph |
| libcrux-poly1305 (Bertie AEAD) | 0.0.2 | 0.0.6 | Exact dependency of chacha20poly1305 0.0.9; minimum advisory fix 0.0.5 |
| libcrux-ed25519 | 0.0.3 | 0.0.9 | Smallest fixed release compatible with full server graph |
| libcrux-ecdsa / sha2 / hmac / hkdf | 0.0.3 | 0.0.3 | Unchanged |
| libcrux-rsa | 0.0.2 | 0.0.2 | Unchanged |
| Bertie rand | 0.9.5 | 0.9.5 | Unchanged |
| scratch-server p256 | 0.13.2 | 0.14.0 | Stable, same key-decoding API |
| scratch-server elliptic-curve | 0.13.8 | 0.14.3 | From p256 |
| scratch-server crypto-bigint | 0.5.5 | 0.7.11 | Patched; no exception |

Crates.io checked 2026-11-21 UTC. Requirements retain upstream's `0.0.x` form
(which cannot cross the next patch digit under Cargo's 0.0 semver rules).
Raw metadata/dependencies: `logs/stage19c-crates.json`,
`logs/stage19c-crate-dependencies.json` and
`logs/stage19c-minimum-compatible-dependencies.json`; KEM 0.0.10 source/changelog downloaded
from the crates.io release archive under ignored `target/stage19c-crates/`.

## Compatibility boundary and API changes

Bertie code changes: none. Default `api,std,defensive` feature list, all dependency
feature selections, DER code and proof sources are unchanged. Only two manifest
requirements move. Individual advisory minima 0.0.8 (AEAD) and 0.0.7 (Ed25519)
resolve hax-lib =0.3.6 via secrets =0.0.5, which conflicts with existing
core-models -> hax-lib ^0.3.7. AEAD 0.0.9 and Ed25519 0.0.9 (via sha2 0.0.8)
are the first releases resolving secrets 0.0.6 / hax-lib ^0.3.7. Ed25519 0.0.8
still has the old secrets constraint through sha2 0.0.7 / traits 0.0.7.
The final suite rerun uses both 0.0.9 versions; see stage19c-bertie-tests99.log. libcrux-kem 0.0.10 has no `kyber` feature or
`Algorithm::X25519Kyber768Draft00`. Removal dates to libcrux-kem 0.0.3 (changelog).
Bertie exposes that algorithm through its enum, named algorithm constant,
identifier conversion and string conversion. Returning UnsupportedAlgorithm,
removing its API or substituting ML-KEM changes behavior and is not authorized by
the no-behavior-change fence. Keeping an old-KEM alias retains vulnerabilities.
Reimplementing hybrid crypto glue is beyond a minimum dependency/API patch.
Asked Andrew for a narrow scope decision; preserved the API pending an answer.

The fixed KEM additionally requires rand ^0.10, libcrux-ecdh =0.0.9 and
libcrux-sha3 =0.0.11; these are the minimum graph implied by its exact pins, not
just the individual advisory minima. Its changed RNG traits need coordinated
Bertie caller and hax-model review if the algorithm-removal conflict is resolved.

## crypto-bigint investigation

The locked old production path is scratch-server -> p256 0.13.2 ->
elliptic-curve 0.13.8 -> crypto-bigint 0.5.5. ecdsa 0.16.9 and primeorder 0.13.6
also lead from p256 to elliptic-curve. Contrary to the initial hypothesis, Bertie
itself has no RustCrypto p256 dependency at this revision; libcrux-p256 is distinct.
Evidence: `logs/stage19c-bigint-before.log`, manifests, and inverse tree after update.

[RUSTSEC-2026-0387](https://rustsec.org/advisories/RUSTSEC-2026-0387) reports
secret-dependent branches in subtraction, zero counting, division and Montgomery
multiplication depending on compiler/CPU, early-exit UintRef equality and an
invertibility branch in BoxedMontyForm::invert. It requires >=0.7.6. Our production
key loader parses a private scalar and validates an optional public point at
startup through elliptic-curve SecretKey and ScalarPrimitive; this reaches secret
integer operations, so absence of exposure was not assumed. Test-only Config
uses the same parser. No runtime exploitability/side-channel audit is claimed.

Crates.io now provides stable p256 0.14.0, depending on elliptic-curve ^0.14.1;
0.14.3 requires crypto-bigint ^0.7.8. An isolated compile of the existing PEM key
loader passes unchanged. Targeted `cargo update -p p256` resolves crypto-bigint
0.7.11. No pre-release or large rewrite is needed; no crypto-bigint ignore is added.

## Bertie gates and reproduction

Used Bertie workspace member: **bertie only** (scratch-server does not depend on
record, simple_https_client/server, bogo_shim or integration_tests). The isolated
Docker workspace mirrors bertie-der's source-copy method, including unchanged
src, tests, benches, test_certs and manifest with only outer workspace tables
removed. The full used-member command is:

```sh
docker run --rm --network none --cpus 6 --memory 5g \
  -v "$PWD/target/stage19c-bertie:/work" -w /work/ws \
  -e CARGO_HOME=/work/cargo-home rust:1.98.0-trixie \
  cargo test --locked --offline --release -p bertie --no-fail-fast \
  -- --skip stream::client::tests::client
```

The network client test is filtered exactly as in bertie-der (network-disabled
container); no claim that it ran. Existing ignored tests remain ignored. The
first isolated run omitted test_certs and failed its fixture lookup; unchanged
public fixtures were restored and the full suite rerun. Initial API integration
handshakes: 2/2 passed. Final suite PASS: 15 library tests, 2 TLS API handshakes and 1 doctest;
zero failures, 2 existing ignored tests, 1 network test filtered.

**hax 0.4.2 / F* gates BLOCKED before extraction.** The original script's existing
cache `<workspace>/projects/formal-coverage/runs/v1.7/bertie`
is missing; Docker rejects the read-only bind source. The corresponding moved
results directory has no compiler/toolchain cache. Asked Andrew for its location.
No toolchain was installed, no proof source modified, and no historical proof
receipt was relabeled as a fresh run. Bertie.Der and Bertie.Tls13formats have
**not** been freshly extracted/typechecked. No libcrux-induced proof failure can
be asserted until extraction runs. `scripts/stage19c-bertie-docker.sh` reuses the
original script method in an isolated scratch-server target directory, with
`BERTIE_HAX_TOOLCHAIN` for the existing cache location. Original bertie-der files
and outputs remain read-only. No assume/admit/lax option was added.

## scratch-server gates / release

Final locked native tests: 57 passed / zero failed; proof counts 211/347/9 +
hello30, zero errors; production tree rustls-free. Strict handshake loop passes 10,000/10,000 (zero failures, 424.43 seconds).
Twenty mutants: 20/20 caught, zero survivors/invalid; all refinement gates pass,
all twenty have actual test-failure evidence, seventeen intent kills (four hello).
Final receipts: logs/stage19c-summary.json and stage19c-mutants/results.json.
Nix cargo-deny fails with six retained KEM/ECDH/SHA3 vulnerabilities and one
unmaintained proc-macro-error2 finding (RUSTSEC-2026-0173). B3 is blocked; no
full flake check, image build, live probes or shipped-image audit attempted.
The requested two ignores were added after all-target/all-feature inverse trees
confirmed each path ends at hax-lib-macros 0.2.0 (proc-macro). The production
normal tree with proc macros excluded contains neither. Exactly those two ignores
are present; the pre-existing cfg(hax) RUSTSEC-2026-0173 ignore was removed to honor
the explicit two-only policy. Snapshot and clock unchanged. Native offline deny also fails before advisory evaluation because its local DB is
absent; no fetch/clock/snapshot change was used. The completed Nix receipt is the
substantive advisory result. See logs/stage19c-native-deny.log and stage19c-deny-log.log.
Any failing advisory remains a release blocker; no ungated image or live container.


Commits: Bertie `fd49638` + `03374d4`; scratch-server B2 `aacb09c`, B3 blocked
record `5757723`; final record commit follows. No pushes. Native tests/handshake
processes exited zero; final pin/lock/policy/protected-scope checks pass. B1 and
release acceptance remain explicitly incomplete for the blockers above.

## D1

Andrew's 2026-11-21 decision replaces the Bertie behavior fence. Removed the obsolete Kyber draft enum, constant, conversions, benchmark entries, documentation and integration test. KEM 0.0.2 → 0.0.10 (minimum patched); exact transitives ECDH 0.0.9, SHA3 0.0.11. A private rand_core 0.10 adapter forwards to existing caller rand 0.9; no RNG source change. Retained identifiers 0x11ec/0x001d/0x0017 and public-key/ciphertext/shared-secret lengths 1216/1120/64, 32/32/32, 65/65/32 are asserted, including shared-secret agreement.

Full workspace `cargo test --workspace --no-fail-fast -- --skip stream::client::tests::client`: **25 passed**, zero failed, 3 original ignored (including doctest), 1 external-network test filtered. Breakdown 16 library, 2 API, 5 stream integration handshakes (all three retained groups plus two RSA configurations), 1 simple-client API, 1 doctest. Existing negative DER tests run unchanged within library suite. Receipt logs/stage19d-bertie-d1.log. No source/proof weakening; src/der.rs unchanged.

D1b baseline at original scratch-server pin 03374d4: **1,200 Bertie in-binary source lines / 665 functions**, production tree **112 unique packages** (includes root and workspace packages). Fresh stage17-method musl audit, logs/stage19d-before-surface.json; source audit, not a shipped-image measurement.

## D1b

Bertie commit a20fa69 follows D1 d34ad5e. Default features remain api/std/defensive; new non-default `client` gates Client, its stream module, exported alias and all executable client-handshake functions. Test/dev workspace consumers explicitly enable it; scratch-server does not. Production target `cargo tree -e features,no-dev` confirms no Bertie client feature; `-e normal` confirms no rustls, libcrux-rsa or libcrux-ed25519. Production unique packages **112 → 109** (root/workspace included, same method). RSA/Ed25519 crypto implementations and dependencies, RSA named configurations and PSK server entry/skip-signature functions are removed. Unsupported configurations now return UNSUPPORTED_ALGORITHM before parsing, RNG use or signing. Invalid KEM decoding returns an error instead of unwrap panic.

Shared ClientHello/format/spec definitions, algorithm identifier enums, key-schedule definitions and shared PSK-binder helper are retained to avoid editing proven code internally. Whole gated client items and removed whole items: sign_rsa, supported_rsa_key_size, valid_rsa_exponent, get_rsa_signature, server_init_psk, get_skip_server_signature, get_skip_server_signature_no_psk; RSA/Ed25519 match branches become unsupported errors. No DER or tls13formats source edit, no new assume/admit/opaque/lax. Existing positive PSK test becomes rejection test, and the two removed RSA positive configurations are replaced by a negative suite covering AES/RSA/Ed25519/X448/PSK at both client/server APIs. Initial workspace **24 passed / zero failed**, 3 original ignored, 1 network test filtered; retained-group stream handshakes **3/3**, negative DER and KEM size/secret checks pass. Logs stage19d-bertie-d1b.log. At that checkpoint, binary after measurement and cargo-deny were pending. This retains shared proof scaffolding; it is not a claim all unused source text has been deleted.

D3 proc-macro-error2: all-target/all-feature inverse tree shows only hax-lib-macros 0.3.7 (proc-macro), via hax-lib/core-models/libcrux and hax-lib/secrets. Absent from production normal tree excluding proc-macros. Narrow RUSTSEC-2026-0173 unmaintained exception added; existing two approved exceptions unchanged. No new vulnerability exception, snapshot or clock change.

D1b wire-offer follow-up: workspace **25 passed / zero failed** (16 library, 4 API, 3 retained-group stream handshakes, 1 simple-client API, 1 doctest; 3 ignored and 1 network filtered). New wire test starts with a real ClientHello and replaces suite/signature/supported-group/key-share offers with AES/RSA/Ed25519/X448; server rejects all five without panic. Receipt stage19d-bertie-d1b-wire.log. Production source unchanged by this test-only commit; prepared scratch pin remains implementation commit a20fa69.

Nix cargo-deny on a20fa69 **PASS** (advisories, bans, licenses, sources); logs/stage19d-deny-log.log. Baseline 19c at identical original 03374d4 graph failed 6 KEM-path vulnerability findings plus proc-macro-error2. Two old unmaintained exceptions now warn that no crate matches; the new exception is build-proc-macro only. No snapshot/clock/flake gate change. Native scratch verify passes 211/347/9 + hello30; locked workspace tests and twenty-mutant baseline fail E0432 because the authenticated production healthcheck uses Bertie Client. No handshake loop/image/live/after binary audit while this conflict remains unresolved.

## D2

**PARTIAL; strict F* gate FAILS.** Rebuilt toolchain at the sole authorized root
`~/src/bertie-toolchain`; no deleted OpenClaw cache recreated. Existing
Docker images were used unchanged/read-only as image bases, never retagged/deleted.
Rust nightly-2025-11-08 is 1.93.0-nightly (843f8ce2e, 2025-11-07). Hax 0.4.2
source is exact commit 5ccf89dd2e996c9f88f2267eaa2a64739c4ca41a. The release CLI
was downloaded; matching frontend, Rust engine and OCaml engine were rebuilt from
the exact-commit archive. Archive builds print `commit=unknown`; provenance comes
from the pinned source URL/checksum, not an invented version-string receipt. F*
2025.10.06 is 7b347386330d0e5a331a220535b6f15288903234 (amd64, OCaml 4.14.2),
bundled Z3 4.13.3; hax's OCaml compiler is 5.3.0.

| Download | SHA256 |
|---|---|
| hax aarch64 Linux release archive | 6ff3905349417637fc1aac7e88a12467c205903c3ee1215504dd9f705a517db4 |
| hax exact 5ccf89d source archive | 682c6caa457a9b2167727b84c91b13aebaf259583fc09a162d0bb9ac42504475 |
| Rust nightly full archive | 4780ffdde76aaf3e93b6942f994f0f3f9495f6b46623ad7f3de1a767505e58ea |
| Rust nightly rustc-dev archive | 9acf81a71929b8cdc41e0bf08240de5060ea8e88deb4bdda72c079577db970b8 |
| F* Linux x86_64 archive | 940c187d7a6dc3e95d17fd5159b5eaec17d62981c7f135a7e6c531ce3b59442d |

Full URLs/checksums, including metadata, OPAM index, Rustup-managed components and
initial superseded source-tag archive, are in `bertie-toolchain/downloads.sha256`.
Cargo/OPAM source-cache provenance: `bertie-toolchain/dependency-downloads.json`,
**676 URL/checksum records**, SHA256
`12cf0243fea53d08479d033283f67b70359995f00bf535dc6c05e42ff527daa6`.
OPAM exact package export is `bertie-toolchain/opam-hax.export`; aggregate receipt
`logs/stage19d-toolchain-provenance.json` records image IDs and ledger hashes.
Reproduction: stage19d-toolchain.sh, stage19d-hax-build.sh (rust, then ocaml),
stage19d-extract.sh (baseline/current), stage19d-typecheck.sh MODULE. Scripts
pass shell/Python syntax checks. Tool assembly failures were logged and corrected
without source patches: missing zstd unpacker (host tar used), missing benchmark
fixtures, rustc-private feature unification (upstream sequential installer), shared
compiler-library path, and parallel OCaml/Dune memory/dependency failures (serial
build). Original scripts and model libraries remain unmodified.

Fresh hax baseline 03374d4 and candidate e1af495 extractions **both exit 0**,
**21 files each**. Seven files differ: API fst/fsti, crypto fsti, handshake fst/fsti,
and both key-scheduler fst files (the latter show generated hoist-name changes).
Der fst/fsti and Tls13formats fst/fsti are byte-identical baseline/candidate.
Diff manifest: logs/stage19d-extraction-diff.json. No model, proof-source or
extraction patch; no assume/admit/lax added. Original abstract crypto models
remain part of the trust boundary; their existing assumptions are not claimed
to have been eliminated.

| Strict F* check | Exit | Seconds |
|---|---:|---:|
| Bertie.Der.fst | 0 | 333 |
| Bertie.Tls13api.fst | 1 | 74 |
| Bertie.Tls13crypto.fsti | 1 | 70 |
| Bertie.Tls13formats.fst | 1 | 72 |
| Bertie.Tls13handshake.fst | 1 | 73 |
| Bertie.Tls13keyscheduler.Key_schedule.fst | 1 | 69 |
| Bertie.Tls13keyscheduler.fst | 1 | 71 |

Der: all verification conditions discharged successfully. The other six checks
each report **one Error 134**, at generated
`Bertie.Tls13crypto.fsti:9:30-37`: namespace
`Bertie.Tls13crypto.Non_hax` cannot be found. This is introduced by Codex's
e1af495 RNG-boundary change: the module is excluded by the original extraction
filter, but an implicit-dependency `open` remains. The item is **stopped**, as
Andrew instructed; no generated-file patch, extra abstraction/assumption, lax
check or source repair was used to turn this into a pass. Tls13formats itself
is unchanged but cannot typecheck through that failed dependency. Individual
receipts: logs/stage19d-proof-*.log; aggregate logs/stage19d-proofs.json.

New trusted Bertie glue is Algorithms::validate plus the three RNG forwarding
methods. Those methods do not gain a proof from hax exclusion. Shared client
types, format/proof definitions and shared PSK/key-schedule helpers remain to
avoid internally changing proven code. Historical checked-in extraction/model
artifacts are not silently rewritten or relabeled as these fresh results.

## D3 / D4 disposition

Candidate pin e1af495 includes d34ad5e, a20fa69, 2f2c2c2 and e1af495. Lock update
is targeted to Bertie and required graph changes. Final feature/inverse/normal
trees confirm no client feature, no production rustls/RSA/Ed25519, 109 packages
including server (108 dependencies), and proc-macro-error2 only behind a
proc-macro. Final Nix deny PASS: logs/stage19d-final-deny-log.log. Only the
approved RUSTSEC-2026-0173 unmaintained ignore is new; old two ignores are now
unmatched warnings, no vulnerability exception. Snapshot/clock/flake unchanged.

Native proofs PASS 211/347/9 + hello30. Final locked workspace tests fail
compilation E0432 at src/tls.rs:183, because the existing authenticated healthcheck
uses the gated client. The twenty-mutant harness passes proof baseline and stops
at failed test compilation, so **no final mutant kills** are claimed. The strict
10,000 loop cannot run. No healthcheck contract change or client feature
enablement was authorized; Andrew's requested decision remains pending.

Final full Nix check FAILS the same compile gate. No image build, container on
8099, temporary reference, eighteen-request comparison, live interop or shipped
image audit. No ungated substitute. Fresh baseline in-binary **1,200 lines / 665
functions**, after **unavailable** because candidate cannot compile; this is
a source audit, not an image measurement. Production tree **112→109 including
root**, or **111→108 dependency crates**. No formal-coverage executable; no new
coverage percentage/browser/independent acceptance. D1b, D2, D3 and D4 acceptance
remain incomplete. No push, remote workflow, key exposure or existing-service
mutation.

Native offline deny was attempted again on the final candidate and exits 1
before advisory evaluation because `.advisory-db/advisory-db-3157b0e258782691`
is absent. No DB fetch, snapshot or clock change; the final Nix deny result is
the substantive advisory verdict. Its newly created empty db.lock was removed.
All nine maintained commits since ef65f76 are listed in the spec note (including
b10a3f0's DER codec, not only its integration head).

Checkpoint commits (configured Dudley): D2 ce83b0d, D3 dcdad81, D4 74749b4;
final receipts are recorded with this documentation. Final assertions in logs/stage19d-summary.json
confirm protected source/proof/test/mutant/flake paths are unchanged, both branches
are correct, Bertie is clean, and acceptance is false. No new native test, mutant,
loop, image or live success is implied.

## E1 — 2026-11-21 proof-boundary repair

Andrew authorized repair of the stopped namespace failure. Bertie commit
`e6ea574f698aadf8c56174b878f945c251a7fccb` moves libcrux key generation and
encapsulation calls inside the existing `tls13crypto::non_hax` module. The
rand_core 0.10 instance is now used only there; the extracted abstract crypto
interface no longer opens the excluded namespace. RNG source, errors and KEM
outputs are unchanged. No proven item, proof source/model, generated patch,
admit/assume/lax flag or dependency version changed.

Bertie's new `TRUSTED.md` explicitly lists the three RNG forwarding methods and
the two KEM wrappers as trusted compatibility glue. The existing abstract crypto
interface does not prove backend implementations or correspondence. The local
strict F* checks are protocol/format checks under that boundary, not an end-to-end
TLS or cryptographic security proof.

Fresh hax 0.4.2 extraction passes, 21 files; E1 changes only crypto.fsti by removing
the unresolved open. The D1/D1b cumulative module set remains API, crypto,
handshake and both key schedulers. Reproduction scripts stage19e-extract.sh and
stage19e-typecheck.sh use the unchanged pinned toolchain/model libraries and a
fresh target/stage19e-bertie output directory. All seven strict checks pass, zero errors.
Bertie locked offline full workspace suite passes: 25 passed, zero failed,
3 original ignored and 1 external-network test filtered. Retained-group stream
handshakes 3/3, wire rejection, DER and KEM-size/shared-secret tests pass.
Receipt: logs/stage19e-bertie-tests-cached.log.

### E1 strict F* receipts

| Module | Exit | Seconds |
|---|---:|---:|
| Bertie.Der.fst | 0 | 381 |
| Bertie.Tls13api.fst | 0 | 83 |
| Bertie.Tls13crypto.fsti | 0 | 97 |
| Bertie.Tls13formats.fst | 0 | 133 |
| Bertie.Tls13handshake.fst | 0 | 76 |
| Bertie.Tls13keyscheduler.Key_schedule.fst | 0 | 79 |
| Bertie.Tls13keyscheduler.fst | 0 | 72 |

All VCs discharged in every invocation; original model files (147) are unchanged.
No proof patch or new assume/admit/lax directive was introduced. Existing abstract
model assumptions remain; this is not a proof of the crypto backend.
Aggregate: logs/stage19e-bertie-proofs.json.

## E2 / E3 — 2026-11-21

Production healthcheck now uses a fixed 208-byte TLS 1.3 ClientHello and bounded
first-ServerHello readiness, no TLS client library or crypto. Four focused tests
pass, including real Bertie and closed/non-TLS/alert/truncation/absolute-timeout
cases. Executable positive/closed-port exit codes are 0/1. The production musl
feature tree excludes client; normal tree excludes rustls/webpki/pki-types and
removed RSA/Ed25519 crates. Exact pin/lock move only Bertie from e1af495 to e6ea574.
No dependency version/policy/snapshot/clock changes in E.

Native proofs 211/347/9+hello30, 60 locked workspace tests and 10,000 strict
handshakes (0 failed, 494.90s) pass. Nix deny passes all four categories; native
offline deny has no DB, no native advisory claim. Nix check/image-static pass on
aarch64 Linux. Healthy hardened container on 127.0.0.1:8099 during acceptance;
18/18 HTTP responses byte-identical to temporary proven-io-ref, removed afterwards.
Final certificate-verified interop: 26 passing rows plus 2 local client-unsupported
selections. The first run exposed the spike's empty-EOF timeout expectation;
the E wrapper instead enforces unchanged HTTP 400, authenticated closure and
10<=elapsed<12 against both images. No server behavior/deadline changes.

Stage17-method same-target/profile DWARF companion: **1,200→736 Bertie lines**,
**665→409 functions**, old TLS crates zero production dep-info. Production package
identities **112→109 root included**, **111→108 dependencies**. Actual stripped
image binary is bound by SHA256 in stage19e-binary-correspondence.json. Debug
companion .text differs by 448 bytes; the line count is its upper bound, not
byte/semantic equivalence or runtime/proof coverage. See the E spec for hashes,
image ID, interop rows and reproduction scripts. No formal-coverage executable,
new percentage, x86_64 server or physical-browser/independent acceptance claimed.
Twenty mutants pass: 20/20 caught, zero survivors/invalid; all refinement gates
pass and all twenty have actual failed-test evidence, 17 intent kills. All
requested automated E gates pass locally; independent acceptance remains outstanding.
