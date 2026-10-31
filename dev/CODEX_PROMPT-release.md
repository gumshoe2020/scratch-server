Implement PLAN-RELEASE.md in this repository. That plan is authoritative.
- Read the plan, README.md, VERIFIED-BOUNDARY.md, SPEC-LEMMAS.md, MUTANTS.md, flake.nix,
  nix-build.sh and verification/verify.sh before starting. Re-probe the grounded facts you rely on.
- Code work happens on `spec-lemmas` (checked out here). For the `main` docs work, use a separate
  worktree: `git worktree add "$TMPDIR/scratch-server-main" main`; remove it when done.
- Hard fence: never weaken a proof, spec, test or mutant; no assume/admit/external_body. Don't
  touch containers or images you didn't create; name yours release-* and clean them up.
- Commit only in this repository, one commit per milestone per branch. Don't push, don't rewrite
  history, don't edit the other stage branches. The git identity is already set; don't change it.
- Redirect every build log (nix, cargo, verus, docker) to a file and read only `tail -n 30` or
  grep. Never use `nix build -L`.
- Keep PROGRESS.md (add a "Release pass" section with M1-M4 and the final check) and NOTES.md
  (timestamped) current. Resolve ambiguity conservatively, log it, don't stop to ask.
