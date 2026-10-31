# Specifications

[The catalog](SPEC-CATALOG.toml) maps spec modules to prose, source files,
checks, maturity and the reason for that maturity. Paths are repository-relative;
Rust module paths do not encode maturity.

`development/` contains proven or evolving specs that are not yet hardened.
They may change as the design develops. `established/` contains settled specs:
a change needs a written reason, a version bump and Andrew's sign-off, and sends
the spec back to `development/`. Promotion requires Andrew's decision that the
spec is settled; move the prose and update its catalog status/reason together.
Demotion likewise moves the prose and updates the catalog, without moving Rust
modules. Keep the written reason and version decision in the build record.

The parser/header policy starts established, with intent lemmas, fifteen mutants
and differential checks. Transport, dispatch, record, I/O and AEAD start in
development: their proofs do not supply dedicated intent lemmas or mutants.
HTTPS and provider prose describe a largely trusted boundary and stay in
development. A document spanning tiers belongs in development, with the conflict
recorded in [NOTES](../dev/NOTES.md).

Run `python3 verification/scripts/check-catalog.py` from the repository root.
The [verification guide](../verification/README.md) describes the checks.
