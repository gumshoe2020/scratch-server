# Notes

## 2026-10-30 09:03 EDT — Stage 14 blocked: malformed diagnostic name spans

Read PROVEN-DISPATCH.md fully first, then PROVEN-TRANSPORT.md, HEADER-POLICY.md,
PROOFS.md and MUTANTS.md. PROGRESS.md and NOTES.md did not exist, so created them.
Branch remains `proven-dispatch`; no branch creation, push, deployment, container
operation, formal-coverage operation, or protected-file modification occurred.

The authoritative spec's property 3 requires every yielded name to contain no CR,
LF or colon. Its equivalence requirement simultaneously requires the same name
sequences as the old iterator for all generated malformed blocks, explicitly
including bare CR and LF. These cannot both hold. The existing walker splits on
CRLF only and takes every byte before the first colon, including bare CR or LF.

Minimal construction (no header value recorded here): a normal request line,
CRLF, the name bytes `X`, a bare CR or bare LF, `Name`, a colon, an otherwise valid
synthetic field value, CRLF, CRLF. The old walk yields the five-letter name with
the embedded line-break byte. With strict policy it produces exactly
`status=400 reason=unknown_header name=XName`. Skipping that invalid name instead
produces `status=400 reason=grammar`; truncating or filtering the span cannot
preserve the required name sequence either. The parser rejects both requests;
this is a diagnostic-walk contract bug, not evidence of parser acceptance or log
injection. The existing sanitizer removes the embedded line break.

Following the explicit stop/commit-evidence instruction, did not implement or
repair production behavior. Preserved the four requested implementations verbatim
in test-only src/dispatch_oracle.rs before any deletion (none was deleted).
Two regression tests reproduce both bare-CR and bare-LF counterexamples, compare
against the unchanged current implementation, check strict diagnostic output and
check parser rejection. A third test checks the preserved sanitizer on all 256
byte values and compares both existing limiters for four limits (4,008 calls).
Assertions deliberately omit raw input payloads from failure messages.

Suggested spec resolution, requiring Andrew's decision: permit arbitrary bytes
other than colon in diagnostic name spans, and keep the strict no-control-byte
property on sanitizer output. This preserves all runtime behavior. Alternatively,
authorize a diagnostic behavior change and relax malformed-name equivalence.
No spec, proof, existing test, or existing behavior was weakened here.

### Function inventory at blocker

No production function moved or added; additional proven runtime SLoC: **0**.
All original src/lib.rs functions remain in the unverified shell with identical
source to 5b880fb (apart from a cfg(test) module declaration). No app.rs exists.
SLoC below counts nonblank, noncomment source lines, including braces, matching
these four function counts in PROVEN-DISPATCH.md; it is not a coverage measurement.

| Function | Old location | Added location | Status | SLoC |
| --- | --- | --- | --- | ---: |
| UnknownLog::should_log | src/lib.rs:41 | src/dispatch_oracle.rs:10 | test-only oracle; production shell unchanged | 15 |
| sanitized | src/lib.rs:57 | src/dispatch_oracle.rs:26 | test-only oracle; production shell unchanged | 8 |
| header_names | src/lib.rs:66 | src/dispatch_oracle.rs:35 | test-only oracle; production shell unchanged | 20 |
| rejection_line | src/lib.rs:139 | src/dispatch_oracle.rs:55 | test-only oracle; production shell unchanged | 31 |
| stage14_blocker_header_spans_contain_bare_line_breaks | new | src/dispatch_oracle.rs:89 | test only, not proven | 11 |
| stage14_blocker_malformed_name_changes_rejection_diagnostic | new | src/dispatch_oracle.rs:103 | test only, not proven | 13 |
| stage14_preserved_oracle_sanitizer_and_limiter | new | src/dispatch_oracle.rs:119 | test only, not proven | 19 |

Full 100,000-case new equivalence suite, random names, all 15 spec mutants, Nix
checks/image, and live checks on 8096 are not run because implementation stopped
at the mandatory blocker. No image/container was built or replaced. Historical
stage-13 results are not claimed as stage-14 results. Existing PROOFS.md remains
unchanged because there are no new proven properties.

## 2026-10-30 09:04 EDT — Blocker tests and baseline workspace validation

`CARGO_HOME="$PWD/.tools/cargo" RUSTUP_HOME="$PWD/.tools/rustup" cargo test
--workspace` passed (exit 0): all 31 existing tests plus 3 new evidence/oracle
tests, **34 passed, 0 failed**. The inherited 100,000 differential cases remain
in this run. New equivalence evidence comprises 2 malformed blocks checked for
both name sequence and strict diagnostic/parser result, 256 single-byte sanitizer
inputs, and 4,008 limiter calls across limits 0, 1, 60, 600. These compare the
preserved old code with unchanged production code, not a new proven core.
Receipt: logs/stage14-tests.log. First-party Rust prohibited-token scan across
tracked src/crates/tests and the new oracle: zero matches (receipt
logs/stage14-prohibited-scan.log); no banned construct was added.

## 2026-10-30 09:04 EDT — Baseline proofs pass; commit blocker evidence

`./verification/verify.sh` completed with exit 0: **211 verified / 0 errors** parser,
**237 / 0** transport, **9 / 0** standalone AEAD. Counts overlap because the
transport root includes parser and AEAD source. Receipt: logs/stage14-verify.log.
These are unchanged baseline proofs, not completion of any stage-14 property.
`git diff --check` passed. The new universal CR/LF-free span property cannot be
proven while preserving these old iterator sequences; implementation stopped
under PROVEN-DISPATCH.md's mandatory bug/equivalence stop rule. Spec-mutant result
this run: **not run / 15**, live 8096 checks: **not run**, Nix: **not run**.
No private key or header value was printed or retained in these notes. The
pre-existing untracked codex-run-stage14.log is left untouched and uncommitted.
No external diary write was made because this task forbids writes outside this
repository (except TMPDIR and Docker); this NOTES.md retains the audit record.

## 2026-10-30 09:17 EDT — Resolution A and core implementation

Resumed after Andrew approved amendment 0cb7016: diagnostic names exclude colon
and CRLF pairs but preserve bare CR/LF. Retained the original evidence tests,
renamed them compatibility tests and updated their comments. The historical
blocker entries above are retained as an audit trail and are resolved.

Pure decisions live in crates/transport-core/src/app.rs under the existing root.
The module uses its own verus! block, like aead.rs; the declaration is at module
scope, so no tooling or coverage configuration change is required. An arbitrary
cursor causes the name walker to rescan from the request line. This prevents a
caller from skipping the first empty line and preserves exact old iteration for
normal cursors. At most 25 names are needed for rejection diagnosis; production
accepted requests have at most 24 names. The small rescans are a deliberate
simplicity tradeoff.

Limiter state has a fixed limit for its lifetime, matching ServerState's immutable
policy; valid(limit) carries that invariant. Timestamp rollback retains future
entries, matching Instant::duration_since's saturating zero age. Comparison occurs
before subtraction, so no underflow. Raw ASCII lowercase storage matches the old
String implementation for production names, which are parser-validated ASCII
tokens; non-ASCII limiter names cannot occur on the accepted-request logging path.
Sanitizer and diagnostic equivalence still cover arbitrary byte inputs.

## 2026-10-30 09:20 EDT — Core checkpoint verified

`./verification/verify.sh` passed: parser **211/0**, transport **285/0**, standalone AEAD
**9/0** (overlapping roots). The transport root adds 48 verification results,
including an induction over arbitrarily long limiter traces proving every
half-open 60,000 ms window contains at most min(limit,64) successful log calls.
The unknown-line and response formatter consume the same route_status function;
status_agrees connects its numeric log code to the fixed wire status line.
Plan carries exactly one handler increment for Dispatch and zero for every other
action. task_response has no bytes for ReadMore/CloseSilently/Done and a fixed
400/non-HEAD/bad-request body for every rejection, independent of diagnostics.
Receipt: logs/stage14-core-verify.log. No earlier proof or contract was weakened.

## 2026-10-30 09:22 EDT — Shell rewired and equivalence checkpoint

src/lib.rs now owns only IO/state construction, socket event mapping, the clock,
mutex, stdout, dynamic filesystem page body, atomic effect and response writes.
Routing, log filtering/formatting, rejection reasons, response/body/status and the
post-machine counter delta are delegated to app.rs. The counter effect remains
after unknown-header logging and before root-body IO, matching the old ordering.
The rejection stdout error remains ignored before writing the fixed response.
No TLS, main, transport, parser or policy source was changed.

As required by the spec, the shell converts elapsed Instant time since state
creation to u64 milliseconds (saturating at u64::MAX). The core's 60,000 ms window
uses that requested clock resolution. Equivalence time tests use exact millisecond
instants, including the 59,000/60,000 boundary; sub-millisecond scheduling is not
modeled by the requested millisecond API. Wall-clock delivery remains trusted.

The first new equivalence run found a test-loader error: malformed.hex uses
space-separated octets. Corrected the loader to split whitespace. It did not
expose an old/new implementation difference and required no behavior change.
The corrected 100,000 generated blocks give **0 name-sequence differences** and
**0 diagnostic differences across 300,000 comparisons** (builtin, strict and
all-guarded-drop policies). All required malformed categories are generated.
Sanitizer: 256 byte values plus 10,000 random names up to 300 bytes, 0 differences.
Limiter: 4,008 inherited/boundary comparisons plus 80,000 generated sliding-window
comparisons, 0 differences; additional rollback and u64::MAX checks pass.
Receipts: logs/stage14-equivalence-debug.log and logs/stage14-tests-final.log.

## 2026-10-30 09:22 EDT — Full workspace acceptance

`cargo test --workspace` (repo-local pinned Cargo/Rustup homes) passed: **39 tests,
0 failures**, including all 31 inherited tests unchanged in meaning, the retained
3 evidence/oracle tests, 3 expanded equivalence tests and 2 response-plan/shell
routing tests. Existing limiter test interface changed from Instant offsets to
equivalent u64 millisecond offsets; its limits, counts and boundary assertions
are unchanged. New integration tests exercise GET/HEAD headers and handler counts
for all three routes, all no-response actions and every rejection reason.
Full new equivalence counts are recorded in the previous entry. No runtime bug
was found by the equivalence tests. No claim of independent acceptance is made.

## 2026-10-30 09:27 EDT — Acceptance campaigns and diagnostic span inventory

Started the existing spec-mutant runner and Nix check. Their top-level output
is redirected to stage14-prefixed logs; their internal receipt paths remain the
existing tooling defaults. The required nix-build.sh entry point uses a transient
release-nix build container and the existing Nix cache volume; the only runtime
container created/replaced for this stage is proven-dispatch on 8096. Protected
container identities/pids/start times were recorded read-only before deployment.

TooManyHeaders also carries the 25th name span, as specified, while its line
remains the fixed name-free prefix. This strengthens diagnostic span validity
without changing any output. Focused re-verification: 48/0. Final full verification
is running against that source; all runtime Rust is now held fixed during the
mutant campaign.

## 2026-10-30 09:30 EDT — Final proof acceptance

Final ./verification/verify.sh passed with exit 0: parser **211/0**, transport **285/0**,
standalone AEAD **9/0**, including the final diagnostic span representation.
Receipt: logs/stage14-verify-final.log. PROOFS.md lists every property with
file/line references and the remaining unverified shell. Repeated first-party
Rust prohibited-token scan: **zero matches** (logs/stage14-prohibited-scan.log).
No earlier executable parser/policy/grammar, proof or test was weakened.

## 2026-10-30 09:35 EDT — Deployment checkpoint and live acceptance

Final ./nix-build.sh check and ./nix-build.sh image-static passed (exit 0).
Native and aarch64-linux Nix verification agree: **211/0, 285/0, 9/0**. Nix
workspace tests: **39/0**. Receipts: logs/stage14-nix-final-check.log,
logs/stage14-nix-image.log, logs/stage14-nix-proof-summary.log and
logs/stage14-nix-test-summary.log (full tool-internal logs remain under logs/nix-*).
Only aarch64-linux was exercised; x86_64-linux was not run.

Deployed container proven-dispatch on **8096**, image sha256:609e691b43f05504928a20ca0e0a04d0dc89709e52685cbde95584dbb2271107
Image size **3,104,699 bytes**, ELF **1,529,848 bytes**. Export/layer
inspection finds exactly the application binary: no dynamic interpreter or
DT_NEEDED, no shell and no private-key file. User 65532:65532, read-only rootfs,
cap-drop ALL, no-new-privileges, read-only TLS and policy mounts all pass. The
existing TLS directory is reused solely through a read-only Docker bind mount;
no host TLS file was written, copied or logged.

python3 scripts/stage14-live.py passed using the existing acceptance.py checks:
TLS 1.3/ChaCha20/X25519/ECDSA success, TLS1.2/AES/P-256-key-exchange/h2 refusals,
plaintext refusal, curl, root/health/404, healthcheck, delayed
3-second request 200, partial request 400 and 17-second idle 0 bytes, capacity
limit, **48 malformed requests**, **32 TLS fixture replays** (8 fixtures x 4
fragmentations), four browser curl replays, duplicate/header-cap/privacy tests.
The 1,000-request/100-name log probe emits **59 new lines**, within the remaining
default 60-line sliding budget after the earlier DNT line. All pass.

Additional **14 rejected-request cases** against header-policy on 8095 and
proven-dispatch on 8096 produce byte-identical complete response buffers and
stdout suffixes, including bare CR/LF names and diagnostic precedence. The
comparison stores only diagnostic names/reasons, never header values. Receipts:
logs/stage14-live.log, logs/stage14-live/stage12-acceptance.json and
logs/stage14-live/rejection-equivalence.json. Full acceptance artifacts are
namespaced under logs/stage14-live. No physical-browser or separate LAN-device
check was performed; browser fixture replays are the required empirical checks.

Protected-container identity/pid/start-time snapshots compare equal with cmp:
scratch-server, nix-scratch, parser-http, parser-https and header-policy were not
restarted, stopped, removed or reconfigured. The only requests sent to 8095 were
the explicitly required rejection comparisons. No push or formal-coverage run.
The mutant campaign continues in temporary copies with runtime Rust held fixed.

## 2026-10-30 09:49 EDT — Stage 14 acceptance complete

The unchanged python3 verification/scripts/spec-mutants.py completed with exit 0: **15/15
killed, 0 survivors, 0 invalid**. Every mutant passed the pre-intent refinement
phase, now including the application proofs. Thirteen failed focused intent
lemmas; all fifteen failed real workspace tests. The runner's working-tree
Rust/config/fixture hash checks passed. Original internal receipts are under
logs/stage13-mutants; archived stage-14 receipts are logs/stage14-mutants with
results.json and per-mutant logs. Top-level log: logs/stage14-mutants.log.
MUTANTS.md records this revalidation without replacing historical evidence.

All acceptance checkboxes are complete. Final counts: **211/285/9 verified
with zero errors** per overlapping root, **39 workspace tests**, **15/15 killed**.
Equivalence: **100,000 header sequences**, **300,000 diagnostic comparisons**,
**256 byte values + 10,000 random sanitizer names**, **84,008 paired limiter
calls**, plus rollback/max-timestamp cases. The live 8096 results and complete
function/SLoC tables above are part of this final audit record.

Final protected-container snapshot again matches the pre-deployment snapshot
exactly, and proven-dispatch is healthy. The prohibited-token Rust scan still
has zero matches. Scope diff against 0cb7016 is empty for TLS, main, transport,
parser/policy, EVOLUTION.md and ATTACK-SURFACE.md. No push, branch switch or
formal-coverage invocation occurred. Commit identity remains repository-configured
Dudley. All writes remain in this repository or Docker; no external diary write.

Unverified by design: physical socket/stdout/atomic/mutex effects and their
wiring, Instant sampling and requested millisecond conversion, filesystem page
content, TLS adapters except inherited AEAD helpers, upstream crypto/OS/toolchain
and allocation success. Verus models the pure decisions, exact bytes and
transition contracts; it has no model of those OS/upstream effects. These were
tested where listed, not claimed proven. There is no new formal-coverage result
or percentage: Dudley runs that independently. Physical browsers, a separate LAN
device and x86_64 were not exercised. The stage is ready for Dudley's independent
acceptance; these are Codex's executed checks, not an independent signoff.

## 2026-10-30 09:51 EDT — Final function/SLoC inventory

Counts are physical nonblank, noncomment source lines. Runtime counts exclude
ghost contracts, loop specifications, proof blocks and assertions; inline code
counts once per physical line. Ghost-only functions have zero runtime SLoC and
a separate source count. This is a local source inventory, not formal-coverage;
Dudley supplies the compiled-files measurement. Old locations refer to 5b880fb.


### crates/transport-core/src/app.rs

| Function | Old location/origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `same` | new | crates/transport-core/src/app.rs:9 | proven core | 11 | 0 |
| `route` | src/lib.rs:108 | crates/transport-core/src/app.rs:25 | proven core | 5 | 0 |
| `route_status_spec` | new | crates/transport-core/src/app.rs:36 | proven ghost | 0 | 3 |
| `route_status` | src/lib.rs:108 | crates/transport-core/src/app.rs:39 | proven core | 3 | 0 |
| `code` | new | crates/transport-core/src/app.rs:42 | proven ghost | 0 | 3 |
| `status_code` | src/lib.rs:193 | crates/transport-core/src/app.rs:45 | proven core | 3 | 0 |
| `status_agrees` | new | crates/transport-core/src/app.rs:49 | proven ghost | 0 | 5 |
| `plan` | src/lib.rs:193 | crates/transport-core/src/app.rs:57 | proven core | 11 | 0 |
| `task_response` | src/lib.rs:108 | crates/transport-core/src/app.rs:77 | proven core | 13 | 0 |
| `safe` | new | crates/transport-core/src/app.rs:117 | proven ghost | 0 | 3 |
| `is_safe` | new | crates/transport-core/src/app.rs:120 | proven core | 3 | 0 |
| `filtered` | new | crates/transport-core/src/app.rs:124 | proven ghost | 0 | 6 |
| `sanitize_spec` | new | crates/transport-core/src/app.rs:130 | proven ghost | 0 | 3 |
| `sanitize` | src/lib.rs:57 | crates/transport-core/src/app.rs:133 | proven core | 11 | 0 |
| `crlf` | new | crates/transport-core/src/app.rs:155 | proven ghost | 0 | 3 |
| `line_end` | new | crates/transport-core/src/app.rs:158 | proven ghost | 0 | 6 |
| `scan_line` | new | crates/transport-core/src/app.rs:164 | proven core | 10 | 0 |
| `colon_end` | new | crates/transport-core/src/app.rs:181 | proven ghost | 0 | 6 |
| `scan_colon` | new | crates/transport-core/src/app.rs:187 | proven core | 10 | 0 |
| `stop` | new | crates/transport-core/src/app.rs:206 | proven ghost | 0 | 7 |
| `line_bounds` | new | crates/transport-core/src/app.rs:213 | proven ghost | 0 | 8 |
| `stop_bounds` | new | crates/transport-core/src/app.rs:221 | proven ghost | 0 | 9 |
| `next_from` | new | crates/transport-core/src/app.rs:230 | proven ghost | 0 | 12 |
| `next_spec` | new | crates/transport-core/src/app.rs:242 | proven ghost | 0 | 4 |
| `next_header_name` | src/lib.rs:66 | crates/transport-core/src/app.rs:248 | proven core | 15 | 0 |
| `diagnostic_valid` | new | crates/transport-core/src/app.rs:284 | proven ghost | 0 | 7 |
| `decision` | new | crates/transport-core/src/app.rs:291 | proven ghost | 0 | 9 |
| `diagnose_from` | new | crates/transport-core/src/app.rs:300 | proven ghost | 0 | 17 |
| `diagnose` | src/lib.rs:139 | crates/transport-core/src/app.rs:317 | proven core | 26 | 0 |
| `prefix` | new | crates/transport-core/src/app.rs:348 | proven ghost | 0 | 10 |
| `name_span` | new | crates/transport-core/src/app.rs:358 | proven ghost | 0 | 7 |
| `diagnostic_spec` | new | crates/transport-core/src/app.rs:365 | proven ghost | 0 | 5 |
| `diagnostic_line` | src/lib.rs:139 | crates/transport-core/src/app.rs:370 | proven core | 22 | 0 |
| `folded` | new | crates/transport-core/src/app.rs:406 | proven ghost | 0 | 3 |
| `lowercase` | new | crates/transport-core/src/app.rs:409 | proven core | 12 | 0 |
| `live` | new | crates/transport-core/src/app.rs:428 | proven ghost | 0 | 3 |
| `retained` | new | crates/transport-core/src/app.rs:431 | proven ghost | 0 | 6 |
| `prune` | new | crates/transport-core/src/app.rs:437 | proven core | 7 | 0 |
| `cap` | new | crates/transport-core/src/app.rs:450 | proven ghost | 0 | 1 |
| `present` | new | crates/transport-core/src/app.rs:451 | proven ghost | 0 | 3 |
| `UnknownLog::valid` | new | crates/transport-core/src/app.rs:456 | proven ghost | 0 | 5 |
| `UnknownLog::new` | new | crates/transport-core/src/app.rs:461 | proven core | 2 | 0 |
| `UnknownLog::should_log` | src/lib.rs:41 | crates/transport-core/src/app.rs:464 | proven core | 15 | 0 |
| `needs_root` | src/lib.rs:108 | crates/transport-core/src/app.rs:494 | proven core | 3 | 0 |
| `rejection_line` | src/lib.rs:171 | crates/transport-core/src/app.rs:497 | proven core | 8 | 0 |
| `next_unknown` | src/lib.rs:86 | crates/transport-core/src/app.rs:510 | proven core | 14 | 0 |
| `unknown_line` | src/lib.rs:86 | crates/transport-core/src/app.rs:531 | proven core | 12 | 0 |
| `recent` | new | crates/transport-core/src/app.rs:555 | proven ghost | 0 | 7 |
| `recent_bounds` | new | crates/transport-core/src/app.rs:562 | proven ghost | 0 | 6 |
| `retain_recent` | new | crates/transport-core/src/app.rs:569 | proven ghost | 0 | 12 |
| `emitted_count` | new | crates/transport-core/src/app.rs:581 | proven ghost | 0 | 5 |
| `log_transition` | new | crates/transport-core/src/app.rs:586 | proven ghost | 0 | 7 |
| `window_accounting` | new | crates/transport-core/src/app.rs:593 | proven ghost | 0 | 15 |
| `sliding_window_bound` | new | crates/transport-core/src/app.rs:611 | proven ghost | 0 | 12 |


### src/lib.rs

| Function | Old location/origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `ServerState::default` | src/lib.rs:23 | src/lib.rs:25 | unverified shell | 3 | 0 |
| `ServerState::with_policy` | src/lib.rs:28 | src/lib.rs:30 | unverified shell | 8 | 0 |
| `log_unknown` | src/lib.rs:86 | src/lib.rs:39 | unverified shell | 17 | 0 |
| `reply` | src/lib.rs:101 | src/lib.rs:56 | unverified shell | 6 | 0 |
| `root_body` | src/lib.rs:108 | src/lib.rs:62 | unverified shell | 16 | 0 |
| `serve_connection` | src/lib.rs:189 | src/lib.rs:80 | unverified shell | 3 | 0 |
| `serve_with_started` | src/lib.rs:193 | src/lib.rs:83 | unverified shell | 57 | 0 |
| `sanitized` | src/lib.rs:57 | src/lib.rs:143 | test adapter | 3 | 0 |
| `header_names` | src/lib.rs:66 | src/lib.rs:147 | test adapter | 8 | 0 |
| `rejection_line` | src/lib.rs:139 | src/lib.rs:156 | test adapter | 3 | 0 |
| `unknown_log_is_bounded_in_every_sliding_minute` | src/lib.rs:255 | src/lib.rs:164 | test only | 16 | 0 |
| `diagnostic_classes_and_privacy` | src/lib.rs:274 | src/lib.rs:181 | test only | 42 | 0 |


### src/dispatch_oracle.rs

| Function | Old location/origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `should_log` | src/lib.rs:41 | src/dispatch_oracle.rs:12 | test only | 15 | 0 |
| `sanitized` | src/lib.rs:57 | src/dispatch_oracle.rs:28 | test only | 8 | 0 |
| `header_names` | src/lib.rs:66 | src/dispatch_oracle.rs:37 | test only | 20 | 0 |
| `rejection_line` | src/lib.rs:139 | src/dispatch_oracle.rs:57 | test only | 31 | 0 |
| `stage14_compatibility_header_spans_contain_bare_line_breaks` | new | src/dispatch_oracle.rs:90 | test only | 11 | 0 |
| `stage14_compatibility_malformed_name_changes_rejection_diagnostic` | new | src/dispatch_oracle.rs:104 | test only | 17 | 0 |
| `stage14_preserved_oracle_sanitizer_and_limiter` | new | src/dispatch_oracle.rs:124 | test only | 23 | 0 |
| `next_random` | new | src/dispatch_oracle.rs:148 | test only | 6 | 0 |
| `stage14_header_and_diagnostic_equivalence_100000_blocks` | new | src/dispatch_oracle.rs:156 | test only | 100 | 0 |
| `stage14_sanitizer_equivalence_random_names` | new | src/dispatch_oracle.rs:258 | test only | 14 | 0 |
| `stage14_limiter_equivalence_sliding_windows` | new | src/dispatch_oracle.rs:274 | test only | 48 | 0 |


### tests/dispatch.rs

| Function | Old location/origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `read` | new | tests/dispatch.rs:17 | test only | 3 | 0 |
| `write` | new | tests/dispatch.rs:22 | test only | 4 | 0 |
| `flush` | new | tests/dispatch.rs:26 | test only | 3 | 0 |
| `shell_routes_get_and_head_with_exactly_one_handler_call` | new | tests/dispatch.rs:31 | test only | 34 | 0 |
| `plans_never_respond_or_increment_except_as_specified` | new | tests/dispatch.rs:66 | test only | 33 | 0 |


Executable function-body/signature inventory totals: **216 proven core SLoC**,
**110 unverified shell SLoC** in src/lib.rs, excluding test adapters,
types and imports. There were no prior app.rs functions. Pure dispatch/reject
logic now lives in task_response; only dynamic root-body IO stays in root_body.
The old production UnknownLog, sanitizer, diagnostic iterator and rejection_line
implementations are removed from the shell and retained only as test oracles.
Ghost helpers carry no shipped runtime SLoC. These source totals do not assert
a formal-coverage numerator, denominator or percentage.

## 2026-10-30 09:51 EDT — Inventory audit correction

Final review corrected the local counting script to recognize pub(crate),
multiline function signatures and the cfg(test) boundary. The original inventory
misclassified two tests as shell functions and omitted serve_with_started. The
corrected tables above include every function and report **216 proven runtime
core SLoC, 110 unverified runtime shell SLoC** (function signatures/bodies only).
The proven-core count is unchanged. The superseded draft is retained under
logs/stage14-inventory-first-draft.md for audit. Runtime source, proof counts,
workspace/Nix results, mutant receipts and deployed image are unchanged.

## 2026-10-30 11:02 EDT — Stage 15 started

Read the authoritative PROVEN-RECORD.md first and all required context. Branch proven-record, initial working tree clean. Module declaration follows aead.rs/app.rs at Rust module scope with its own verus! block. Only permitted behavior change: encrypted_payload_len saturates above usize::MAX - 17; production rustls lengths are bounded to 16,384. No external diary write because this task confines writes to this repository, TMPDIR and Docker.

## 2026-10-30 11:06 EDT — Stage 15 function/SLoC inventory

Physical nonblank/noncomment source lines, including signatures and braces. Runtime SLoC excludes ghost contracts, loop specifications, proof blocks and assertions; ghost-only functions have zero runtime SLoC. This is a source inventory, not formal-coverage. Old source is 13c8124. Repeated method names are numbered in source order. All TLS production functions remain unverified shell; their calls into the core do not prove the surrounding trait wiring.

| TLS function | Old line | New line | Before SLoC | After SLoC | Status |
| --- | ---: | ---: | ---: | ---: | --- |
| `provider#1` | 21 | 21 | 12 | 12 | shell |
| `fill#1` | 48 | 48 | 3 | 3 | shell |
| `try_next_u32#1` | 54 | 54 | 5 | 5 | shell |
| `try_next_u64#1` | 59 | 59 | 5 | 5 | shell |
| `try_fill_bytes#1` | 64 | 64 | 3 | 3 | shell |
| `start#1` | 74 | 74 | 3 | 3 | shell |
| `hash#1` | 77 | 77 | 3 | 3 | shell |
| `output_len#1` | 80 | 80 | 3 | 3 | shell |
| `algorithm#1` | 83 | 83 | 3 | 3 | shell |
| `fork_finish#1` | 88 | 88 | 5 | 5 | shell |
| `fork#1` | 93 | 93 | 3 | 3 | shell |
| `finish#1` | 96 | 96 | 3 | 3 | shell |
| `update#1` | 99 | 99 | 3 | 3 | shell |
| `extract_from_zero_ikm#1` | 107 | 107 | 3 | 3 | shell |
| `extract_from_secret#1` | 110 | 110 | 11 | 11 | shell |
| `expander_for_okm#1` | 122 | 122 | 3 | 3 | shell |
| `hmac_sign#1` | 125 | 125 | 5 | 5 | shell |
| `expand_slice#1` | 132 | 132 | 9 | 9 | shell |
| `expand_block#1` | 141 | 141 | 6 | 6 | shell |
| `hash_len#1` | 147 | 147 | 3 | 3 | shell |
| `start#2` | 159 | 159 | 8 | 8 | shell |
| `name#1` | 167 | 167 | 3 | 3 | shell |
| `complete#1` | 172 | 172 | 7 | 7 | shell |
| `pub_key#1` | 179 | 179 | 3 | 3 | shell |
| `group#1` | 182 | 182 | 3 | 3 | shell |
| `encrypter#1` | 193 | 193 | 3 | 3 | shell |
| `decrypter#1` | 196 | 196 | 3 | 3 | shell |
| `key_len#1` | 199 | 199 | 3 | 3 | shell |
| `extract_keys#1` | 202 | 202 | 7 | 7 | shell |
| `encrypt#1` | 211 | 211 | 31 | 33 | shell |
| `encrypted_payload_len#1` | 242 | 244 | 3 | 3 | shell |
| `decrypt#1` | 247 | 249 | 26 | 26 | shell |
| `unpad_message#1` | new | 279 | 0 | 10 | shell |
| `public_key_alg_id#1` | 279 | 293 | 3 | 3 | shell |
| `signature_alg_id#1` | 282 | 296 | 3 | 3 | shell |
| `verify_signature#1` | 285 | 299 | 21 | 21 | shell |
| `fmt#1` | 316 | 330 | 3 | 3 | shell |
| `load_private_key#1` | 321 | 335 | 31 | 31 | shell |
| `fmt#2` | 355 | 369 | 3 | 3 | shell |
| `choose_scheme#1` | 360 | 374 | 5 | 5 | shell |
| `algorithm#2` | 365 | 379 | 3 | 3 | shell |
| `public_key#1` | 368 | 382 | 3 | 3 | shell |
| `scheme#1` | 373 | 387 | 3 | 3 | shell |
| `sign#1` | 376 | 390 | 13 | 13 | shell |
| `load_config#1` | 391 | 405 | 8 | 8 | shell |
| `server_config#1` | 400 | 414 | 12 | 12 | shell |
| `der_bytes#1` | 430 | 444 | 3 | 3 | test only |
| `algorithm#3` | 433 | 447 | 3 | 3 | test only |
| `sign#2` | 438 | 452 | 8 | 8 | test only |
| `configs#1` | 447 | 461 | 37 | 37 | test only |
| `exchange#1` | 484 | 498 | 18 | 18 | test only |
| `exchange_policy#1` | 502 | 516 | 55 | 55 | test only |
| `idle_and_partial_deadlines_tcp_and_tls#1` | 558 | 572 | 89 | 89 | test only |
| `idle_handshake_budget_is_absolute#1` | 655 | 669 | 39 | 39 | test only |
| `tls_routes_and_safari#1` | 697 | 711 | 51 | 51 | test only |
| `default_policy_browser_fixtures_and_sensitive_names#1` | 749 | 763 | 87 | 87 | test only |
| `tls_case_and_count_boundaries#1` | 837 | 851 | 70 | 70 | test only |
| `malformed_tls_never_dispatches#1` | 908 | 922 | 59 | 59 | test only |
| `aead_rejects_tampered_record_and_x25519_rejects_low_order#1` | 969 | 983 | 16 | 16 | test only |
| `hkdf_rfc5869_case1_and_hash_fork#1` | 986 | 1000 | 30 | 30 | test only |

| Core function | Origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| `all_zero` | new | record.rs:10 | proven ghost | 0 | 3 |
| `unpad` | rustls into_tls13_unpadded_message via src/tls.rs:272 | record.rs:13 | proven core | 16 | 0 |
| `aad` | rustls make_tls13_aad via src/tls.rs:230,263 | record.rs:46 | proven core | 6 | 0 |
| `inner_plaintext` | src/tls.rs:217–219 | record.rs:57 | proven core | 10 | 0 |
| `encrypted_payload_len` | src/tls.rs:242 | record.rs:75 | proven core | 4 | 0 |
| `x25519_peer` | src/tls.rs:174 | record.rs:81 | proven core | 9 | 0 |
| `p256_point_shape` | src/tls.rs:294 | record.rs:95 | proven core | 2 | 0 |
| `offers` | src/tls.rs:360 | record.rs:98 | proven core | 10 | 0 |

New shell helper `unpad_message` is included in the TLS table. It maps core errors, sets rustls type/version and slices through the public into_plain_message_range API. No upstream private API is used.

| Added test function | Location | Status | SLoC |
| --- | --- | --- | ---: |
| `random` | src/record_tests.rs:4 | test only | 6 |
| `compare_unpad` | src/record_tests.rs:11 | test only | 13 |
| `stage15_unpad_equivalence_100000` | src/record_tests.rs:26 | test only | 16 |
| `stage15_aad_equivalence_80001` | src/record_tests.rs:44 | test only | 10 |
| `stage15_inner_and_length_equivalence` | src/record_tests.rs:56 | test only | 33 |
| `stage15_key_shape_and_offers_equivalence` | src/record_tests.rs:92 | test only | 22 |

Seven new executable core functions total **57 runtime SLoC**; all_zero is ghost-only. The extra shell payload gather is needed because rustls OutboundChunks is not a plain slice. Both gathered payload and constructed inner plaintext are Zeroizing-owned; allocation success and temporary Vec ownership transfer are trusted. All existing production TLS functions remain in place. No compiled-files coverage percentage is claimed.

## 2026-10-30 11:06 EDT — Stage 15 core checkpoint

./verification/verify.sh passed: parser 211/0, transport 298/0, standalone AEAD 9/0 (overlapping roots). New record module adds 13 results. Receipt logs/stage15-core-verify.log. Focused record verification also passes 13/0. The initial bit-vector assertion needed to state the expression directly rather than refer to local aliases; no contract was weakened. PROOFS.md records all six properties and remaining shell limitations. Runtime source inventory: seven proven helpers, 57 SLoC.

## 2026-10-30 11:07 EDT — Stage 15 shell checkpoint

TLS shell delegates unpadding/AAD/inner construction/length/key-share shape/point shape/scheme membership to record.rs. Errors are mapped to the identical rustls variants and version TLSv1_3; into_plain_message_range borrows the original payload. libcrux calls and all-zero ECDH rejection are retained. All four new equivalence tests passed; full inherited workspace suite is still completing. Final full verifier receipt logs/stage15-core-verify-final.log confirms 211/298/9, zero errors.

## 2026-10-30 11:08 EDT — Stage 15 equivalence and workspace acceptance

Full cargo test --workspace passed, 43 tests/0 failures: all 39 inherited tests unchanged in meaning, plus four new tests. New equivalence counts: 100,000 unpadding cases (including all types, zero buffers, CCS with/without padding, required boundary lengths, long zero runs and random bytes); 80,001 AAD cases (70,001 consecutive lengths plus 10,000 random usize); 10,000 inner constructions and actual encrypted records, covering every content type, empty/full/random payloads and fragmented OutboundChunks; 20,019 length checks (10,000 bounded payload lengths, 10,000 random representable lengths, 19 near-MAX boundary/saturation cases); 25,856 peer copies and 25,856 point-shape comparisons (lengths 0–100 x every prefix); 10,000 scheme lists plus 10,000 arbitrary-wanted membership checks. Zero oracle differences. Receipt logs/stage15-tests.log; assertions do not dump record bytes.

Final verifier 211/298/9, zero errors; first-party tracked Rust scan for all five prohibited tokens: zero matches (logs/stage15-prohibited-scan.log). Started unchanged spec-mutant runner in temporary copies and ./nix-build.sh check; runtime Rust is held fixed during the campaign. The prescribed Nix wrapper uses transient release-nix build containers and existing Nix cache volume. Only runtime container created/replaced will be proven-record on 8097.

## 2026-10-30 11:14 EDT — Stage 15 deployment and live acceptance

./nix-build.sh check and ./nix-build.sh image-static passed, exit 0. aarch64-linux Verus counts match native: 211/298/9, zero errors; 43 Nix workspace tests pass. Only aarch64-linux exercised, not x86_64. Receipts logs/stage15-nix-check.log, stage15-nix-image.log, stage15-nix-proof-full.log and stage15-nix-test-full.log.

Deployed proven-record on 8097: image sha256:234a8b3036b68ca1b784e4d8e62576aaa21115522d6e5f5d2012d911ddd3ee89, image 3,104,699 bytes, ELF 1,529,560 bytes. Existing TLS directory is bound read-only, never copied or written. User 65532:65532, read-only rootfs, cap-drop ALL, no-new-privileges and read-only policy mount verified. Export and saved-image layer inspection find only the application binary, no shell/key, no ELF interpreter or DT_NEEDED.

python3 scripts/stage15-live.py passed, reusing acceptance.py unchanged: TLS1.3/ChaCha20/X25519/ECDSA P-256 success; TLS1.2/AES/P-256 key exchange/h2 and plaintext HTTP refused; curl/routes/healthcheck pass; delayed 3-second request 200; partial timeout 400; 17-second idle closes with 0 bytes. All 48 malformed requests, 32 TLS browser fixture replays (8 x 4 fragmentation sizes), four curl browser replays, capacity and privacy checks pass. The 1,000-request/100-name log probe produces 59 new lines within the remaining 60-line sliding budget. Eighteen scripted requests (root/health/404/HEAD plus 14 rejections) give byte-identical full responses and stdout suffixes against proven-dispatch on 8096. No physical browser or separate LAN device check performed. Receipts logs/stage15-live.log and logs/stage15-live/*.json.

Protected-container identity/pid/start-time snapshots compare equal with cmp: scratch-server, nix-scratch, parser-http, parser-https, header-policy, proven-dispatch. Only proven-record was created. No push, branch switch, formal-coverage invocation or modification of EVOLUTION.md/ATTACK-SURFACE.md. Mutant campaign remains active; first mutant killed, remaining results pending.

The added live-runner functions repeat stage 14 tooling for 8097/8096; they are not shipped runtime code. Physical nonblank/noncomment function SLoC:

| Function | Origin | New location | Status | SLoC |
| --- | --- | --- | --- | ---: |
| `stdout` | scripts/stage14-live.py | scripts/stage15-live.py:22 | acceptance tooling, unproven | 2 |
| `exchange` | scripts/stage14-live.py | scripts/stage15-live.py:26 | acceptance tooling, unproven | 9 |
| `compare` | scripts/stage14-live.py | scripts/stage15-live.py:37 | acceptance tooling, unproven | 33 |

## 2026-10-30 11:32 EDT — Stage 15 final acceptance record

The unchanged spec-mutant runner completed with exit 0: **15/15 killed,
0 survivors, 0 invalid**. All fifteen pre-intent refinement phases passed,
including the new record proofs. Thirteen focused intent lemmas killed faults;
all fifteen actual workspace test runs failed as expected for the deliberately
incorrect programs. Temporary-copy and working-tree hash checks passed. Archived
receipts: logs/stage15-mutants/results.json and per-mutant logs; top-level
logs/stage15-mutants.log. MUTANTS.md preserves the earlier stage evidence.

All stage 15 acceptance command checkboxes are complete. Verified per root:
**211 parser / 298 transport / 9 standalone AEAD**, zero errors (overlapping
roots), including **13 new record results**. Workspace and aarch64-linux Nix
both pass **43 tests**, preserving all 39 inherited tests. No prohibited token
matches in tracked first-party Rust. Nix check/static-image and live 8097 checks
pass, with 18 byte-identical response/stdout comparisons against 8096. Full new
equivalence counts and function inventories are above; no old-code/rustls bug
was exposed. Saturation above usize::MAX - 17 remains the sole authorized
behavior difference.

Local source measurement: seven new proven executable helpers, **57 SLoC**;
one ghost helper all_zero, **0 runtime SLoC / 3 ghost source SLoC**. TLS has
45 existing production functions, all retained in the shell: encrypt changes
31→33 SLoC, each of the other 44 is unchanged in count. Added shell helper
unpad_message has **10 SLoC**. Thus TLS production function SLoC is **305→317**.
The full per-function before/after table above includes all 60 resulting TLS
functions, including unchanged test functions; the record and added test/tool
function tables identify origin, new location, status and SLoC. These counts
are physical source inventories, not compiled-files coverage. Dudley measures
formal-coverage independently; it was never run or modified here.

Final audit logs/stage15-final-audit.log confirms: proven-record healthy;
all six protected container identities/pids/start times unchanged; branch still
proven-record; no diff in out-of-scope sources against initial a6a0bf8; no push;
no EVOLUTION.md/ATTACK-SURFACE.md edit; no prohibited Rust tokens; whitespace
check passed. Commits use configured Dudley identity. Writes stayed inside this
repository or Docker; no external diary entry was made.

Unproven by design: rustls type/error/range wiring, trait integration, payload
gathering, allocation/Zeroizing effects, upstream crypto/DER and TLS state
machine, RNG, private-key loading, hash/HMAC/HKDF/signing, config/IO, compiler,
OS and hardware. No sound local models/contracts exist for those upstream/OS
effects under the ban on opaque proof escapes. All six requested pure properties
are proven; selected shell effects are tested, not claimed proven. No end-to-end
TLS, secrecy/timing, resource-success or upstream release/target proof is claimed.
Physical browsers, separate LAN device and x86_64 were not exercised. These are
Codex's executed checks; Dudley's independent acceptance is still required.

## 2026-10-30 13:36 EDT — Stage 15 plaintext capacity fix started

Andrew requested one allocation fix on `proven-record`, starting at 37fb951.
`inner_plaintext` now reserves `payload.len() + 1` before copying bytes. The
existing `sealed_len` guard excludes overflow; behavior and the `ensures` clause
are unchanged. Added one capacity regression for lengths 0, 1, 255 and 16,384.
Checks are running with receipts under `logs/stage15-capacity-`.

The mutant runner hardcodes `logs/stage13-mutants`. Run its original source via
Python with only that output-directory literal redirected in memory to
`logs/stage15-capacity-mutants`; keep `__file__` at the original script path.
No runner file, mutation logic, proof phase, or test command changes. This keeps
all campaign receipts under the requested prefix and preserves older logs.
No containers, deployment, external memory writes, or files outside this repo
are in scope for this fix.

## 2026-10-30 14:01 EDT — Stage 15 plaintext capacity fix verified

`./verification/verify.sh` exited 0: **211 parser / 298 transport / 9 standalone AEAD**,
zero errors, unchanged counts. `Vec::with_capacity(payload.len() + 1)` required
no proof adjustment; the existing guard proves the addition safe and the existing
loop and `ensures` clause remain unchanged. Reserving the final length before
any plaintext push prevents growth reallocations during construction.

Stage 15's five prohibited word tokens have **zero matches** across all 18
tracked first-party Rust files. `cargo test --workspace` exited 0 with **44
passed / 0 failed**, including all 43 inherited tests and the new exact-capacity
test at payload lengths 0, 1, 255 and 16,384. The test checks returned capacity;
it does not instrument the allocator or claim an end-to-end secrecy proof.

The full spec-mutant campaign exited 0: **15/15 killed, 0 survivors, 0 invalid**;
all refinement phases pass and every mutated workspace test run fails with
actual test failures. The runner's working-tree source hash checks pass.
Receipts: `logs/stage15-capacity-verify.log`, `-prohibited-scan.log`, `-tests.log`,
`-mutants.log`, and `logs/stage15-capacity-mutants/` (including results.json).
Only the documented log-directory redirect was used; no algorithm changes.

Final scope: one allocation line, one nine-line test addition, these notes,
PROGRESS.md and receipts. No deployment/image/container actions, branch changes,
push, protected-document edits or writes outside this repository. These are
Codex's executed checks; independent acceptance remains Dudley's. Nothing
unexpected surfaced; the test count increases by one as requested.

## 2026-10-30 14:16 EDT — Stage 15 capacity fix, owner check

Andrew reports the redeployed proven-record on 8097 (image sha256:809255c5…, commit 6860b91) works from his side. This is the first human check of this build; Dudley's automated live acceptance passed at 14:15.

## 2026-10-30 15:48 EDT — Stage 16 blocked by admission equivalence

Read PROVEN-IO.md fully first, then PROVEN-RECORD.md, PROVEN-DISPATCH.md,
IDLE-CONNECTIONS.md, PROOFS.md, MUTANTS.md, PROGRESS.md and NOTES.md. Started
on proven-io at 5ca8b97 with a clean working tree. No production code changed.

The required property-3 oracle exposes an overflow in the old admission closure
at src/main.rs:95–96 (unchanged from 6860b91):
`(n < 64).then_some(n + 1)`. Rust eagerly evaluates the then_some argument even
when n < 64 is false. At usize::MAX the old closure panics under debug overflow
checks, whereas the specified function must return None. Counts 0..=200 agree
exactly. This is an old arithmetic bug at an explicitly required oracle input,
not evidence of a reachable network denial of service: active starts at zero,
successful admission caps it at 64, and production releases disable overflow
checks. Release-mode testing confirms that the unused argument wraps and the
old closure returns None. No live connections or containers were used.

Following the mandatory stop/commit-evidence rule in PROVEN-IO.md (also user
non-negotiable 6), stopped before implementing io.rs, rewiring the shell, changing
the deadline or fixing the old closure. The evidence test preserves the closure
verbatim and compares it with a test-only expression of the specified admission
rule. Its MAX test asserts the observed debug panic and the matching release
result; a passing evidence test does not mean universal admission equivalence.

Both commands passed with two tests each:
`cargo test --test stage16_admission -- --nocapture` and
`cargo test --release --test stage16_admission`, using repository-local
CARGO_HOME/RUSTUP_HOME. Receipts: logs/stage16-admission-debug.log (contains the
caught overflow panic/location) and logs/stage16-admission-release.log. There
are 201 equal comparisons plus one differing debug boundary, repeated in release
with all 202 equal. No deadline/classification/port equivalence cases ran.

Suggested resolution for Andrew: authorize the guarded addition required by
property 3 as an additional permitted difference for unreachable counts under
overflow-checking builds, and explicitly permit resuming despite this evidence.
Shipped release outputs already agree at MAX. No production repair, spec
amendment, proof weakening or existing-test weakening was made here.

### Stage 16 inventory at blocker

Physical nonblank/noncomment whole-file source lines (including imports, types,
braces and existing test code; not a formal-coverage measurement):

| File | Before (5ca8b97) | After |
| --- | ---: | ---: |
| src/main.rs | 150 | 150 |
| src/transport.rs | 64 | 64 |
| src/lib.rs | 218 | 218 |

All three files are byte-identical to the starting commit. No production function
moved or was added; additional proven runtime SLoC is zero. No io.rs exists.
Existing production shell functions retain their previous status.

| Function | Old location/origin | New location | Status | Function SLoC |
| --- | --- | --- | --- | ---: |
| old_admit | src/main.rs:95–96 closure body | tests/stage16_admission.rs:4 | test-only old oracle, unproven | 3 |
| specified_admit | new, property 3 expression | tests/stage16_admission.rs:9 | test only, unproven | 7 |
| stage16_admission_equivalence_0_through_200 | new | tests/stage16_admission.rs:18 | test only, unproven | 5 |
| stage16_admission_max_exposes_old_overflow | new | tests/stage16_admission.rs:25 | test only, unproven | 10 |

Inventory receipt: logs/stage16-blocker-inventory.json. Test function SLoC counts
physical nonblank/noncomment signatures and bodies, including test assertions.

Stage acceptance remains incomplete: no new properties proven, TRUSTED-BASE.md
not created (no stage-16 subtotals), deadline timings plain/TLS not measured,
spec mutants not run (no stage-16 killed/15 result), Nix check/image not run,
8098 live checks and 8098-vs-8097 trickle test not run, no deployment. These are
deliberately deferred under the stop rule, not implied by prior-stage results.
No external diary write: this task restricts writes to the repository, TMPDIR
and Docker. This NOTES.md preserves the audit record for Dudley.

## 2026-10-30 15:49 EDT — Stage 16 blocker workspace validation

Full `cargo test --workspace` exited 0: **46 passed / 0 failed**, all 44
inherited tests unchanged plus the two admission evidence tests. The spec's
43-test baseline predates the stage-15 capacity regression; 44 is the actual
inherited count at 5ca8b97. Receipt: logs/stage16-blocker-workspace.log.
Prohibited-token scan passed with **zero matches across 19 first-party Rust
files**, including the new evidence test (logs/stage16-prohibited-scan.log).
`git diff --check` passed. Production source, proofs, authoritative spec and
protected documents remain byte-identical to 5ca8b97. No Docker commands,
network/live probes, formal-coverage, branch switches, push or external writes
were performed. Baseline full Verus is still completing; no new proof claim.

## 2026-10-30 15:50 EDT — Stage 16 baseline proofs and blocker checkpoint

`./verification/verify.sh` exited 0: **211 parser / 298 transport / 9 standalone AEAD
verified, zero errors** (overlapping roots). Receipt:
logs/stage16-baseline-verify.log. These are unchanged baseline proofs; **zero
stage-16 properties** have been added or proven. The admission boundary evidence
requires an explicit resolution before implementation resumes. PROOFS.md stays
unchanged because no new proof exists. Checked first-party source/doc scope
against 5ca8b97 and git diff --check again; only NOTES.md, PROGRESS.md and the
new test file are changed. Commit uses the configured Dudley identity.

## 2026-10-30 15:59 EDT — Stage 16 resumed after admission approval

HEAD 401bbe2 contains Andrew's admission amendment. Implemented io.rs under the
existing transport root: deadline arithmetic and per-read bound, read
classification/start flag, guarded admission, saturating release/round trip,
port syntax and numeric completeness, shared crypto sizes. Focused I/O Verus
passes 19/0; full ./verification/verify.sh is running. Shell converts nanosecond clock samples
with saturation, records the first decrypted byte once, maps error kinds while
retaining the original error, and uses the proven admission/release decisions
inside fetch_update. No other behavior change is intended.

Small interpretation choices: module declaration is at Rust module scope, with
its own verus! block, matching the existing aead/app/record pattern. The spec's
banned token `admit` is also its required API name; scans will report all such
identifier occurrences separately from forbidden proof primitives. No Verus
admit primitive is present. Key-shape APIs now use the shared size constants in
both types and contracts; no prior guarantee was weakened.

Oracle development found two harness count mistakes, not production differences:
the 15-symbol port alphabet yields 12,204,241 strings, and the OS plus stable
ErrorKind constructors expose 44 distinct variants in the pinned toolchain.
FilesystemLoop/InProgress are unstable names but stable raw-errno conversion
constructs them, plus InputOutputError/TooManyOpenFiles/Uncategorized. Tests cover
all stable constructors plus errno -1..=4096 and assert all 44 observed kinds.
Port equivalence has zero differences across the exhaustive set. Existing
43-test wording predates the capacity regression: inherited production baseline
is 44 tests; the two blocker tests are retained and updated for the approved fix.

The unmodified live suite fills all 64 workers, explicitly prohibited for this
stage. Added an opt-in --skip-capacity option (default behavior unchanged) and
--image-tag to acceptance.py; stage16-live.py sets those and compares 8098 to
8097 using the existing 18-request set. It will report capacity=false, and use
only three concurrent idle probes or two comparative trickle connections.
Admission at the cap is covered by the proven core and local atomic tests, not
by a live cap-filling check. This is an explicit user constraint, not a test
silently weakened. flake.nix uses the image name proven-io so nix-build.sh never
loads/overwrites scratch-server:static; wrapper supports NIX_LOG_PREFIX for
stage16-prefixed internal logs. No protected container will be reconfigured.

## 2026-10-30 16:03 EDT — Stage 16 core and shell checkpoints verified

Full ./verification/verify.sh exited 0: **211 parser / 317 transport / 9 standalone AEAD**,
zero errors; io adds 19 results. Receipt logs/stage16-core-verify-final.log.
Prohibited-construct scan across 22 first-party Rust files finds zero forbidden
proof/escape constructs; the required admission API identifier is reported
separately with every location in logs/stage16-prohibited-scan-final.log.

Full cargo test --workspace -- --nocapture exited 0: **52 passed / 0 failed**,
including all 44 inherited tests, the two updated blocker tests, and six more
stage-16 tests. The release admission test command also passes all three tests
(logs/stage16-admission-release-final.log). No other old-code difference found.
Logs: logs/stage16-tests.log. Deadline plain/TLS timings: trickle 400 at
10.003/10.003 s (both logged reason=slow); eight-second complete request 200 at
8.038/8.041 s; after 3 s idle, request-to-response 0.004/0.001 s, both 200;
idle closure observed at 15.002/14.943 s since post-handshake client start, zero
bytes, asserted again after full 17 s. The server idle clock includes handshake.
At most two new local slow connections run concurrently. Inherited tests remain
unchanged in meaning.

Equivalence counts: 100,000 deadline pairs, compared with request deadline both
disabled and enabled; 39 stable error constructors plus 4,098 raw errno errors
and Ok(0/1/1024), all 44 distinct ErrorKind values observed, plus Data(MAX);
201 exact old/new admission counts and MAX approved exception in both profiles;
202 atomic admit/release inputs including MAX. Ports: 12,204,241 exhaustive byte
strings + 10,000 random strings + 7 explicit boundaries = **12,214,248** parser
comparisons, plus unset/non-UTF-8/default/error shell tests. No payload or header
value appears in assertion output. The only caught panic is historical MAX
admission evidence, now approved and documented.

## 2026-10-30 16:01 EDT — Stage 16 function/SLoC inventory

Old location is 401bbe2 (production source unchanged from the original 5ca8b97). Physical nonblank/noncomment source lines include signatures/braces. Core runtime excludes contracts, loop specifications, assertions and proof blocks. Ghost functions have zero runtime SLoC. This source audit does not run formal-coverage.

| File | Whole file before | After | Production source before | After | Production function SLoC before | After |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| src/main.rs | 150 | 171 | 150 | 149 | 137 | 136 |
| src/transport.rs | 64 | 70 | 64 | 70 | 44 | 49 |
| src/lib.rs | 218 | 226 | 135 | 141 | 110 | 116 |
| src/tls.rs | 1005 | 1006 | 408 | 408 | 317 | 317 |

| Function | Old origin | New location | Status | Runtime SLoC | Ghost source SLoC |
| --- | --- | --- | --- | ---: | ---: |
| request_end | new | crates/transport-core/src/io.rs:13 | proven ghost | 0 | 4 |
| timeout_spec | new | crates/transport-core/src/io.rs:17 | proven ghost | 0 | 9 |
| read_timeout | src/transport.rs:23 inline timeout calculation | crates/transport-core/src/io.rs:26 | proven core | 12 | 0 |
| deadline_bound | new | crates/transport-core/src/io.rs:51 | proven ghost | 0 | 6 |
| classify | src/lib.rs:93 inline read match | crates/transport-core/src/io.rs:62 | proven core | 10 | 0 |
| admission_result | new | crates/transport-core/src/io.rs:78 | proven ghost | 0 | 3 |
| release_result | new | crates/transport-core/src/io.rs:81 | proven ghost | 0 | 3 |
| admit | src/main.rs:95 closure | crates/transport-core/src/io.rs:84 | proven core | 4 | 0 |
| release | src/main.rs:113 fetch_sub decrement | crates/transport-core/src/io.rs:90 | proven core | 4 | 0 |
| admission_round_trip | new | crates/transport-core/src/io.rs:95 | proven ghost | 0 | 5 |
| port_start | new | crates/transport-core/src/io.rs:101 | proven ghost | 0 | 3 |
| digit_range | new | crates/transport-core/src/io.rs:104 | proven ghost | 0 | 3 |
| port_form | new | crates/transport-core/src/io.rs:107 | proven ghost | 0 | 5 |
| decimal_prefix | new | crates/transport-core/src/io.rs:112 | proven ghost | 0 | 11 |
| parse_port | src/main.rs:16 port parse | crates/transport-core/src/io.rs:123 | proven core | 22 | 0 |
| sizes_agree | new | crates/transport-core/src/io.rs:180 | proven ghost | 0 | 3 |
| port#1 | src/main.rs:16 | src/main.rs:16 | shell | 5 | 0 |
| run#1 | src/main.rs:36 | src/main.rs:35 | shell | 79 | 0 |
| read#1 | src/transport.rs:22 | src/transport.rs:23 | shell | 8 | 0 |
| serve_tcp#1 | src/transport.rs:47 | src/transport.rs:44 | shell | 31 | 0 |
| elapsed_ns#1 | new | src/transport.rs:77 | shell | 4 | 0 |
| serve_with_started#1 | src/lib.rs:83 | src/lib.rs:83 | shell | 52 | 0 |
| read_result#1 | new | src/lib.rs:137 | shell | 11 | 0 |
| output_len#1 | src/tls.rs:80 | src/tls.rs:80 | shell | 3 | 0 |
| hash_len#1 | src/tls.rs:147 | src/tls.rs:147 | shell | 3 | 0 |
| x25519_peer | crates/transport-core/src/record.rs:81 | crates/transport-core/src/record.rs:82 | proven core, shared constants only | 9 | 0 |
| p256_point_shape | crates/transport-core/src/record.rs:95 | crates/transport-core/src/record.rs:96 | proven core, shared constants only | 2 | 0 |

| Test/helper function | Old origin | New location | Status | SLoC |
| --- | --- | --- | --- | ---: |
| old_classify | new | src/io_tests.rs:5 | test only, unproven | 10 |
| stage16_classification_all_error_kinds | new | src/io_tests.rs:16 | test only, unproven | 24 |
| random | new | src/io_tests.rs:42 | test only, unproven | 6 |
| old_timeout | new | src/io_tests.rs:50 | test only, unproven | 7 |
| stage16_deadline_equivalence_100000 | new | src/io_tests.rs:58 | test only, unproven | 29 |
| compare_port | new | src/io_tests.rs:89 | test only, unproven | 4 |
| stage16_port_exhaustive_and_random | new | src/io_tests.rs:95 | test only, unproven | 27 |
| stage16_request_deadlines_plain_and_tls | new | src/io_deadline_tests.rs:3 | test only, unproven | 84 |
| old_admit | blocker evidence, updated | tests/stage16_admission.rs:4 | test only, unproven | 3 |
| stage16_admission_equivalence_0_through_200 | blocker evidence, updated | tests/stage16_admission.rs:9 | test only, unproven | 5 |
| stage16_admission_max_exposes_old_overflow | blocker evidence, updated | tests/stage16_admission.rs:16 | test only, unproven | 10 |
| stage16_atomic_admission_and_release | new | tests/stage16_admission.rs:32 | test only, unproven | 18 |
| stage16_port_unset_non_utf8_and_error | new | src/main.rs:161 | test only, unproven | 18 |

scripts/stage16-live.py repeats stdout/exchange/compare from stage15-live.py and adds trickle/trickle_compare (unproven acceptance tooling). acceptance.py main/idle_checks add image-tag selection and an explicit opt-in capacity-probe omission; default checks remain intact. nix-build.sh adds only log-prefix selection; flake.nix changes only the image name. Audit generator logs/stage16-inventory.py is unshipped measurement tooling.

New IO executable core: **52 runtime SLoC**. TRUSTED-BASE.md totals **63 unproven production functions / 618 SLoC**, with per-function reasons, test coverage and explicit gaps. Every TLS production function remains shell. Runtime constants are not function bodies.

Stage 16 acceptance-tool function inventory (physical nonblank/noncomment lines; all unproven, not shipped):

| Function | Origin | New location | SLoC |
| --- | --- | --- | ---: |
| stdout | scripts/stage15-live.py | scripts/stage16-live.py:25 | 2 |
| exchange | scripts/stage15-live.py | scripts/stage16-live.py:29 | 9 |
| compare | scripts/stage15-live.py | scripts/stage16-live.py:40 | 33 |
| trickle | new | scripts/stage16-live.py:75 | 33 |
| trickle_compare | new | scripts/stage16-live.py:110 | 5 |

## 2026-10-30 16:09 EDT — Stage 16 Nix and deployment checkpoint

NIX_LOG_PREFIX=stage16-nix ./nix-build.sh check and image-static both exited 0. ARM Linux proofs repeat 211/317/9, zero errors; Nix workspace repeats 52 tests, zero failures. x86_64 was not run. Receipts: logs/stage16-nix-check.log, logs/stage16-nix-image.log, logs/stage16-nix-proof-log.log and logs/stage16-nix-test-log.log. The runner's only change is log-path selection.

Deployed only proven-io on 8098 from proven-io:static, sha256:5d1184971b5b66eafc1a76c8f6dad7abf5f5ad6bb6fd7572dac38331c238edab, 3,104,694 image bytes. Verified image tag, user 65532:65532, read-only rootfs, cap-drop ALL, no-new-privileges, read-only TLS and policy mounts, and healthy state. Existing TLS files are mounted only; no private-key content was read/copied/logged. scratch-server:static image ID remains byte-identical to the pre-load snapshot. Metadata receipt: logs/stage16-deployment-inspect.json.

The full live suite is running with the explicit capacity-probe omission. The mutant baseline passed full proofs/workspace tests; all 15 faults are queued in temporary copies. Runtime Rust remains frozen. No push, branch switch or protected-container mutation occurred.

## 2026-10-30 16:11 EDT — Stage 16 live acceptance passed

python3 scripts/stage16-live.py exited 0. ARM static image inspection found exactly the application binary in exported filesystem/image layers, no shell/key, no ELF interpreter or DT_NEEDED. Image 3,104,694 bytes; ELF 1,529,760 bytes. Hardened user/mounts/capabilities/no-new-privileges and healthcheck all pass.

8098 passes TLS1.3/ChaCha20/X25519/ECDSA P256 negotiation; TLS1.2/AES/P256 key exchange/h2/plain HTTP refusal; routes/curl/healthcheck; 3-second delayed 200, partial timeout 400, 17-second idle zero bytes; 48 malformed cases; 32 TLS fixture replays (8 fixtures x 4 fragmentation sizes); four curl browser replays; duplicates/header cap/privacy and 1,000-request/100-name log probe (59 new lines within the remaining 60-line budget). Capacity is explicitly false/not run: no 64-slot live probe was attempted.

All 18 scripted complete response buffers and stdout suffixes match proven-record on 8097 byte-for-byte. Comparative 35-byte trickle (one byte/TLS record every 1.5 seconds, only two slow live connections): 8098 gives **400 reason=slow at 10.058 s**, after 7 bytes; 8097 gives **200 at 51.023 s**, after all 35 bytes. Receipts: logs/stage16-live.log, logs/stage16-live/stage12-acceptance.json, rejection-equivalence.json and trickle-equivalence.json.

Protected snapshot comparison passed for scratch-server, nix-scratch, parser-http, parser-https, header-policy, proven-dispatch and proven-record: IDs/pids/start times unchanged. scratch-server:static image ID unchanged. Out-of-scope documents and PROVEN-IO.md unchanged from approved 401bbe2. No physical browser, separate LAN client or x86_64 check was performed. The only pending campaign is all 15 spec mutants; two killed so far.

TRUSTED-BASE.md audit refined per-method descriptions so pure accessors name rustls trait wiring as their reason, rather than attributing a libcrux call to a constant-returning body. Totals remain 63 functions / 618 SLoC. Unused secret extraction and fixed-string Debug formatters explicitly have no direct test coverage; this is a disclosed gap, not a claimed proof/test.

## 2026-10-30 16:31 EDT — Stage 16 final acceptance record

The full spec-mutant campaign exited 0: **15/15 killed, zero survivors, zero
invalid**. Every pre-intent refinement phase passes, including I/O and record
proofs. 13 focused intent lemmas kill faults; all fifteen actual workspace
test runs fail as required. Original source/fixture/config hash checks pass.
Temporary copies only; no working-tree mutation. Receipts:
logs/stage16-mutants.log, logs/stage16-mutants/results.json and per-mutant logs.
The sole runner adaptation is the previously documented in-memory output-path
redirect; no fault, proof, test command or kill criterion changed.

Final audit exited 0 (logs/stage16-final-audit.log): native, Nix ARM and mutant
baseline proofs **211/317/9, zero errors**; all three workspace baselines
**52 passed / zero failed**; all 44 inherited tests retained. Five new executable
I/O helpers total **52 proven runtime SLoC**; eleven ghost helpers have zero
runtime SLoC. Full origin/location/status/per-function counts, including test
and acceptance-tool functions, are in the stage-16 inventory above. Two new
shell helpers: elapsed_ns **4 SLoC**, read_result **11 SLoC**. Rewired wrappers
stay shell; crypto key-shape helpers stay proven using shared constants.

Whole-file main/transport/lib SLoC: **150→171 / 64→70 / 218→226**. Production
source excluding cfg(test): **150→149 / 64→70 / 135→141**. Function-body shell
subtotals in TRUSTED-BASE.md: main **136**, transport **49**, application **116**,
TLS **317** = **618 SLoC / 63 functions**. TLS production functions remain shell.
These are source inventories, not compiled-files coverage or percentages.

New equivalence: 100,000 deadline pairs with both deadline modes; all 44
ErrorKind values through 39 constructors and 4,098 errno cases plus Ok(0/1/1024),
plus Data(MAX); 201 exact admission comparisons plus MAX approved exception,
and 202 atomic admission/release cases; **12,214,248 port comparisons** plus
environment unset/non-UTF-8/default/error checks. No other old-code bug surfaced.
Plain/TLS trickle timings **10.003/10.003 s**, both 400 reason=slow; eight-second
requests **8.038/8.041 s**, both 200; after three idle seconds, response latency
**0.004/0.001 s**, both 200; zero-byte idle closures observed **15.002/14.943 s**
from post-handshake test start, checked again at full 17 seconds.

Nix check/image-static pass. Deployed healthy proven-io:static on 8098, image
sha256:5d1184971b5b66eafc1a76c8f6dad7abf5f5ad6bb6fd7572dac38331c238edab.
Image **3,104,694 bytes**, ELF **1,529,760 bytes**, static/shell-free/key-free,
non-root/read-only/no caps/no-new-privileges, read-only TLS and policy. Full
allowed live suite passes: protocol/suite/group/ALPN policy, browser fixtures,
48 malformed cases, privacy/log limiter, idle/slow, healthcheck and routes.
Eighteen full response/stdout comparisons are byte-identical to 8097. Live
trickle: **8098 400 reason=slow at 10.058 s** vs **8097 200 at 51.023 s**.
Only two comparative slow connections; no cap-filling attempt.

Final protected IDs/pids/start times and scratch-server:static image ID are
unchanged. Branch remains proven-io; no push, branch switch, protected-document
edit, formal-coverage run/edit, external memory write or secret logging. Rust
source is unchanged from the shell checkpoint 322f895. First-party scan across
22 Rust files: **zero forbidden proof/escape constructs**, six occurrences of
the required admit API identifier (listed separately, never the Verus primitive).

Unproven by design: Instant/OS timeout delivery, callbacks/Cell ownership,
atomics, threads, mutex/stdout/socket/filesystem effects, all rustls traits,
upstream TLS/crypto/DER, allocation/resource success, cleanup/secrecy/timing,
compiler/OS/hardware and upstream release/target proof correspondence. No sound
local models exist for those effects under the ban on opaque proof escapes.
All five requested pure properties are proven; no end-to-end TLS or wall-clock
liveness proof is claimed. Unused traffic-secret extraction and fixed-string
Debug formatters have no direct test; failure injection is not exhaustive.
Physical browsers, a separate LAN client, x86_64 and independent acceptance
were not run. Dudley performs coverage and independent acceptance separately.
The live capacity-fill check is deliberately omitted under Andrew's explicit
constraint, with capacity=false in the receipt; the cap is proven/unit-tested.

### Stage 17 — 2026-10-31: setup and measurement choice

Read RUSTLS-SURFACE.md and required stage context. Branch rustls-surface; initial
untracked codex-run-stage17.log belongs to the calling run and will not be committed.
No production access change is needed: tls::provider, tls::server_config and
serve_connection are public. Use native aarch64-apple-darwin release codegen for M1,
with debug=2 and strip=false only: the existing nix-build.sh invokes docker run,
which conflicts with the explicit no-container rule, and musl cross-linking is not
available in this host toolchain. This is the spec's permitted native fallback,
not a measurement of the Linux image. No containers or formal-coverage touched.
Memory diary writes are omitted because this stage explicitly forbids writes
outside the repository and named toolchain/temp exceptions.

### Stage 17 — 2026-10-31 06:46 EDT: M1 measured on the real musl binary (supersedes the native-fallback choice above)

The run continued under Claude after the Codex run stopped. Andrew clarified that ephemeral
`docker run --rm` build containers like nix-build.sh's are allowed (only long-lived
containers are forbidden; no existing container touched). So M1 measures the shipped
target instead of the native fallback: `verification/scripts/rustls-surface-build.sh` builds the flake's
own `server-static` (aarch64-unknown-linux-musl, release profile, same flake.lock toolchain
and flags) via `nix build --impure --expr` with `overrideAttrs` adding only
CARGO_PROFILE_RELEASE_DEBUG=2, CARGO_PROFILE_RELEASE_STRIP=false, dontStrip and
doCheck=false, and copies rustc's dep-info `.d` files out. flake.nix and nix-build.sh are
unchanged. Build took about 3 minutes from the existing Nix store volume.
Codegen check against the shipped binary (extracted from image-static.tar.gz): identical
.text size (706072 bytes) and address, identical .rodata size; not byte-identical, but
99.67% of instruction words match as a multiset once PC-relative immediates (b/bl/adr/adrp)
are masked, i.e. the same code with a different function layout. Recorded as the codegen
caveat for M1 item 3.
Counting rules (verification/scripts/rustls-surface.py): sources are the .crate archives from the cargo
cache, verified against the Cargo.lock checksums and unpacked to $TMPDIR; counter tokei
15.0.0 "code" lines. Crate total excludes test/bench-only files inside src/ (test.rs,
*_test.rs, test_macros.rs, *bench.rs); inline #[cfg(test)] modules inside other files
cannot be separated by tokei and remain in the total. Compiled = files in the crate's
dep-info. In-binary = distinct (file, line) pairs from DWARF 4 line-table sequences that
start in an executable section, with relative directories resolved against each CU's
DW_AT_comp_dir. Functions = defined t/T/w/W symbols in executable sections whose outermost
llvm-symbolizer frame lies in the crate.
Finding for item 5: the rustls client role is in the binary (751 in-binary lines) because
`--healthcheck` connects to our own endpoint with a rustls client. It is not on the accept
path and not peer-reachable from the network, so it is reported as "not unused" rather
than as dead code. No logging feature (no `log` crate compiled) and no TLS 1.2 files.
Attacker-facing module list (item 4) is a fixed file list in the script; extension codecs
live inside msgs/handshake.rs so they are counted with the handshake codecs.

### Stage 17 — 2026-10-31 07:00 EDT: fuzz harness, seeds, false OOM

`fuzz/` is a cargo-fuzz crate with its own `[workspace]` and its own Cargo.lock (started as
a copy of the server's, so rustls 0.23.46 / webpki 0.103.16 / pki-types 1.15.1 are the same
pins); the server workspace and Cargo.lock are untouched (cksum checked after every build).
Nightly toolchain 1.101.0-nightly (92b5b8c87 2026-10-29) and cargo-fuzz 0.13.2 live in
~/.rustup and ~/.cargo; homebrew cargo is first on PATH, so the nightly bin directory is put
first explicitly. All three targets load the production config through
`scratch_server::tls::load_config` from a throwaway P-256 key made by
scripts/make-lan-cert.py into $TMPDIR/stage17-tls (never in the repo). No production
visibility change was needed.
- `server_first_flight`: byte 0 seeds chunking (0 = one read), rest via read_tls.
- `server_after_handshake`: full in-memory handshake with a rustls client on our provider,
  then raw or sealed ops; sealed records use the client's extracted application traffic
  key/IV with our own ChaCha20-Poly1305 encrypter (the key stays in process memory; nothing
  logs it). Inner content type 0 lets the payload carry arbitrary padding. Limitation: after
  a KeyUpdate the harness keeps the old key, so later sealed records fail to decrypt.
  A `STAGE17_DEBUG` env var prints rustls error variants (no plaintext) for seed checks;
  the seeds give the expected errors (DecryptError after KeyUpdate, IllegalTlsInnerPlaintext,
  PeerSentOversizedRecord, ...) and app data is accepted.
- `proven_shell_pipeline`: `serve_connection` over `rustls::StreamOwned` and an in-memory
  socket, then the serve_tcp close_notify tail. serve_tcp itself takes a TcpStream for its
  deadline wrapper, so DeadlineSocket and the timeout paths are not exercised (choice logged).
  Raw mode feeds wire bytes; client mode embeds a rustls client in the socket so HTTP bytes
  reach the proven machine after a real handshake.
Seeds: scripts/stage17-capture-hellos.py ran the native release server on 127.0.0.1:18443
(binds 0.0.0.0 for under a minute) behind a recording relay on 18444 and captured the
ClientHello records of openssl s_client (1587 B), Homebrew curl/OpenSSL (1590 B), system
curl (319 B) and our rustls client (207 B); all four completed with HTTP 200. Only the
hello records are stored. `cargo run --example seed_corpus write` derives 8/22/18 seeds.
False OOM, not a finding: a 20 s trial reported an OOM at rss 301 MB after 2 s. Replaying
that input alone 1/3000/20000 times gives max RSS 48/215/441 MB with ASan defaults but 48 MB
after 20000 runs with ASAN_OPTIONS=quarantine_size_mb=0: ASan's 256 MB free-quarantine
counts toward RSS. The fuzz script sets quarantine_size_mb=16 (72-75 MB steady state for
all three targets), keeping -rss_limit_mb=256 and use-after-free detection.
Fuzz settings: -fork=2 per target, three targets concurrently (6 workers, within 8),
-max_total_time=1800, -timeout=1, -rss_limit_mb=256, -ignore_{crashes,timeouts,ooms}=0.
Inputs are not fully deterministic (server randomness and X25519/ECDSA nonces come from
getrandom through our provider); accepted, since the provider must not change.

### Stage 17 — 2026-10-31 07:35 EDT: fuzz results, coverage, acceptance

Runs 06:54–07:24 EDT, the three targets at once, 2 forked workers each (6 of the 8 allowed),
alongside the Bertie spike. All exited 0 after about 1815–1820 s of fuzzing, with
oom/timeout/crash 0/0/0 throughout and no artifacts, so there are no findings and
fuzz/findings/ was not created. Executions (last fork-mode status line) and on-disk corpus:
server_first_flight 29,848,149 / 2410 files; server_after_handshake 595,019 / 476;
proven_shell_pipeline 671,188 / 1780. The handshake per input costs throughput in the last two
(about 175–210 exec/s for two workers).
Coverage: verification/scripts/stage17-coverage.py builds coverage binaries with `cargo fuzz coverage`.
cargo-fuzz's batched replay logged fewer inputs than the corpus holds, so the script replays
every corpus and seed file in a single -runs=0 pass and merges with llvm-profdata. The totals
matched the batched result exactly. Reached = lcov DA lines with count>0, intersected with
the M1 in-binary (file, line) set by crate-relative path (same source, different codegen:
the coverage build is not LTO/opt-z, so "reached (any line)" exceeds the in-binary count).
Result: 1972/4152 rustls, 271/693 webpki, 164/250 pki-types. Two harnesses run a rustls
client, so the script also reports the figure outside client-role files (1639/3252) and for
server_first_flight alone (1252), which has no client. Client-role code inside shared
files (codecs used by both sides) cannot be separated by file.
Largest unreached in-binary rustls files (unreached/in-binary): enums.rs 448/453,
client/tls13.rs 257/402, msgs/handshake.rs 253/580, server/tls13.rs 224/351, client/ech.rs
149/149, client/hs.rs 118/238, error.rs 110/114, tls13/key_schedule.rs 84/239,
msgs/enums.rs 61/66, msgs/persist.rs 52/52. enums.rs and msgs/enums.rs are almost entirely
`enum_builder!` macro invocations: DWARF puts the expanded code on the invocation lines,
but llvm-cov attributes macro-expanded regions to the macro, so those ~510 lines read as
unreached whatever the fuzzer does. This is a measurement artifact, and the true reached
figure is higher. The remaining unreached lines were not analysed one by one; that is left
for Dudley's review and is not a finding.
Acceptance: verification/verify.sh 211/317/9 verified, 0 errors (logs/stage17-verify.log). cargo test
--workspace 52 passed (logs/stage17-cargo-test.log). Cargo.lock cksum unchanged after every
build. `git diff 9d6adbb` touches nothing under src/, crates/, config/, flake.nix,
flake.lock, Cargo.toml, Cargo.lock, nix-build.sh, verification/verify.sh, EVOLUTION.md or
ATTACK-SURFACE.md. No tracked file contains a private key; the throwaway key stays in
$TMPDIR/stage17-tls. The only containers started were the two ephemeral --rm measurement
builds, and no images were created or left behind. Not pushed.
