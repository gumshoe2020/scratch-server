You are resuming stage 19 of scratch-server, `bertie-boundary`, in this directory (git branch
`bertie-boundary`, head dac4d41). The first run (dev/CODEX_PROMPT-stage19.md) implemented M1-M3 but
acceptance was blocked by an upstream Bertie bug: ECDSA CertificateVerify signatures were sometimes
emitted with invalid DER (see the "Results — 2026-11-01" section of the spec, dev/NOTES.md and
dev/PROGRESS.md). That bug is now fixed and verified on a separate Bertie branch. Your job is to
move the pin to the fixed Bertie and carry stage 19 through acceptance.

`specs/development/BERTIE-BOUNDARY.md` is still the authoritative spec, with the changes below.
Read it fully first, then dev/CODEX_PROMPT-stage19.md (its non-negotiables 1-8 all still apply,
except where this prompt changes them), the tails of dev/PROGRESS.md and dev/NOTES.md, and
`git log --oneline spec-layout..bertie-boundary`.

Changes to the spec:
A. New Bertie pin. The fixed Bertie is the local git repo ~/src/bertie-stage19, branch
   `integration/stage19-v2`, rev 8ff624adda8b74c8c0edbec29b7c92492164233f. It is the old pin
   (fbbcc13, the encrypted close_notify fix) plus two commits: the verified DER codec for ECDSA
   signatures and its use in CertificateVerify. This rev is NOT on GitHub yet; do not push it or
   anything else anywhere. Point the dependency at it as
   `git = "file://~/src/bertie-stage19", rev = "8ff624adda8b74c8c0edbec29b7c92492164233f"`
   and update Cargo.lock. Read-only use of that repo: never commit, branch or check out in it.
   Use the same Bertie features as before and log them.
B. Nix. Try to make the flake check and image-static build work with that local git source
   (e.g. cargoLock outputHashes / fetchGit of a local URL with an explicit rev). If Nix in pure mode
   cannot fetch it, record exactly what fails and stop that line of work: do NOT vendor Bertie into
   this repo, copy its source in, use --impure in the committed flake, or weaken any gate to get
   around it. Andrew will push the branch to his fork in the morning and the pin can then move to
   the GitHub URL with the same rev. In that case, steps that need the image (container,
   live comparison, interop, in-binary audit of the image) are recorded as "pending push", and
   everything else still has to pass natively.
C. The DER regression test added in M2/dac4d41 must now pass with its expected-success assertion
   unchanged. Additionally run a native loop of at least 10,000 Bertie handshakes against a strict
   DER-validating client (the dac4d41 detector or equivalent) and record failures (expected: 0).
D. Reference server for the byte-identical comparison. The `proven-io` container on 8098 is not
   currently running. You may create ONE temporary reference container named `proven-io-ref` from
   the existing image `proven-io:static`, bound to 127.0.0.1:8098 only, hardened exactly like the
   spec says for `bertie-boundary`, with the existing TLS directory mounted read-only. Remove it
   (and only it) when the comparison is done. Do not rebuild or retag `proven-io:static`.
   `bertie-boundary` on 8099 is still the only long-lived container you may create; leave it running
   if acceptance passes, and stopped/removed if it does not.
E. Rerun everything acceptance requires from scratch: verify.sh (counts may only go up from
   211/347/9 and hello 30), all 20 mutants (15 existing + 5 hello) caught, `cargo test --locked
   --workspace --no-fail-fast`, the production `cargo tree` rustls check, Nix check and image, the
   container, the 18 byte-identical requests, the interop matrix (certificate verification on), the
   stage 17 in-binary measurement, and formal-coverage if the executable is available (if not,
   say so; do not install it).
F. Results: keep "Results — 2026-11-01 (blocked; not accepted)" as history and mark it superseded
   at its heading. Add "Results — 2026-11-21" below it with the same table plus the 10,000-handshake
   result, and update the browser checklist. Add a "Stage 19 — rerun on fixed Bertie" subsection
   to dev/PROGRESS.md with R1 (pin + native tests), R2 (Nix + image), R3 (container + live +
   measurements), Acceptance.

Commits: commit after R1, R2, R3 and final, as Dudley (the repo's configured identity). Do not push.
Stay on `bertie-boundary`. Redirect noisy output to logs/stage19b-*.log and read only tails.
The untracked codex-run-stage19*.log files are operator output; leave them alone.

Finish with the summary listed in item 8 of dev/CODEX_PROMPT-stage19.md, plus the 10,000-handshake
result and anything recorded as pending push.
