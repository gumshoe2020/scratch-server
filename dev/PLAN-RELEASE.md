# PLAN-RELEASE: make scratch-server ready to publish

This plan is authoritative for this pass. The repository is about to become public under the MIT
license. An independent acceptance check (2026-10-29) re-ran the proofs, tests and spec mutants
and all held. It found one real defect (the Nix image is not gated on proofs), inconsistent
coverage figures, and docs written for the build operator rather than a public reader.

Historical coverage examples in this plan refer to first-party formal-coverage v1
measurements from 2026-10-27; they describe the pre-release findings.

## Grounded facts (checked 2026-10-29)
- Branch layout is deliberate. `main` is the landing branch: stage 1 code plus EVOLUTION.md (the
  stage index with compare links) and ATTACK-SURFACE.md. Stages 2–13 each live on their own
  branch, each built on the previous one. `spec-lemmas` (stage 13, `2fa5eb7`) is the tip and
  contains every stage branch. `main` is not an ancestor of the stage branches; it has 19 docs
  commits of its own.
- On `spec-lemmas`: verification/verify.sh gives 211/237/9 verified with 0 errors (macOS and Linux);
  `cargo test --locked` passes 31/31; 15/15 spec mutants are killed.
- **Defect:** `nix build .#image-static` (what `./nix-build.sh image-static` runs) builds a
  deployable image from a tree with a broken proof. Only `nix flake check` runs the proofs and
  cargo-deny; the image derivations depend on tests (`doCheck`) alone. This contradicts commit
  `bc2c73b` ("gate Nix builds on proofs, tests and dependency checks") and EVOLUTION's stage row.
- Coverage: formal-coverage v1.4 measures `spec-lemmas` first-party security code at 644 of 1,363
  SLoC proven (47.25%): 644 proven, 300 tested, 419 unverified. Shipped third-party runtime
  dependencies are 336,654 SLoC for aarch64-unknown-linux-musl (38–39 packages, 0 unknown). The
  repo instead states 42.43%, 11.73% and "715 lines" in various places, and 47.25% nowhere.
- `main` tracks build artifacts: `image.tar.gz` (17 MB), `image-static.tar.gz` and a `result`
  symlink into `/nix/store`.
- The cargo-deny advisory snapshot makes fresh checks fail after about 2026-11-10.
  `nix-build.sh` uses unpinned `nixos/nix:latest`.

## Non-negotiables
- Do not weaken a proof, spec, test or mutant, and add no `assume`/`admit`/`external_body`.
  verification/verify.sh must still report 0 errors with verified counts at least 211/237/9. All 15 mutants must
  still be killed.
- Code changes go on `spec-lemmas`; docs changes on `main` only where named below. Leave the other
  stage branches alone: they are historical snapshots, cleaned later by a history rewrite. Do not
  push, rewrite history or create branches.
- Do not touch running containers or images you did not create (`parser-https`, `header-policy`,
  `parser-http`, `nix-scratch` and their tags). Name anything you create `release-*` and remove it
  when done.
- Redirect every build log to a file and read only `tail -n 30` or grep. Never use `nix build -L`.
- One commit per milestone per branch.

## Milestones

### M1: the image is gated on proofs and dependency checks (spec-lemmas)
- Make `image` and `image-static` in flake.nix depend on the same proof (Verus) and cargo-deny
  derivations that `nix flake check` runs, so building either image fails when any of them fails.
- Pin the `nixos/nix` image in nix-build.sh by digest.
- cargo-deny advisories: make the check deterministic for a given commit. Use a pinned advisory-db
  revision in the Nix build, and document how to bump it. A clean clone must not start failing on
  a calendar date. If that isn't feasible, document the exact behavior and the refresh command in
  the README limits section and log the reason in NOTES.md.
- Acceptance:
  - On a clean tree, `./nix-build.sh image-static` succeeds and the image runs (`--healthcheck`
    passes in a `release-*` container).
  - In a throwaway clone with one proof deliberately broken, `nix build .#image-static` fails, with
    the Verus derivation named as the failure. Same for a cargo-deny failure (e.g. a denied
    license injected into deny.toml in the clone).
  - Record both negative runs' commands and tail output in NOTES.md.

### M2: one coverage figure, stated honestly (spec-lemmas and main)
- Everywhere the repo states proof coverage, use the v1.4 figure: "first-party security-relevant
  code: 644 of 1,363 SLoC proven (47.25%), measured by formal-coverage v1.4 at `2fa5eb7`; 300
  tested, 419 unverified. Shipped third-party runtime dependencies: 336,654 SLoC (not proven)."
  Always say "first-party", give the counting rule (package source SLoC, security boundary
  defined by formal-coverage's scratch-server config), and link to formal-coverage.
- Remove the old figures (42.43%, 11.73%, 715 lines, and the 17.4%→34.4% in EVOLUTION), or label
  each with the tool version and date it came from. Nothing may read as current unless it is.
- EVOLUTION.md per-stage coverage column (on main): leave the cells as "v1.4 remeasure pending"
  with one note under the table. Per-stage v1.4 numbers are being regenerated in formal-coverage
  and will be filled in afterwards. Do not invent them.
- Acceptance: `git grep -n -E "42\.43|11\.73|715 lines|34\.4%"` on both branches returns nothing
  unlabelled; every coverage claim says first-party and names the tool version.

### M3: public docs and scripts (spec-lemmas and main)
- README.md on `spec-lemmas` for a public reader:
  - what this is, the threat model, what is proven / tested / trusted;
  - how to build and run (relative paths, generic ports, no local paths), and how to verify
    (verification/verify.sh, tests, mutants, nix-build);
  - limits: TLS status as it actually stands; x86_64 never built; browser and second-device tests
    come only from the author's own sessions; the Nix build is not bit-reproducible (build-dir
    paths in panic strings).
  - Remove operator text (protected containers, "Dudley verifies and pushes"). Retitle
    VERIFIED-BOUNDARY.md (drop "spike, not started").
- README.md and EVOLUTION.md on `main`: same treatment, plus a short "Start here" pointing to
  EVOLUTION. `git rm` `image.tar.gz`, `image-static.tar.gz` and `result` from main and add them to
  .gitignore.
- Scripts: `scripts/acceptance.py` takes container, port and host as arguments (defaults
  localhost); `scripts/make-lan-cert.py` takes the host as an argument; `measure-coverage.py`
  takes the formal-coverage path as an argument and fails clearly without it.
- LAN address: replace `203.0.113.28` with `203.0.113.28` (TEST-NET-3, same length, so byte
  offsets in fixtures don't move) in fixtures, scripts and docs, and confirm the tests still pass.
- Names and private details: "Andrew" and "Dudley" may stay. Remove other personal names, device
  descriptions (e.g. a named person's desktop) and mentions of other private projects. Run
  `scripts/check-public.sh <notes>/public-denylist.txt` until it reports zero hits on both
  branches. The script greps tracked files case-insensitively for each denylist
  line, also flags a marker of five hyphens followed by BEGIN, and exits non-zero on hits. Commit the script, not the denylist.
- Build-record files (CODEX_PROMPT*.md, PLAN*.md, PROGRESS.md, NOTES.md) stay, moved into `dev/`
  with a `dev/README.md` explaining they are the build record. Replace absolute paths in them
  (including this plan) with placeholders such as `<notes>/public-denylist.txt`.
- Typography: fix the missing spaces Codex left in prose ("on 8095", "of 1–64", "at most 24",
  "Stage 11", "uid 65532", "mode 700" and the like) across all Markdown on both branches. Prose only:
  never change code, fixtures or code blocks.
- Acceptance: check-public.sh exits 0 on both branches; `rg -n "[a-z][0-9]{2,}|[0-9]–[0-9]+[a-z]"`
  over *.md shows no prose run-togethers (code spans and identifiers are fine); tests, verification/verify.sh
  and mutants still pass.

### M4: license (both branches)
- LICENSE: MIT, "Copyright (c) 2026 Andrew VanLoo". Add `license = "MIT"` to every Cargo.toml
  package. Update deny.toml's comment that "no license grant is made". Make sure cargo-deny's
  license check still passes for our own crates.
- Acceptance: `nix flake check` passes on spec-lemmas; LICENSE is present on both branches.

## Final check (spec-lemmas)
verification/verify.sh (0 errors, ≥ 211/237/9), `cargo test --locked`, the stage-13 mutant run (15/15 killed),
`./nix-build.sh image-static`, and check-public.sh all pass. Record the results in PROGRESS.md.

## Out of scope
- Changing proofs, specs or the HTTP subset's behavior beyond what M1 and M3 name.
- Pushing, history rewriting, GitHub settings, editing other stage branches.
- Anything in formal-coverage.
