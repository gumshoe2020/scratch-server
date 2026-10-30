# Evolution

The project was built in 13 stages, each on top of the one before. This repository holds the
latest stage (13, `spec-lemmas`); the table below records what each earlier stage changed and
what was measured at the time. Each stage's design notes are kept in this tree and linked in the
last column.

The ports below are historical example deployment ports. Choose an available local
port when running a stage. Browser observations are from the author's own sessions;
see the [README](README.md) for current TLS and verification limits.

| # | Port | Stage | What changed | Result | First-party proof coverage | Design notes |
|---|---|---|---|---|---|---|
| 1 | 8088 | initial | Rust web server in a two-stage Dockerfile ending `FROM scratch` | one static binary in the image; no shell, interpreter or package manager; 1.2 MB image | v1.4 remeasure pending | — |
| 2 | 8089 | `nix-scratch` | `flake.nix` replaces the Dockerfile; image = the binary's runtime closure | static: one file, 2.8 MB. glibc variant: ~1,000 files, 125 MB (why static linking matters). Pinned by `flake.lock` | v1.4 remeasure pending | — |
| 3 | 8090 | `verified-boundary` | Every untrusted byte through formally verified code (see `VERIFIED-BOUNDARY.md`, `PROOFS.md`) | **M1:** strict HTTP request parser proven with Verus for every input: no panic/overflow/out-of-bounds, accepts exactly the written grammar, limits hold. 18 verified, 0 errors; no assume/admit/external_body (re-run by Dudley). **M2 done:** axum/hyper/tokio removed; every byte from the socket goes through the proven parser before anything else; 48 malformed requests (at 3 read sizes) all get 400 with the handler called 0 times. **M3 done:** 100,000 generated requests, 0 mismatches: 50,000 valid ones parsed identically to httparse, field for field; 50,000 mutated/random ones classified the same as an independent grammar checker; exact limit edges (256/257, 16/17, 4096/4097) tested. **M4 (HTTPS) blocked:** AWS-LC's published proofs don't cover a complete TLS cipher suite on ARM (see `TLS-COVERAGE.md`); plain HTTP kept. **M5:** `nix flake check` runs proofs, tests and cargo-deny (image builds were gated on tests alone until the release fix on `spec-lemmas`); static image 1.3 MB (binary 649 KB, no interpreter, no shared libraries); hardened container passes; browsers get 400 by design (they send headers outside the 4 allowed) | v1.4 remeasure pending | [VERIFIED-BOUNDARY.md](VERIFIED-BOUNDARY.md), [PROOFS.md](PROOFS.md), [TLS-COVERAGE.md](TLS-COVERAGE.md) |
| 4 | 8090 | `browser-whitelist` | Header whitelist grows from 4 to 9 names, taken from a captured iPhone Safari request (see `BROWSER-WHITELIST.md`); rejected requests log the unknown header name only, never values | Real browsers load the page. Proofs re-run against the 9-name spec: 18 verified, 0 errors, no escape hatches. 66 malformed cases (incl. wrong-case and near-miss names) → 400, handler called 0 times; 100,000 differential cases, 0 mismatches; `Cookie: secret` logged as `unknown_header=Cookie` with no value; image 1.35 MB | v1.4 remeasure pending | [BROWSER-WHITELIST.md](BROWSER-WHITELIST.md) |
| 5 | 8091 | `https` | TLS 1.3 on a custom rustls crypto provider built only from libcrux (HACL*-derived, F*-verified) crates; one suite, one key exchange, HTTP/1.1 only (see `HTTPS.md`, `TLS-PROVIDER.md`) | Negotiates TLS 1.3 / ChaCha20-Poly1305 / X25519 / ECDSA P-256; TLS 1.2, AES suites, P-256 key exchange and h2 refused; plain HTTP refused. Crypto primitives verified upstream (HACL*/F*); rustls handshake, X.509/key encoding, OS randomness and glue code trusted. Parser unchanged, proofs 18/0. Cert/key outside repo and image, mounted read-only. Image 3.1 MB | v1.4 remeasure pending | [HTTPS.md](HTTPS.md), [TLS-PROVIDER.md](TLS-PROVIDER.md) |
| 6 | 8091 | `https-headers` | Whitelist grows from 9 to 12: Safari's HTTPS-only `Sec-Fetch-Dest`, `Sec-Fetch-Site`, `Sec-Fetch-Mode`, taken from a captured iPhone request (names only; see `HTTPS-HEADERS.md`) | iPhone Safari loads the page over HTTPS. Proofs 18/0 against the 12-name spec, no escape hatches; 100,000 differential cases, 0 mismatches; lowercase and unlisted (`Sec-Fetch-User`) still 400; TLS policy unchanged from stage 5 | v1.4 remeasure pending | [HTTPS-HEADERS.md](HTTPS-HEADERS.md) |
| 7 | 8091 | `idle-connections` | Transport fix (see `IDLE-CONNECTIONS.md`): iPhone Safari got 400 on HTTPS because it preconnects and the server answered a 2-second idle timeout with 400. Now: up to 15 s idle before the first byte, then a silent close; 2 s between reads once a request starts; every rejection logs its reason | Delayed request (3 s idle) → 200; idle 17 s → closed with 0 bytes, `closed reason=idle`; half request then stall → 400 `reason=slow`. Parser, proofs (18/0), whitelist and TLS policy unchanged | v1.4 remeasure pending | [IDLE-CONNECTIONS.md](IDLE-CONNECTIONS.md) |
| 8 | 8091 | `proven-transport` | Connection code split into a Verus-proven core (state machine + response formatting + AEAD nonce/length helpers) and a thin unverified I/O shell (see `PROVEN-TRANSPORT.md`, `PROOFS.md`) | Proven: buffer bound, dispatch only on full parser acceptance and at most once, terminal states, rejection classes, bounded steps, Content-Length/no header injection/HEAD. Still unverified: I/O shell, threads/clock, most rustls provider glue, rustls itself | v1.4 remeasure pending | [PROVEN-TRANSPORT.md](PROVEN-TRANSPORT.md) |
| 9 | 8091 | `firefox-headers` | Whitelist 12 → 13: Firefox's `Sec-Fetch-User`, from a captured Windows Firefox request (names and lengths only; see `FIREFOX-HEADERS.md`) | Firefox loads over HTTPS. Proofs 18/0, 43/0, 9/0; no escape hatches; lowercase `sec-fetch-user` still 400. | v1.4 remeasure pending | [FIREFOX-HEADERS.md](FIREFOX-HEADERS.md) |
| 10 | 8091 | `chromium-headers` | Header names matched case-insensitively (ASCII folding, as HTTP defines); whitelist 13 → 18 (`Cache-Control`, `Sec-GPC`, `Sec-CH-UA`, `Sec-CH-UA-Mobile`, `Sec-CH-UA-Platform`, from a captured Brave request); header cap 16 → 24 (see `CHROMIUM-HEADERS.md`) | Brave/Chromium loads. Proofs 21/0, 46/0, 9/0; no escape hatches. `sec-ch-ua`/`HOST` accepted; `content-length`, `COOKIE`, `Sec-Fetch-Users` rejected; 24 headers OK, 25 rejected. | v1.4 remeasure pending | [CHROMIUM-HEADERS.md](CHROMIUM-HEADERS.md) |
| 11 | 8091 | `pragma-header` | Whitelist 18 → 19: `Pragma`, observed in the server log when Brave ran with DevTools "Disable cache" (see `PRAGMA-HEADER.md`) | Brave with cache disabled loads. Proofs pass; no escape hatches; `Pragmas` rejected. | v1.4 remeasure pending | [PRAGMA-HEADER.md](PRAGMA-HEADER.md) |
| 12 | 8091 | `header-policy` | Explicit whitelist → configurable proven header policy, after Brave sent `dnt` (likely Privacy Badger): 9 hard-reject names fixed in code, 16 guarded names reject by default (config may set drop), unknown well-formed names dropped and logged; `unknown reject` = strict mode. Duplicate Host/Connection rejected. Handler sees only method and path (see `HEADER-POLICY.md`) | Brave with DNT loads. Proven: config cannot loosen hard-reject names; strict mode = stage 11 minus duplicates; no dropped header reaches the handler. Proofs 56/82/9, no escape hatches; bad config refuses to start. | v1.4 remeasure pending | [HEADER-POLICY.md](HEADER-POLICY.md) |
| 13 | — | `spec-lemmas` | Checks the spec against intent (no runtime change). 42 proven intent lemmas: captured Safari/Firefox/Brave requests accepted (non-vacuity), ~25 classic attacks rejected, and "never" statements over every input and every valid config; INTENT.md lists each in plain English. 15 deliberate spec+code mutants, each consistent with the refinement proofs (see `SPEC-LEMMAS.md`) | 15/15 mutants caught (13 by intent proofs, 2 by fixture/config tests), 0 survivors; Dudley re-ran Transfer-Encoding and Cookie mutants independently. `/../` paths get 404, no filesystem access (tested + source review). Proofs 211/237/9, no escape hatches. | v1.4 remeasure pending | [SPEC-LEMMAS.md](SPEC-LEMMAS.md), [INTENT.md](INTENT.md), [MUTANTS.md](MUTANTS.md) |

Per-stage first-party coverage is v1.4 remeasure pending; no historical tool figures
are comparable until remeasured with the same counting rule.

First-party security-relevant code: **644 of 1,363 SLoC proven (47.25%)**, measured by
[formal-coverage v1.4](https://github.com/gumshoe2020/formal-coverage) at `2fa5eb7`;
300 tested, 419 unverified. Counting rule: package source SLoC, with the security
boundary defined by formal-coverage's scratch-server config. Shipped third-party
runtime dependencies: **336,654 SLoC (not proven)**, for aarch64-unknown-linux-musl.
This is the stage-13 baseline measurement, not a new measurement of this release pass.

## Goal
See [ATTACK-SURFACE.md](ATTACK-SURFACE.md) for what attack surface remains and the plan for each piece.

A web application in Docker with no attack surface except the app itself and its pinned
dependencies, then with every interface to untrusted input limited to formally verified code
where that exists.

## Ideas (not scheduled)
- **`/tell-dudley` easter egg** (Andrew, 2026-10-27): a route that answers "Captured, thanks. Tell Dudley." like the header catcher did. A new route is new security-relevant code, so it gets measured like everything else.

The release pass on `spec-lemmas` gates both Nix image targets on proofs and dependency
checks as well as tests. Earlier stages gated their images on tests alone.
