You are building stage 13 of scratch-server, `spec-lemmas`, in this directory:
<workspace> (git branch `spec-lemmas`).

`SPEC-LEMMAS.md` is the authoritative spec: read it fully first, then HEADER-POLICY.md, PROOFS.md,
PROGRESS.md and NOTES.md. Reuse existing tooling (./verification/verify.sh, ./nix-build.sh,
scripts/acceptance.py, verification/scripts/measure-coverage.py, .tools/).

Non-negotiables:
1. Stay on branch `spec-lemmas`. Do not check out or modify other branches. Do not edit
   EVOLUTION.md or ATTACK-SURFACE.md.
2. Do not `git push`. Commit locally at sensible checkpoints and when the stage passes (or when
   blocked, with the blocker). Commits use this repo's configured identity (Dudley); do not
   override it.
3. Do not stop, remove or reuse containers `scratch-server`, `nix-scratch`, `parser-http`,
   `parser-https` or `header-policy`. This stage needs no deployment.
4. Read <workspace> only if a test needs it;
   never write there. Use <workspace> read-only
   (building in place is fine). Mutant runs use temporary copies under ./target or $TMPDIR, never
   the working tree. Write nowhere else outside this directory.
5. Never commit, print or log private keys or header values.
6. Don't change runtime behaviour. If a lemma exposes a real bug in the spec or the code, stop,
   commit the evidence and explain it in NOTES.md; don't fix it. Never weaken a lemma or drop a
   mutant to make results look better. Proof claims need a passing ./verification/verify.sh. Never add assume,
   admit, external_body, external or unsafe.

Resolve small ambiguities by choosing reasonably and logging the choice in NOTES.md; don't stop.

Keep your context small: never use `nix build -L`. Redirect output to ./logs/ and read only
`tail -n 30`, or grep it for errors.

Observability: add a "Stage 13" section to PROGRESS.md, and tick items only when their commands
passed. Append timestamped NOTES.md entries. Finish with a short summary that includes: verified
counts per proof group, the number of intent lemmas, the mutants killed out of the total, the
survivors with their resolution, and the formal-coverage number.
