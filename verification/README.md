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

The four verify roots are `crates/http-subset/src/lib.rs`,
`crates/transport-core/src/lib.rs` `crates/transport-core/src/aead.rs` and `crates/transport-core/src/hello.rs`.
Their 211/347/9/30 verified obligations overlap; adding the groups would double-count.
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

Current CI coverage uses formal-coverage v1.9.0 and [the shared kit](../COVERAGE.md).
`verify.sh` retains one log and source-bound `.evidence.json` receipt per root in
`logs/verus-*.log`; each receipt is written only after a successful complete run.
The coverage workflow passes all four logs with repeated `--verus-log` arguments.
Python 3 is required alongside Rust/rustup. The old `measure-coverage.py` script
and stage19 hax/F* replay scripts are historical tooling, not current CI inputs;
the latter require the retained extraction toolchain via BERTIE_HAX_TOOLCHAIN
and historical source via BERTIE_SOURCE.
