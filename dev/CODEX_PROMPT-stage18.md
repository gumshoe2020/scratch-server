You are building stage 18 of scratch-server, `spec-layout`, in this directory (git branch
`spec-layout`).

`SPEC-LAYOUT.md` is the authoritative spec. Read it fully first, then README.md, EVOLUTION.md,
flake.nix, verification/verify.sh, nix-build.sh, verification/scripts/spec-mutants.py and the tails of dev/PROGRESS.md and
dev/NOTES.md.

Non-negotiables:
1. Stay on branch `spec-layout`. Do not check out or modify other branches. Do not `git push`.
   Commit at each milestone with the repo's configured identity (Dudley); do not override it.
2. This is a layout change. No runtime behaviour change, no edits to spec meaning, no weakened or
   deleted proofs, lemmas, tests or mutants. Use `git mv` so history follows files.
3. Do not start, stop, remove or reuse any long-lived Docker container. Ephemeral `docker run --rm`
   builds like nix-build.sh's are allowed; remove images you create.
4. Write nowhere outside this directory except $TMPDIR and the existing tool caches. Do not touch
   the TLS key directory outside the repo. Never print or commit keys.
5. Redirect long build/verifier/test output to files under logs/ and read only tails or grep.
6. Keep dev/PROGRESS.md (stage 18 section, M1-M4) and dev/NOTES.md (timestamped) current. Resolve
   ambiguity conservatively, log it, and keep going rather than stopping to ask.
7. Finish with a short summary: per-milestone result, proof counts, test count, both binary sha256s
   from M1 (and the M2 binary comparison), link-check and catalog-check results, and anything not done.
