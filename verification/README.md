# Verification

Local Verus proofs cover the HTTP subset and header policy, pure transport and
response formatting, dispatch/logging decisions, record/provider gates, I/O
choices and AEAD helpers. They establish refinement to written specs; the parser
also has intent lemmas and fifteen deliberate spec mutants. TLS protocol logic,
I/O adapters and other trusted components remain outside that proof claim.
See [proofs](reports/PROOFS.md), [intent](reports/INTENT.md),
[mutants](reports/MUTANTS.md) and the [trusted base](TRUSTED-BASE.md).

Run from the repository root, retaining output under `logs/`:

```sh
./verification/verify.sh > logs/proofs.log 2>&1
python3 verification/scripts/spec-mutants.py > logs/mutants.log 2>&1
CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" \
  cargo test --locked --workspace > logs/tests.log 2>&1
./nix-build.sh check > logs/checks.log 2>&1
python3 scripts/check-links.py
python3 verification/scripts/check-catalog.py
python3 verification/scripts/exec-identity.py --before 3c5f2f7
```

The three verify roots are `crates/http-subset/src/lib.rs`,
`crates/transport-core/src/lib.rs` and `crates/transport-core/src/aead.rs`.
Their 211/317/9 verified obligations overlap; adding the groups would double-count.
Spec functions and spec-property proofs live in crate-local `spec.rs` or `spec/`
modules; exec contracts and invariants stay beside executable functions.
`intent.rs` retains its fixtures, helpers and lemmas.

[The catalog](../specs/SPEC-CATALOG.toml) assigns each spec source file exactly
one owner, links its prose and records tier/checks/reason. Its tier describes
maturity, not proof coverage. HTTPS/provider prose entries have no Verus files.
The checker validates source ownership, file/prose existence, prose coverage and
matching tier folders. [Spec policy](../specs/README.md) explains promotion and
demotion. Historical stage notes live in [docs/stages](../docs/stages/).

Coverage scripts live in `verification/scripts/`: the historical
`measure-coverage.py /path/to/formal-coverage` retains the v1 receipt method;
`rustls-surface-build.sh`, `rustls-surface.py` and `stage17-coverage.py` preserve
stage 17's build, surface and corpus-coverage measurements. Consult their usage
and [surface report](reports/RUSTLS-SURFACE.md) before running them: corpus
coverage requires explicit TLS fixtures. No new coverage measurement is implied
by this layout change. [README](../README.md) states the measured baseline and limits.
