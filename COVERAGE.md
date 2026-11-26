# scratch-server coverage

Measured with formal-coverage v1.9.0, public kit commit
`2c7e26a8c90afb44323ecebce69822e8b4bc9ca5`, using the four complete Verus
roots in `verification/verify.sh`. Receipts are caller-supplied source-bound
execution records, not authenticated attestations. Overlapping roots are unioned.
Tests are syntactic references, not executed line coverage.

<!-- measured coverage: begin -->

| Security runtime | Lines | Share |
| --- | ---: | ---: |
| proven | 887 | 64.93% |
| tested | 215 | 15.74% |
| unverified | 264 | 19.33% |

Total: **1366** first-party security runtime SLoC. Proven includes explicitly labelled
`proven-extracted` hax credit (0 lines); Verus proven: 887 lines.

Proof-to-code ratio: **6.40** (5677 proof SLoC / 887 proven + proven-extracted runtime SLoC); informational, never blocking.

Proof components: 0 hax annotation SLoC, 0 standalone `cfg(hax)` proof-code SLoC, and 0 inline `cfg(hax)` SLoC (overlaps count once). F* models: 0 SLoC of trusted assumptions, excluded from the ratio.

| Module | Proven runtime | Proof SLoC | Proof / proven runtime |
| --- | ---: | ---: | ---: |
| http_subset::<unattributed> | 332 | 116 | 0.35 |
| http_subset::Policy | 4 | 1 | 0.25 |
| http_subset::intent | 0 | 387 | n/a |
| http_subset::intent::brave | 0 | 491 | n/a |
| http_subset::intent::brave_cache_disabled | 0 | 517 | n/a |
| http_subset::intent::brave_dnt | 0 | 548 | n/a |
| http_subset::intent::firefox | 0 | 341 | n/a |
| http_subset::intent::reject_absolute | 0 | 28 | n/a |
| http_subset::intent::reject_bare_lf | 0 | 30 | n/a |
| http_subset::intent::reject_body | 0 | 37 | n/a |
| http_subset::intent::reject_cl_te | 0 | 57 | n/a |
| http_subset::intent::reject_cl_zero | 0 | 46 | n/a |
| http_subset::intent::reject_colon_space | 0 | 40 | n/a |
| http_subset::intent::reject_duplicate_connection | 0 | 58 | n/a |
| http_subset::intent::reject_duplicate_host | 0 | 58 | n/a |
| http_subset::intent::reject_empty_value | 0 | 40 | n/a |
| http_subset::intent::reject_fragment | 0 | 32 | n/a |
| http_subset::intent::reject_h2_preface | 0 | 28 | n/a |
| http_subset::intent::reject_headers25 | 0 | 649 | n/a |
| http_subset::intent::reject_http12 | 0 | 31 | n/a |
| http_subset::intent::reject_http20 | 0 | 31 | n/a |
| http_subset::intent::reject_lower_get | 0 | 27 | n/a |
| http_subset::intent::reject_name65 | 0 | 48 | n/a |
| http_subset::intent::reject_obs_space | 0 | 39 | n/a |
| http_subset::intent::reject_obs_tab | 0 | 39 | n/a |
| http_subset::intent::reject_path257 | 0 | 96 | n/a |
| http_subset::intent::reject_percent | 0 | 32 | n/a |
| http_subset::intent::reject_pipeline | 0 | 41 | n/a |
| http_subset::intent::reject_post | 0 | 27 | n/a |
| http_subset::intent::reject_query | 0 | 32 | n/a |
| http_subset::intent::reject_star | 0 | 28 | n/a |
| http_subset::intent::reject_te | 0 | 47 | n/a |
| http_subset::intent::reject_value257 | 0 | 106 | n/a |
| http_subset::intent::reject_value_cr | 0 | 42 | n/a |
| http_subset::intent::reject_value_nul | 0 | 42 | n/a |
| http_subset::intent::safari | 0 | 315 | n/a |
| http_subset::spec | 0 | 266 | n/a |
| http_subset::spec::Policy | 0 | 2 | n/a |
| transport_core::<unattributed> | 40 | 27 | 0.68 |
| transport_core::Machine | 41 | 30 | 0.73 |
| transport_core::aead | 29 | 24 | 0.83 |
| transport_core::app | 199 | 137 | 0.69 |
| transport_core::app::UnknownLog | 17 | 14 | 0.82 |
| transport_core::hello | 119 | 49 | 0.41 |
| transport_core::hello_intent | 0 | 56 | n/a |
| transport_core::io | 48 | 55 | 1.15 |
| transport_core::record | 58 | 38 | 0.66 |
| transport_core::spec | 0 | 41 | n/a |
| transport_core::spec::Machine | 0 | 9 | n/a |
| transport_core::spec::aead | 0 | 4 | n/a |
| transport_core::spec::app | 0 | 213 | n/a |
| transport_core::spec::app::UnknownLog | 0 | 5 | n/a |
| transport_core::spec::hello | 0 | 122 | n/a |
| transport_core::spec::io | 0 | 55 | n/a |
| transport_core::spec::record | 0 | 3 | n/a |

### Proof depth

| Contract depth | Lines | Share of proven lines |
| --- | ---: | ---: |
| safety-only | 887 | 100.00% |
| functional, checked only against own code | 0 | 0.00% |
| functional | 0 | 0.00% |

Safety-only means proven free of panics, overflows and out-of-bounds access, including length and range facts; it says nothing about what the code computes.

<!-- measured coverage: end -->

## Measurement boundary

The first-party security runtime denominator conservatively starts at main and
includes startup, TLS adapters and the readiness probe. Test/proof code is separated
structurally by the tool. Third-party TLS/crypto code is outside this badge's
first-party denominator and remains trusted; no Bertie F* receipts are supplied.
Verus credit requires accepted evidence and explicit exec contracts. Uncontracted
exec functions, inactive cfg branches, trusted external bodies and unsafe/escape
sites cannot become proven merely because a root passes. The full report retains
those reasons, trust sites and escape inventory. No audit is disabled for credit.

All four receipts were accepted. The tool excludes `#[path]` spec/intent
modules and inactive cfg branches from receipt scope; passing roots do not
establish credit for those excluded items. Proof SLoC is an inventory cost, not
a count of independently validated specifications.

The kit's 450 unverified lines comprise 367 ordinary unverified and **83 hazard**
lines. v1.9.0 conservatively flags the existing connection-admission function
`transport_core::io::admit` (4 runtime lines) and its reference in
`scratch_server::run` (79 runtime lines) as `admit` escapes. Inspection shows an
ordinary capacity-check function with a Verus postcondition, not a newly added
proof escape. The tool also flags its test references. We retain that audit and
withhold the credit; no rename, suppression or tool change was used to raise it.
The local macOS full inventory reports 0 trusted items and 4,168 escape sites
(including dependency unsafe and out-of-scope sites); receipt-local scopes report
zero trusted items and one flagged admission-name site in the transport root.
These are conservative inventory counts, not 4,168 admitted proofs. Third-party
compiled-size evidence falls back to whole-crate size for chacha20 0.10.2; the
inventory remains complete and the fallback does not affect first-party badges.

The historical 47.25% (644/1,363) used formal-coverage v1.4 on source 2fa5eb7.
Changes to code, runtime/proof separation, test classification and evidence rules
make it an historical reference, not a comparable trend or a regression floor.

## Badge policy and ratchet

`coverage/ci.json` uses the kit defaults: proven thresholds 90/75/50/25%;
unverified exclusive upper thresholds 10/25/50/75%. Tested is blue. Proof/code
is informational, development range 5–15 (established 10–20); it never blocks.
Unverified security lines cannot increase, proven share cannot fall, and security
runtime edits must increase proven share unless explicitly approved below.

An initial reviewed baseline must land before enabling the ratchet. The kit fails
closed when the merge base/before commit has no baseline; the first adoption PR
cannot ratchet against the pre-adoption tree. No fabricated historical baseline
or exception is installed. Maintainers must enable up-to-date branch checks.

## Reproduce

From the root, fetch with `cargo fetch --locked`, then run
`./verification/verify.sh > logs/proofs.log 2>&1`. Pass the four
`logs/verus-{http-subset-lib,transport-core-lib,transport-core-aead,transport-core-hello}.log`
files individually via repeated `--verus-log` to the pinned tool's `report` or
kit `ci/run.py` (the exact argument array is in the coverage workflow).
Use the kit's `ci/write-coverage.py` for reviewed baseline/document updates;
`ci/run.py` checks them byte-for-byte and ratchets against a supplied git base.
Generated reports, receipts and badge JSON stay in ignored logs/ and target/.

## Portability

Current build/test/Verus/coverage use public pinned sources. Historical stage19
hax/F* extraction replay scripts require explicit BERTIE_HAX_TOOLCHAIN/BERTIE_SOURCE paths and are
not inputs to these workflows. Historical reports/prompts/logs preserve old paths.
The surface build's `git+file:///src` names the checked-out repository in Docker;
it is not a host-local dependency. Nix/audit wrappers derive Git metadata mounts.

## Exceptions

| Commit | Approving maintainer | Reason | Restoration obligation |
| --- | --- | --- | --- |
