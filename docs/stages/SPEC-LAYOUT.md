# SPEC-LAYOUT.md: stage 18 (specs/, verification/, docs/; no behaviour change)

Branch `spec-layout`, built on `rustls-surface` (`3c5f2f7`). Requested by Andrew 2026-10-31 ("oh
Lord, they are all in the root"; "we should make a folder for all of the machinery associated with
formal verification"). Authoritative for this stage. This file itself moves to
`docs/stages/SPEC-LAYOUT.md` in M1.

## Why

The repository root holds 26 Markdown files: current specs, verification reports and superseded
stage notes side by side. Verus spec functions are interleaved with executable code in the same
source files. The coming spec-strength work (formal-coverage v1.7) needs specs to be findable,
catalogued, and lockable by path, with a development/established status per spec.

## Target layout

Andrew chose three top-level folders: `specs/`, `verification/`, `docs/`, with `specs/` split
into `established/` and `development/` ("then we need to subdivide the specs into established and
development").

```
README.md  EVOLUTION.md  ATTACK-SURFACE.md  LICENSE      (project level, stay in root)
Cargo.*  flake.*  deny.toml  Dockerfile  nix-build.sh  nix/  config/  src/  crates/  tests/  fuzz/
dev/                     build record (prompts, NOTES, PROGRESS), unchanged role
specs/                   what the code must satisfy
  README.md              new, short: what the two tiers mean, how promotion/demotion works
  SPEC-CATALOG.toml      new: one entry per spec (see below)
  established/           settled; changes need a written reason, a version bump and Andrew's
                         sign-off, and send the spec back to development/
    HEADER-POLICY.md
  development/           proven or in progress, not yet hardened; free to change
    PROVEN-TRANSPORT.md  PROVEN-DISPATCH.md  PROVEN-RECORD.md  PROVEN-IO.md
    HTTPS.md  TLS-PROVIDER.md
verification/            the machinery that checks it
  README.md              what is proven, how to run it, where everything is (new, short)
  verify.sh              moved from root
  TRUSTED-BASE.md        moved
  reports/               PROOFS, INTENT, MUTANTS, SPEC-LEMMAS, VERIFIED-BOUNDARY, TLS-COVERAGE,
                         RUSTLS-SURFACE
  scripts/               spec-mutants.py, measure-coverage.py, rustls-surface-build.sh,
                         rustls-surface.py, stage17-coverage.py
docs/
  stages/                superseded stage notes kept as history: BROWSER-WHITELIST, HTTPS-HEADERS,
                         FIREFOX-HEADERS, CHROMIUM-HEADERS, PRAGMA-HEADER, IDLE-CONNECTIONS
                         (and this file)
```

Root `NOTES.md` and `PROGRESS.md` move to `dev/` (append to the existing `dev/NOTES.md` /
`dev/PROGRESS.md` only if those are the same documents; otherwise move as `dev/NOTES-root.md`,
`dev/PROGRESS-root.md` and log the choice). Other scripts (acceptance, live checks, cert, public
check, fuzz/capture) stay in `scripts/`. `fuzz/` stays at root (cargo-fuzz convention).

Inside each Verus crate, spec functions (`spec fn`) and spec-only helpers move into a `spec.rs`
module of that crate (`crates/http-subset/src/spec.rs`, `crates/transport-core/src/spec.rs`,
plus per-module files if one file would mix unrelated modules: `spec/app.rs`, `spec/io.rs`, ...).
Rust module paths must not encode tier (promotion must not force a Rust module move); the tier
of a Verus spec file comes from its catalog entry, and the prose doc's folder must match it.
`intent.rs` stays where it is. `proof fn`s that prove properties of a spec (not of exec code) move
with it; proofs about exec functions stay next to them. `requires`/`ensures` and loop invariants
stay on the exec functions.

## specs/SPEC-CATALOG.toml

One `[[spec]]` per spec module:

```toml
[[spec]]
id = "http-subset.request"
status = "established"          # or "development"
prose = "specs/established/HEADER-POLICY.md"
files = ["crates/http-subset/src/spec.rs", "crates/http-subset/src/intent.rs"]
checks = ["intent lemmas", "15 spec mutants", "differential vs httparse and grammar checker"]
reason = "hardened over stages 3-13; intent layer and spec mutants target it"
```

Initial status: the parser/header-policy spec is `established`; transport, dispatch, record, io
and aead specs are `development` (they have proofs but no intent lemmas or mutants of their own).
HTTPS.md and TLS-PROVIDER.md go to `development/` (they describe the TLS boundary, largely
trusted code with no spec-level checks). If a prose doc covers specs of both tiers, it goes to
`development/` and the conflict is logged in NOTES.
Every entry needs a non-empty `reason`. Add `verification/scripts/check-catalog.py`: every listed
file exists, every `spec fn` in the Verus crates belongs to exactly one catalog entry's files,
every prose link resolves, every `.md` under `specs/established/` or `specs/development/` is some
entry's `prose`, and each entry's `prose` path sits in the folder matching its `status`.

## Milestones and acceptance

- **M1 documents and scripts.** `git mv` every file in the layout (history must follow:
  `git log --follow` works for a sample). Update every reference: Markdown links in all `.md`
  files (README, EVOLUTION table, cross-links), `flake.nix` (`proofSource` fileset and the
  `proofs` check, which run `./verification/verify.sh`), `nix-build.sh`, `scripts/*.py` and `.sh`,
  `verification/verify.sh`'s own relative paths, `.gitignore`. A link checker script
  (`scripts/check-links.py`, local links and anchors only) reports zero broken links.
  No Rust source changes in M1. **Acceptance:** `verification/verify.sh` gives 211 / 317 / 9
  verified, 0 errors; `cargo test --workspace` passes 52; the Nix static server binary is
  **byte-identical** to the one built from `rustls-surface` (record both sha256s); `nix flake check`
  passes (or, if it cannot run offline here, the proofs and tests derivations it contains are
  built individually and the reason is logged).
- **M2 Verus spec split.** Move spec functions as described. **Acceptance:** proofs still 211 /
  317 / 9 with 0 errors (counts may change only if a proof item was split or merged; any change is
  explained item by item in NOTES); `cargo test --workspace` passes 52; a script
  (`verification/scripts/exec-identity.py`) shows every exec function's token stream (with
  `requires`/`ensures`/invariants and ghost blocks stripped) is identical before and after; the
  stage 13 spec mutants still run and are all killed (update `spec-mutants.py` paths). The binary
  may differ only through file/line metadata; report whether it is byte-identical and, if not,
  confirm `.text` size is unchanged.
- **M3 catalog.** `specs/SPEC-CATALOG.toml`, `specs/README.md` and
  `verification/scripts/check-catalog.py` as above, passing.
  `verification/README.md` written (short: what is proven, the three verify roots, how to run
  verification/verify.sh / mutants / coverage, catalog meaning).
- **M4 audit.** EVOLUTION.md gets a stage 18 row (no port; "layout only"). Zero broken links.
  No changes under `src/` except `mod` lines if needed; no runtime behaviour change; no
  containers started, stopped or reused (ephemeral `docker run --rm` builds as in nix-build.sh are
  allowed). `git status` clean.

## Non-negotiables

- Move, don't rewrite: prose specs keep their content; only links and paths change. Do not merge
  or edit specs' meaning. Do not weaken or delete any proof, lemma, test or mutant.
- Stay on branch `spec-layout`; don't push; commit per milestone as the repo identity (Dudley).
- Keep `dev/PROGRESS.md` (stage 18 section) and `dev/NOTES.md` current with timestamped decisions.
- Never print or commit keys; don't touch the TLS key directory outside the repo.
