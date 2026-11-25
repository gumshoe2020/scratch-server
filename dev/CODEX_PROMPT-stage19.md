You are building stage 19 of scratch-server, `bertie-boundary`, in this directory (git branch
`bertie-boundary`).

`specs/development/BERTIE-BOUNDARY.md` is the authoritative spec. Read it fully first, then
README.md, verification/TRUSTED-BASE.md, src/tls.rs, src/transport.rs, crates/transport-core's io.rs and
spec.rs, flake.nix, deny.toml, verification/verify.sh, nix-build.sh,
verification/scripts/spec-mutants.py, the stage 17 measurement scripts and write-up, and the tails of
dev/PROGRESS.md and dev/NOTES.md. Read ../bertie-spike/REPORT.md, ../bertie-spike/server/src/main.rs
and ../bertie-spike/scripts/interop.py (read-only; do not modify bertie-spike).

Non-negotiables:
1. Stay on branch `bertie-boundary`. Do not check out or modify other branches. Do not `git push`.
   Commit at each milestone (M1, M2, M3, final) with the repo's configured identity (Dudley); do not
   override it.
2. Only the TLS layer changes. The Verus HTTP parser, transport state machine, dispatch, response
   formatting, deadlines and connection admission stay as they are. Never weaken, delete or
   `assume`/`admit` an existing proof, lemma, test or mutant; never add `external_body` to new
   attacker-facing logic in hello.rs. Proof counts may only go up.
3. Docker: the only long-lived container you may create, start, stop or replace is
   `bertie-boundary` on host port 8099. Read from `proven-io` (8098) and `bertie-spike` (8092) but
   never stop, restart, remove or reconfigure them or any other container. Ephemeral
   `docker run --rm` builds like nix-build.sh's are allowed; remove scratch images you create.
4. Write nowhere outside this directory except $TMPDIR and the existing tool caches (cargo, nix).
   Mount the existing TLS directory read-only; never print, copy into the repo or commit keys.
5. Redirect long build, verifier, fuzz and test output to files under logs/ (prefix `stage19-`) and
   read only `tail -n 30` or grep. Never use `nix build -L` interactively.
6. Keep dev/PROGRESS.md (a "Stage 19 — bertie-boundary" section with M1-M3 and Acceptance) and
   dev/NOTES.md (timestamped) current. Resolve ambiguity conservatively, log it, and keep going rather
   than stopping to ask. Report Bertie bugs or gaps in NOTES.md; do not patch Bertie.
7. If a milestone cannot pass, record exactly what fails and why in NOTES.md and PROGRESS.md, and
   commit what is sound. Do not claim acceptance you have not run.
8. Finish with a short summary: per-milestone result, proof counts (parser/transport/AEAD and the
   new hello module), mutants caught, test count, `cargo tree` rustls check, Nix check and image
   result, container status on 8099, byte-identical comparison result, interop table, in-binary line
   counts (Bertie vs stage 17 rustls), coverage numbers, new trusted functions, and anything not done.

The spec says `transport-core/src/hello.rs`; the crate lives at `crates/transport-core`, so the module goes in `crates/transport-core/src/hello.rs`.
