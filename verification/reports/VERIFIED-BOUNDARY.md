# VERIFIED-BOUNDARY.md: plan for a formally verified network boundary

Branch `verified-boundary`, built on `nix-scratch`. Status: approved by Andrew 2026-10-27; build
spec (milestones + acceptance) at the bottom. This file is authoritative for the build.

## Goal
A web app in a shell-free Nix image whose attack surface is only the app and its pinned
dependencies (done on `nix-scratch`), and where every interface that touches untrusted input
runs through formally verified code wherever that is available today.

"Formally verified" means proven to match a written spec. It does not cover a wrong spec, timing
side channels, or the compiler. It shrinks what must be trusted; it does not remove it.

## Where each interface stands (survey, 2026-10-27)

| Interface | Today in scratch-server | Verified option | Status |
|---|---|---|---|
| Cryptography | none yet (no TLS) | aws-lc-rs (rustls default) has formally verified implementations of several core algorithms; Cryspen libcrux; HACL* | available now |
| TLS protocol | none yet | rustls is memory-safe and audited, not verified. Verified TLS exists in research (Project Everest, F*) | not available for Rust in production |
| Parsing untrusted bytes | axum/hyper/httparse (fuzzed, not verified) | Vest: Verus-verified parsers/serializers for binary formats (incl. TLS messages). EverParse (C/F*) | partial; no verified HTTP text parser in Rust |
| Kernel / syscalls | host Linux kernel (shared) | seL4 is the only production-grade verified kernel | requires leaving Linux/Docker; research track |
| Our own code | unverified | Verus, Kani (AWS; used on Firecracker), Creusot, Prusti | available now |

Reminder of why logic matters, not just memory safety: rustls-webpki had a 2026 bug in
certificate revocation list matching (CVE-2026-93602). That class of bug is what a proof catches.

## The spike
Make every byte that arrives from the network pass through proven code before anything else sees it.

1. **Minimal HTTP request parser, verified.** Replace axum/hyper's request parsing with our own
   parser for a strict subset only: `GET`/`HEAD`, a short path from an allowed character set, a
   handful of known headers, hard size limits, no chunked bodies. Prove with Verus (or Kani) that it:
   never panics or reads out of bounds on any input; accepts exactly the grammar in the spec;
   enforces the limits. Anything outside the subset is rejected with 400 before other code runs.
2. **Fuzz against an oracle.** Differential-fuzz our parser against httparse on the subset: same
   accept/reject and same parsed fields. The oracle loop contributors describe.
3. **TLS with verified crypto.** Add rustls with the aws-lc-rs provider; document exactly which
   algorithms are covered by proofs and restrict the cipher suites to those. Evaluate a libcrux
   provider if one is mature.
4. **Proofs in the build.** `nix flake check` runs the Verus/Kani proofs, the fuzz smoke test and
   cargo-deny, so a change that breaks a proof fails the build.

### Acceptance
- `nix flake check` passes, including the proofs.
- Image stays static, shell-free, and under ~5 MB.
- The LAN test from `nix-scratch` still passes (page, `/health`, no `sh`).
- A malformed-request corpus is rejected with 400 and never reaches the handler.
- Written list of what is proven and what is still trusted (TLS state machine, kernel, compiler).

## Build spec (authoritative for Codex)

### Grounded facts (probed 2026-10-27; re-verify, don't assume)
- Verus is NOT in nixpkgs. Prebuilt releases exist for arm64-linux, arm64-macos, x86-linux,
  x86-macos (github.com/verus-lang/verus/releases; latest non-rolling `release/0.2026.10.26.d17fbd0`).
  Verus requires one specific Rust toolchain version; read its release notes/README for it.
- Kani is NOT in nixpkgs. Releases (kani-0.68.0) exist for aarch64/x86_64 Linux and macOS.
- nixpkgs has cargo-deny 0.20.2 and cargo-fuzz 0.13.2.
- The host is an arm64 Mac with Docker. Nix builds of Linux images run inside the `nixos/nix`
  container via `./nix-build.sh` (cache volume `scratch-server-nix-store`). Native `nix` on the Mac
  builds only aarch64-darwin.
- Already running, do not touch: containers `scratch-server` (port 8088) and `nix-scratch`
  (port 8089). Use port 8090 for this work's tests; container name `verified-boundary`.

### Milestones
**M1: verified request parser.** A small crate (e.g. `crates/http-subset`) implementing the strict
subset from step 1 above, written for Verus. Prove, for every input (unbounded, not just small
sizes): no panic, no arithmetic overflow, no out-of-bounds access; the parser accepts exactly the
inputs a spec function in the same file defines as valid, and the parsed fields equal the spec's;
all size limits hold. `./verification/verify.sh` runs the proofs and exits 0. If Verus proves unworkable for a
property, use Kani for that property, state the bound, and log why in NOTES.md. Write PROOFS.md:
each proven property in plain English with a file:line reference, and what is assumed/trusted.

**M2: server on the verified parser.** Every byte read from a socket goes through the M1 parser
before any other code sees it. Remove axum/hyper from the request path (tokio for TCP is fine).
Responses are written by our code. Keep `/`, `/health` and `--healthcheck` behaving as today.
Malformed or out-of-subset requests get 400 (or 414/431 for size limits) and never reach a
handler. Tests: a corpus of at least 30 malformed requests (in `tests/`) all rejected, with a
counter proving the handler ran 0 times; normal requests still work.

**M3: differential test against an oracle.** A proptest (or cargo-fuzz) harness comparing the
M1 parser with `httparse` on generated requests: on inputs inside the subset both must accept
with identical method/path/headers; on inputs outside it ours must reject. At least 100,000 cases,
0 mismatches. Commit any found counterexamples as regression tests.

**M4: TLS with verified crypto.** rustls with the aws-lc-rs provider; restrict cipher suites and
key exchange to ones whose AWS-LC implementations are covered by AWS's published formal
verification (cite the AWS-LC documentation URL for each; if you cannot confirm coverage from
a primary source, say "unconfirmed" rather than guess). Certificate and key are read from files
mounted at runtime; nothing secret is committed. Provide a script that makes a self-signed LAN
cert outside the repo. If the static musl Nix build of aws-lc-rs cannot be made to work, stop M4,
keep plain HTTP working, and write the exact blocker in NOTES.md. Do not silently switch to ring.

**M5: Nix + checks.** `./nix-build.sh image-static` builds and loads the image. Image stays
static with no shell (`docker export … | tar -t` shows only the binary plus any CA data you add),
under 8 MB. `nix flake check` (run inside the nixos/nix container) runs cargo-deny and the test
suite; it runs the Verus proofs too if Verus can be fetched as a pinned release in the flake on
aarch64-linux, otherwise NOTES.md explains why and `./verification/verify.sh` remains the proof entry point.
LAN test: run the image as container `verified-boundary` on port 8090 with `--read-only --cap-drop
ALL --security-opt no-new-privileges`; `/` and `/health` answer (over HTTPS if M4 landed); `docker
exec verified-boundary sh` fails. Leave the container running for Dudley's check.

## Later
- seL4 track: a native seL4 build of the same app (no Linux, no Docker). Needs a network stack
  and drivers on seL4; evaluate before committing.
- Same template for another project services once the spike shows the cost.
