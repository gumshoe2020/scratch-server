# scratch-server

[![CI](https://github.com/gumshoe2020/scratch-server/actions/workflows/ci.yml/badge.svg)](https://github.com/gumshoe2020/scratch-server/actions/workflows/ci.yml)
[![proofs](https://github.com/gumshoe2020/scratch-server/actions/workflows/verify.yml/badge.svg)](https://github.com/gumshoe2020/scratch-server/actions/workflows/verify.yml)
[![coverage](https://github.com/gumshoe2020/scratch-server/actions/workflows/coverage.yml/badge.svg)](https://github.com/gumshoe2020/scratch-server/actions/workflows/coverage.yml)
[![proven](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/gumshoe2020/scratch-server/badges/proven.json)](COVERAGE.md)
[![tested](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/gumshoe2020/scratch-server/badges/tested.json)](COVERAGE.md)
[![unverified](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/gumshoe2020/scratch-server/badges/unverified.json)](COVERAGE.md)
[![proof-ratio](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/gumshoe2020/scratch-server/badges/proof-ratio.json)](COVERAGE.md)
[![ratchet](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/gumshoe2020/scratch-server/badges/ratchet.json)](COVERAGE.md)

A small HTTPS server with a strict HTTP subset. The goal is a web server whose
attack surface is our own code and pinned dependencies, with as much as possible
formally proved.

Verus checks the HTTP parser, header policy, transport decisions, dispatch,
response formatting, deadlines, admission rules and ClientHello selection against
written specifications. Socket adapters, clocks, threads, cryptographic backends,
compiler, verifier, operating system and container runtime remain trusted.
See the [proof boundary](verification/reports/PROOFS.md) and
[trusted inventory](verification/TRUSTED-BASE.md).

TLS comes from [bertie-tls](https://github.com/gumshoe2020/bertie-tls), our maintained
fork of Cryspen's Bertie. Credit to Cryspen for Bertie and its protocol proofs.
Production enables Bertie's api/std/defensive features; its client feature is off.
The exact source revision is pinned in [Cargo.toml](Cargo.toml) and Cargo.lock.

## Build and run

Linux image builds require Docker. Native verification requires Rust and rustup.
Cargo fetches the exact public Bertie revision pinned in Cargo.toml and Cargo.lock.

```sh
mkdir -p logs
./nix-build.sh check > logs/check.log 2>&1
./nix-build.sh image-static > logs/image.log 2>&1
docker run --rm --read-only --user 65532:65532 --cap-drop ALL \
  --security-opt no-new-privileges -p 127.0.0.1:8099:8080 \
  --mount type=bind,src=/absolute/tls,dst=/tls,readonly \
  bertie-boundary:static
```

The TLS directory contains a readable P-256 certificate and PKCS#8 key as
cert.pem and key.pem. Keys stay outside the checkout and image.
SCRATCH_PORT defaults to 8080 and SCRATCH_TLS_DIR to /tls; invalid TLS material
fails startup. SCRATCH_HEADER_POLICY optionally selects a policy file; otherwise
the built-in policy applies. See [header policy](specs/established/HEADER-POLICY.md).

The in-binary healthcheck sends a fixed ClientHello to loopback and checks for a
bounded ServerHello within two seconds. It checks readiness without authenticating
the peer or exercising HTTP. External acceptance checks verify certificates and
HTTP responses.

## Verify and measured results

```sh
./verification/verify.sh > logs/proofs.log 2>&1
CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" \
  cargo test --locked --workspace --no-fail-fast > logs/tests.log 2>&1
python3 verification/scripts/spec-mutants.py > logs/mutants.log 2>&1
```

[Local results, 2026-11-21](specs/development/BERTIE-BOUNDARY.md#results--2026-11-21-e):
Verus: parser 211, transport 347, retained AEAD helpers 9, standalone hello 30;
zero errors. These proof roots overlap.
Seven extracted Bertie F* checks pass under their existing abstract crypto
models. Workspace tests: 60 passed. Mutants: 20/20 caught. Strict-client
handshakes: 10,000, zero failures. Nix check and static image gates pass.
The stage-17 DWARF method counts 736 Bertie lines in a debug companion of the
image's source/profile; this is an upper bound, not execution or proof coverage.

The fresh [formal-coverage v1.9.0 measurement](COVERAGE.md) counts 887 proven,
373 tested and 450 unverified lines out of 1,710 first-party security runtime
SLoC: **51.87% proven**, with a **6.40** proof-to-code ratio. Tested means syntactic
test references, not executed line coverage. Unverified includes 83 hazard lines;
see COVERAGE.md for the tool's conservative admission-name warning and evidence limits.
The last
[formal-coverage v1.4 inventory](verification/reports/PROOFS.md#first-party-proof-coverage),
for source commit 2fa5eb7 (2026-10-27), counted 644/1,363 first-party security SLoC
proved (47.25%). That percentage describes the older source, not today's server.

## Limits

- One GET or HEAD request per connection; no bodies, queries, percent escapes,
  pipelining or keepalive. Limits: 4,096 request bytes, 24 headers, 64-byte names,
  256-byte values and paths. Header values never reach the handler.
- At most 64 concurrent connections. A 15-second idle budget includes TLS;
  HTTP bytes start a 10-second request budget, with two-second later I/O timeouts.
- TLS 1.3 only: ChaCha20-Poly1305/SHA256, X25519MLKEM768/X25519/P-256 and ECDSA P-256.
  No HRR, handshake reassembly, AES, ALPN selection, resumption, 0-RTT, KeyUpdate
  or received close_notify. The server sends encrypted close_notify and loads
  one certificate. Retained local record/AEAD proofs do not prove Bertie's layer.
- Checked on arm64 macOS and aarch64 Linux. x86_64 server builds and physical
  browser acceptance remain unchecked. Pinned inputs do not guarantee identical
  image bytes; build paths can appear in panic strings.
- Cargo-deny uses advisory snapshot 99670cfc0bc574433034b75876a4b791ccaa2efb
  (2026-10-27) and a matching frozen check clock. This is not current advisory
  status; offline checks also cannot detect registry yanks. Narrow build-time
  unmaintained exceptions are documented in [deny.toml](deny.toml).

[Verification guide](verification/README.md) ·
[Design history](EVOLUTION.md) · [Build records](dev/README.md)

### Shipped tree coverage

Run `nix build .#coverage` and open `result/report.md` or `result/report.json`.
Until v1.13.0 is published, replace the placeholder input at build time by adding
`--no-write-lock-file --override-input formal-coverage "git+file://$FC_CHECKOUT"`,
where `FC_CHECKOUT` is the absolute path to the part-1 checkout.

The informational Shipped tree counts runtime lines proven in their own package.
The report credits bertie-tls at the server's locked revision and enabled features.
Compiled dependency lines outside its receipt count as unverified. Cross-package
interfaces are outside the proof claim. The ratchet still uses first-party totals.

At the current lock on aarch64-darwin, the tree contains 302,681 lines.
The receipt moves 1,151 lines to tested and 25 to trusted. Bertie's
1,757 lines outside the receipt remain unverified. Both reports count
887 proven lines, all first-party.
