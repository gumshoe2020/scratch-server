You are building stage 17 of scratch-server, `rustls-surface`, in this directory:
<workspace> (git branch `rustls-surface`).

`RUSTLS-SURFACE.md` is the authoritative spec. Read it fully first, then TLS-PROVIDER.md,
PROVEN-IO.md, TRUSTED-BASE.md, ATTACK-SURFACE.md, and the tail of PROGRESS.md and NOTES.md. Reuse
the existing tooling where it fits: ./verify.sh, ./nix-build.sh, .tools/.

Non-negotiables:
1. Stay on branch `rustls-surface`. Do not check out or modify other branches. Do not edit
   EVOLUTION.md or ATTACK-SURFACE.md.
2. Do not `git push`. Commit locally at sensible checkpoints (M1 scripts and numbers, fuzz targets
   building, fuzz results) and when the stage passes, or when blocked, with the blocker. Commits use
   this repo's configured identity (Dudley); do not override it.
3. Do not start, stop, remove or reuse any Docker container. This stage deploys nothing. Docker
   builds for measurement are allowed if needed; remove any images you create.
4. Do not run or modify the formal-coverage repository. Write nowhere outside this directory,
   except $TMPDIR, ~/.cargo, ~/.rustup (a nightly toolchain and cargo-fuzz are allowed) and Docker.
   Do not touch the TLS key directory.
5. Never commit, print or log private keys, traffic secrets or plaintext record contents.
6. No runtime behaviour change. If fuzzing finds a crash or a real bug in our code or in rustls,
   stop that target, commit the minimised evidence and explain it in NOTES.md; don't fix it. Never
   weaken an existing proof, lemma or test. Never add assume, admit, external_body, external or
   unsafe to production code.
7. Another Codex run is working on this Mac at the same time. Keep fuzzing to at most 8 jobs in
   total and do not run more than one heavy build (Nix or Verus) at a time.

Resolve small ambiguities by choosing reasonably and logging the choice in NOTES.md; don't stop.

Keep your context small: never use `nix build -L`. Redirect build, test, fuzz and verifier output
to ./logs/stage17-*.log and read only `tail -n 30`, or grep it for errors.

Observability: add a "Stage 17" section to PROGRESS.md with one checkbox per acceptance item, and
tick each one only after its command has passed. Append timestamped NOTES.md entries. Finish with a
short summary that includes: the M1 table (crate total / compiled / in-binary lines and functions
per crate, and in-binary lines per attacker-facing module), unused compiled features, fuzz
executions and corpus size per target, rustls lines reached by fuzzing out of in-binary lines, any
findings, and confirmation that verify.sh and cargo test results are unchanged.
