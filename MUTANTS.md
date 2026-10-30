# Spec mutants — Stage 13

Final result: **15/15 killed**, **0 survivors**, **0 malformed/invalid runs**.
All fifteen preserved the preexisting refinement proofs. Thirteen failed focused
intent proofs; the unknown-default and Cookie-default faults were caught by
fixture/config intent tests. Every mutant also failed actual workspace tests.

Run `python3 scripts/spec-mutants.py` from a clean checkout with the repository's
pinned tools available. The baseline must pass full `./verify.sh` and workspace
tests. Each mutant then runs full `./verify.sh` with the new intent module
excluded to check every preexisting refinement proof. After restoring the module,
the runner checks the focused intent lemma listed below and runs all workspace
tests (`--no-fail-fast`), including differential, config fuzz and loopback TLS.
Independent intent helpers are all verified in the baseline. A malformed mutant
is never a kill. Solver exhaustion is inconclusive; a genuine test failure can
still kill that mutant. Tool/compile errors are reported separately.

Temporary copies and shared build artifacts stay under target; working-tree Rust,
config and fixture hashes must remain unchanged. Logs retain outcomes, test names
and source locations, excluding request literals and failure payloads. Receipts:
`logs/stage13-mutants/results.json` and adjacent per-mutant logs.

| Mutant | Killed? | What killed it |
| --- | --- | --- |
| 1. Remove Transfer-Encoding from hard class | Yes | `hard_name_intent`, `attack_intent` |
| 2. Move Expect to guarded | Yes | `hard_name_intent`, `attack_intent` |
| 3. Header cap 25 | Yes | `reject_headers25`, `attack_intent` |
| 4. Value cap 257 | Yes | `reject_value257`, `attack_intent` |
| 5. Empty values | Yes | `reject_empty_value`, `attack_intent` |
| 6. Bare LF header lines | Yes | `header_framing_intent`, `attack_intent` |
| 7. Case-insensitive GET | Yes | `reject_lower_get`, `attack_intent` |
| 8. HTTP/1.2 | Yes | `reject_http12`, `attack_intent` |
| 9. Duplicate Host only | Yes | `reject_duplicate_connection`, `attack_intent` |
| 10. Built-in unknown reject | Yes | Brave+DNT TLS witness and `shipped_config_intent` |
| 11. Percent in path | Yes | `reject_percent`, `attack_intent` |
| 12. Strict ignores guarded mask | Yes | `strict_checks_guarded`, `shipped_config_intent` |
| 13. Space in token | Yes | `reject_colon_space`, `attack_intent` |
| 14. Trailing bytes | Yes | `reject_body`, `attack_intent` |
| 15. Built-in Cookie drop | Yes | Cookie rejection in `attack_intent`; `shipped_config_intent` |

Survivor resolution: **none required; no mutant survived or was excluded**.

Small mutation choices: Expect moves to the existing guarded Cookie bit, so a
valid policy enabling that bit exposes the forbidden hard-to-guarded transition.
The method fault targets GET, including the required lowercase-get witness.
Bare LF is admitted for header lines and the final blank line, with an explicit
LF-header test; request-line framing stays unchanged. Empty-value guards and
matching bound annotations change together, preserving memory safety/refinement.
These choices are recorded in NOTES.md and retain all fifteen required faults.

Final baseline: `./verify.sh` **211/0 parser,237/0 transport,9/0 AEAD**; full
workspace tests pass, including 100,000 differential inputs (three policies, zero
mismatches),10,000 config fuzz cases and loopback TLS acceptance. The independent
aarch64-linux Nix check also passes. Counts overlap across proof roots. No
deployment or protected-container reuse; production executable source unchanged.
