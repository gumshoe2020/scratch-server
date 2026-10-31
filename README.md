# scratch-server

A small HTTPS server exploring how much of a network boundary can be proved.
This is stage 16 (`proven-io`): a strict GET/HEAD HTTP subset, configurable
header policy, and a Verus core covering the parser, transport state, response
formatting, dispatch and diagnostics, TLS record helpers, read deadlines and
connection admission, plus intent lemmas and deliberate spec mutants. The [stage index](EVOLUTION.md)
shows the earlier designs. [ATTACK-SURFACE.md](ATTACK-SURFACE.md) names the remaining trusted pieces and [TRUSTED-BASE.md](TRUSTED-BASE.md) lists every unproven production function. The historical build record lives in [dev/](dev/README.md).

## Threat model and assurance

Network peers may send arbitrary bytes, malformed requests, slow reads or excess
connections. The handler sees method/path only, never headers. The parser caps
requests at 4096 bytes, headers at 24, names at 64 bytes, values and paths at 256
bytes. It rejects bodies, query strings, percent escapes, pipelining, duplicate
Host/Connection and hard-reject headers. Unknown well-formed headers are dropped
by default; guarded headers reject unless explicitly configured to drop.
[HEADER-POLICY.md](HEADER-POLICY.md) defines the policy and configuration grammar.

Proven against written specs: parser acceptance and bounds for all inputs,
transport event/byte state transitions, response formatting, selected AEAD
helpers, routing, rejection diagnostics, log sanitizing and the unknown-header
log limiter ([PROVEN-DISPATCH.md](PROVEN-DISPATCH.md)), TLS record unpadding,
AAD, inner-plaintext and key-share shape checks
([PROVEN-RECORD.md](PROVEN-RECORD.md)), and read deadlines, read
classification, connection admission and port parsing
([PROVEN-IO.md](PROVEN-IO.md)). Intent lemmas check non-vacuity, attacks and universal properties of the
spec. See [PROOFS.md](PROOFS.md), [INTENT.md](INTENT.md) and
[VERIFIED-BOUNDARY.md](VERIFIED-BOUNDARY.md). Proofs cannot establish that the spec
captures every security requirement. Fifteen [spec mutants](MUTANTS.md) exercise
specific mistakes, with refinement proofs checked before intent/test failures.

Tested: 100,000 differential inputs under three policies, 10,000 generated
configs, malformed requests, real loopback TLS, fragmentation, deadlines and
exact route dispatch. Trusted: I/O adapters, clocks, threading, atomic
connection counters, rustls protocol logic, X.509/key parsing, OS randomness, compiler, verifier,
kernel and container runtime. A shell-free image still depends on the host kernel.

## Build and run

Docker is required for Linux Nix builds; Rust and rustup are required for native
checks. Run from this checkout. Nix inputs, Verus and the Nix runner are pinned.

```sh
./nix-build.sh image-static > build.log 2>&1
# Create an existing private directory outside the checkout; no keys enter the image.
TLS_DIR="$(mktemp -d "${TMPDIR:-/tmp}/release-tls.XXXXXX")"
python3 scripts/make-lan-cert.py "$TLS_DIR" localhost
# On Linux, give the container uid access without making the key world-readable.
sudo chown -R 65532:65532 "$TLS_DIR"
sudo chmod 700 "$TLS_DIR"
sudo chmod 600 "$TLS_DIR/key.pem" "$TLS_DIR/cert.pem"
docker run -d --name scratch-server -p 127.0.0.1:8443:8080 \
  --read-only --cap-drop ALL --security-opt no-new-privileges \
  --mount type=bind,src="$TLS_DIR",dst=/tls,readonly \
  --mount type=bind,src="$PWD/config/header-policy.conf",dst=/config,readonly \
  -e SCRATCH_HEADER_POLICY=/config scratch-server:static
curl -kfsS https://localhost:8443/health
# Wait until this reports healthy before running acceptance.py.
docker inspect --format '{{.State.Health.Status}}' scratch-server
# On native Linux, run this helper with sudo after the ownership change above.
python3 scripts/acceptance.py --container scratch-server \
  --host localhost --port 8443 --tls-dir "$TLS_DIR" > acceptance.log 2>&1
docker rm -f scratch-server
sudo rm -rf "$TLS_DIR"
```

Docker Desktop may already make the private mount readable to the container;
use the ownership commands above on a native Linux Docker host.

The image runs as uid/gid 65532, has no shell or shared libraries and provides
its own `--healthcheck`. `SCRATCH_PORT` defaults to 8080, `SCRATCH_TLS_DIR` to
`/tls`. Missing/invalid TLS material fails startup with no HTTP fallback.
The self-signed certificate triggers a browser warning; `curl -k` is for this
local example. The certificate helper accepts a hostname or IP as its second
argument. For LAN tests supply that host and `--lan-host HOST` to acceptance.py.

`SCRATCH_HEADER_POLICY` selects a required config file; without it the built-in
default applies. Invalid/unreadable config fails before binding. See
[config/header-policy.conf](config/header-policy.conf) and the strict variant.
At most 64 connections run concurrently. An absolute 15-second budget includes
TLS handshaking before the first HTTP byte; expiry closes silently. Once the
first request byte arrives, the whole request must finish within 10 seconds
(otherwise `400`, logged `reason=slow`), with at most two seconds between reads.
Writes have two-second timeouts. Each connection serves one request.
Logs retain sanitized names/status, never values; unknown-name logging is bounded.

## Verify

```sh
./verify.sh > proofs.log 2>&1
CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" \
  cargo test --locked --workspace > tests.log 2>&1
python3 scripts/spec-mutants.py > mutants.log 2>&1
./nix-build.sh check > checks.log 2>&1
scripts/check-public.sh /path/to/private-denylist.txt
```

Proof groups have 211/237/9 verified obligations and zero errors; groups overlap.
Both image targets depend on the same proof and cargo-deny derivations as
`nix flake check`, alongside the binary's tests. Logs stay under `logs/`.
`./nix-build.sh proof-log`, `test-log` and `deny-log` retrieve retained evidence.

## Limits

TLS is TLS 1.3 only: ChaCha20-Poly1305/SHA256, X25519 and ECDSA P-256, via a custom
libcrux rustls provider. Upstream primitive verification claims are distinguished
from locally checked proofs in [TLS-PROVIDER.md](TLS-PROVIDER.md). This is not a
verified TLS protocol or a proof of the whole server. Historical AWS-LC blockers
in [TLS-COVERAGE.md](TLS-COVERAGE.md) describe an earlier stage.

Only aarch64 Linux and native macOS have been checked. x86_64 has never been
built. Physical browser and second-device observations come only from the
author's sessions; automated loopback fixtures do not reproduce those sessions.
The Nix build is not bit-reproducible: build-directory paths appear in panic
strings. Reproducible inputs do not imply identical image bytes.

Cargo-deny uses advisory-db revision `99670cfc0bc574433034b75876a4b791ccaa2efb`
(2026-10-27), with its clock frozen by libfaketime to the snapshot timestamp.
File timestamps stay real (`NO_FAKE_STAT=1`). Before cargo-deny runs, a UTC date
assertion rejects a missing or ineffective preload.
This makes the 14-day freshness check deterministic for a commit; it does not
claim current advisory status. To refresh, update the archive revision and NAR hash (`fetchFromGitHub`),
FETCH_HEAD timestamp and frozen clock timestamp together in flake.nix, then run
`./nix-build.sh check` and review advisory changes. Offline checks cannot detect
registry yanks. The documented cfg(hax)-only advisory exception remains in
[deny.toml](deny.toml). All workspace and third-party license grants are MIT or
among the explicitly allowed licenses; dependency provenance remains checked.

## First-party proof coverage

First-party security-relevant code: **644 of 1,363 SLoC proven (47.25%)**, measured by
[formal-coverage v1.4](https://github.com/gumshoe2020/formal-coverage) at `2fa5eb7`;
300 tested, 419 unverified. Counting rule: package source SLoC, with the security
boundary defined by formal-coverage's scratch-server config. Shipped third-party
runtime dependencies: **336,654 SLoC (not proven)**, for aarch64-unknown-linux-musl.
This is the stage-13 baseline measurement, not a new measurement of this release pass.

The historical measurement helper requires an explicit formal-coverage checkout:
`python3 scripts/measure-coverage.py /path/to/formal-coverage`. It retains the v1
three-receipt union method for auditing older records; its output is not the v1.4
baseline above. No formal-coverage checkout is modified by the helper.
