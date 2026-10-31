You are building stage 14 of scratch-server, `proven-dispatch`, in this directory:
<workspace> (git branch `proven-dispatch`).

`PROVEN-DISPATCH.md` is the authoritative spec. Read it fully first, then PROVEN-TRANSPORT.md
(the stage 8 pattern you are repeating), HEADER-POLICY.md, PROOFS.md, MUTANTS.md, PROGRESS.md and
NOTES.md. Reuse the existing tooling: ./verify.sh, ./nix-build.sh, scripts/acceptance.py,
scripts/spec-mutants.py, .tools/.

Non-negotiables:
1. Stay on branch `proven-dispatch`. Do not check out or modify other branches. Do not edit
   EVOLUTION.md or ATTACK-SURFACE.md.
2. Do not `git push`. Commit locally at sensible checkpoints (core proven, shell rewired,
   equivalence tests, deployment) and when the stage passes, or when blocked, with the blocker.
   Commits use this repo's configured identity (Dudley); do not override it.
3. Do not stop, remove or reuse the containers `scratch-server`, `nix-scratch`, `parser-http`,
   `parser-https` or `header-policy`. Your only container is `proven-dispatch` on port 8096. You
   may replace it while you iterate.
4. Do not run or modify the formal-coverage repository. Write nowhere outside this directory,
   except $TMPDIR and Docker. Mutant runs use temporary copies, never the working tree.
5. Never commit, print or log private keys or header values.
6. Don't change runtime behaviour: responses and log lines must stay byte-identical. If an
   equivalence test exposes a real bug in the old code, stop, commit the evidence and explain it
   in NOTES.md; don't fix it. Never weaken an existing proof, lemma or test to make results look
   better. Proof claims need a passing ./verify.sh. Never add assume, admit, external_body,
   external or unsafe.

Resolve small ambiguities by choosing reasonably and logging the choice in NOTES.md; don't stop.

Keep your context small: never use `nix build -L`. Redirect build, test and verifier output to
./logs/stage14-*.log and read only `tail -n 30`, or grep it for errors.

Observability: add a "Stage 14" section to PROGRESS.md with one checkbox per acceptance item, and
tick each one only after its command has passed. Append timestamped NOTES.md entries. Finish with a short
summary that includes: verified counts per Verus root, the list of moved/added functions with
proven-or-shell status and SLoC, the equivalence-test case counts, spec mutants killed out of 15,
the live-check results on 8096, and anything you could not prove and why.
