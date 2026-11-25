You are unblocking stage 19 of scratch-server (`bertie-boundary`). Working directory:
~/src/scratch-server (branch `bertie-boundary`, head after this prompt's commit).

Background. Run 19b (dev/CODEX_PROMPT-stage19b.md; results in specs/development/BERTIE-BOUNDARY.md
"Results — 2026-11-21" and the tail of dev/NOTES.md) proved the DER fix: 57 tests, 10,000
handshakes, 0 failures, proofs 211/347/9 + hello 30, 20/20 mutants. Release is blocked only by the
cargo-deny advisory gate (logs/stage19b-deny-log.log):
- 11 vulnerability advisories, all with fixed versions: libcrux-kem 0.0.2 (>=0.0.10),
  libcrux-sha3 0.0.2 (>=0.0.10), libcrux-chacha20poly1305 0.0.2 (>=0.0.8), libcrux-ed25519 0.0.3
  (>=0.0.7), libcrux-ecdh 0.0.2 (>=0.0.6), libcrux-poly1305 0.0.2 (>=0.0.5), crypto-bigint 0.5.5
  (>=0.7.6).
- 2 unmaintained crates with no upgrade: paste 1.0.15 (RUSTSEC-2024-0436), proc-macro-error 1.0.4
  (RUSTSEC-2024-0370).
Andrew approved (2026-11-20): update Bertie's libcrux dependencies on a new Bertie branch, and add
narrow, documented cargo-deny exceptions for the two unmaintained crates only.

Read first: this prompt; dev/CODEX_PROMPT-stage19.md and -stage19b.md (their non-negotiables still
apply unless changed here); the spec; the tails of dev/PROGRESS.md and dev/NOTES.md; deny.toml;
flake.nix; ~/src/bertie-der/{PLAN.md,REPORT.md,NOTES.md} and its scripts/ (read-only:
they show how Bertie's tests and the hax/F* DER proof were run, in Docker).

## B1: Bertie dependency update (Bertie repo)
1. Create a new worktree and branch from the current pin, without touching any existing branch or
   worktree:  `git -C ~/src/bertie-der/bertie worktree add -b deps/libcrux-advisories
   ~/src/bertie-deps 8ff624adda8b74c8c0edbec29b7c92492164233f`.
   Work only in ~/src/bertie-deps. Commit with that repo's configured identity; do not
   change it. Never push, never force anything, never modify other branches.
2. Raise Bertie's libcrux requirements to the smallest releases that clear every listed libcrux
   advisory (check crates.io; keep `0.0.x` pins exact-minor as upstream does). Adapt Bertie's code
   to any API changes with the minimum diff. No behaviour change, no new features, no removed
   checks, no changed default features. Do not touch src/der.rs or the DER proof except where an
   import path forces it.
3. crypto-bigint 0.5.5 comes in through RustCrypto (elliptic-curve / p256 0.13, used by Bertie and
   by scratch-server). Find out exactly which crates pull it in. If clearing it needs a RustCrypto
   major bump that is only available as a pre-release, or a large rewrite, STOP on that item:
   record the dependency path, the advisory text, and whether our code reaches the affected API,
   and leave it failing. Do not add an exception for it; that is Andrew's call.
4. Bertie gates: `cargo test` for the workspace members we use (record which), the Bertie
   handshake tests, and the hax 0.4.2 extraction + F* check of Bertie.Der and Bertie.Tls13formats
   using the bertie-der scripts' method (zero errors, no assume/admit/lax). If the F* check of any
   module whose extraction changed fails because of the libcrux bump, record it precisely.
5. Commit B1 in the Bertie worktree. Write the Bertie-side summary in scratch-server at
   dev/BERTIE-DEPS-REPORT.md (versions before/after, API changes, gate results).

## B2: scratch-server pin + exceptions
1. Move the pin to the B1 rev:
   `git = "file://~/src/bertie-deps", rev = "<B1 sha>"`, keep `version = "=0.1.0-pre.2"`,
   update Cargo.lock minimally (`cargo update -p bertie` plus only the crates that must move), and
   set deny.toml's allow-git to that one URL.
2. In deny.toml add exactly two advisory ignores, RUSTSEC-2024-0436 and RUSTSEC-2024-0370, each
   with a `reason` naming the crate, that it is unmaintained (not a vulnerability), and which crate
   pulls it in. First confirm with `cargo tree -i` that each is a build-time proc-macro dependency
   only, not linked into the server binary; if either is linked into the production binary, do
   not add its ignore and record why. No other ignores, no snapshot change, no clock change.
3. Rerun the native gates: verify.sh (counts may only go up from 211/347/9, hello 30), the 20
   mutants, `cargo test --locked --workspace --no-fail-fast`, the production cargo tree rustls
   check, the 10,000-handshake loop (scripts/stage19b-handshakes.py), and cargo-deny.
   Commit B2.

## B3: release and live (only if cargo-deny passes in Nix)
Carry out the spec's M3 items 2-4 and the 19b prompt's item D unchanged: Nix check and image-static,
container `bertie-boundary` on 8099 (hardened as specified, left running only if acceptance passes),
temporary `proven-io-ref` on 127.0.0.1:8098 from `proven-io:static` for the 18 byte-identical
requests (remove it afterwards), the interop matrix with certificate verification on, and the
stage 17 in-binary audit of the shipped image. If cargo-deny still fails (e.g. crypto-bigint), do
NOT build or run an ungated image; record B3 as blocked and why.

## Records
- Spec: add "Results — 2026-11-21 (libcrux update)" after the 19b results; mark 19b's heading
  superseded. Same table as before, plus dependency versions before/after and the two exceptions.
- dev/PROGRESS.md: a "Stage 19 — dependency update" subsection with B1, B2, B3, Acceptance.
- dev/NOTES.md: timestamped entries as you go.
Commits: Bertie B1 in ~/src/bertie-deps; scratch-server after B2, B3 and final, as Dudley.
No pushes anywhere. Noisy output to logs/stage19c-*.log; read only tails.

Finish with a short summary: per-milestone result, dependency versions before/after, API changes,
Bertie gate results, cargo-deny result and exceptions added, proof counts, mutants, tests,
handshake loop, Nix/image, container status, byte-identical result, interop table, in-binary
lines, and anything left for Andrew.
