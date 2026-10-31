# Proof boundary

## HTTP subset and policy (Stage 12)

The byte grammar is `(GET|HEAD) SP path SP HTTP/1.(0|1) CRLF headers CRLF`.
Path starts with `/`, contains ASCII letters, digits, `/`, `-`, `_`, `.` and is
1–256 bytes. There is no query, fragment, decoding, body or pipeline. Request
size is at most 4096 bytes; headers are at most 24 lines. Each name is an RFC9110
ASCII token of 1–64 bytes, matched ignoring ASCII case; each line is exactly
`name ": " value CRLF`. Values are 1–256 printable ASCII bytes with no leading
or trailing space. HTTP/1.1 without Host is intentionally permitted. Connections
close after one request. Header values are never exposed to the handler.

The 19 quiet names remain Host, Accept, Connection, User-Agent, Accept-Language,
Accept-Encoding, Upgrade-Insecure-Requests, Priority, Referer, Sec-Fetch-Dest,
Sec-Fetch-Site, Sec-Fetch-Mode, Sec-Fetch-User, Cache-Control, Sec-GPC, Sec-CH-UA,
Sec-CH-UA-Mobile, Sec-CH-UA-Platform, Pragma. They are dropped without logging.

Hard-reject names are Content-Length, Transfer-Encoding, TE, Trailer, Expect,
Upgrade, HTTP2-Settings, Proxy-Connection, Keep-Alive. Guarded names are Cookie,
Authorization, Proxy-Authorization, Range, If-Range, If-Match, If-None-Match,
If-Modified-Since, If-Unmodified-Since, Forwarded, X-Forwarded-For,
X-Forwarded-Host, X-Forwarded-Proto, X-Real-IP, Via, Max-Forwards. Guarded names
default to reject; configuration can drop them individually. Other token names
follow unknown (default drop; strict reject). Duplicate Host or Connection is
always rejected; other duplicates follow policy.

## Proven by Verus

`./verification/verify.sh` on 2026-10-27 (Stage 12): parser **56 verified/0 errors**,
transport root **82/0**, standalone AEAD **9/0**. Groups overlap: the transport
root includes the parser and AEAD module. No bounded model checking. Cargo
compiles the same source after ghost erasure with the pinned Verus macros.

Policy and request model live in `crates/http-subset/src/lib.rs`:

- `Policy`, `parse_config`, `drop_header`, `hard_reject_is_fixed`,
  `config_cannot_loosen`: for every byte-slice input configuration fails or
  returns a valid policy. Every representable policy rejects classes 19..27,
  a stronger property than only configuration outputs. Exact ASCII-insensitive
  `classify` establishes those are the nine hard-reject names. Policy stores
  only unknown-drop, a 16-bit guarded-drop mask and a bounded log limit; there
  is no hard-reject override or Use permission to manufacture. `Request` stores
  only method and path ranges; neither header values nor their ranges exist in
  its type or returned specification. The code-fixed Use set is empty.
- `parse_policy`, `request_spec`, `check`, `accepted_safety`: for every input and
  valid policy, parse succeeds iff the selected whole-input grammar yields
  headers that all follow drop policy and have no duplicate Host/Connection.
  `accepted_safety` explicitly proves no accepted class is hard-reject and
  all guarded/unknown classes obey drop. Path/method equal the specification;
  total size, path bounds and 24-header/256-value limits remain enforced by the
  private raw grammar. Returned requests project only method/path.
- `strict_equivalence`, `stage11_names`, `stage11_header_names`: strict acceptance
  iff the unchanged `stage11_request_spec` accepts AND `no_duplicates` holds;
  accepted method/path are identical. Strict mode selects the retained private
  stage-11 parser then applies the same proven policy/duplicate filter. Other
  policies select the token grammar. Equivalence follows from the retained
  grammar and structural induction on its header list, rather than substituting
  a new definition for stage 11 or relying on a finite oracle test.
- Every executable function is checked for bounds, overflow, panic freedom and
  termination for arbitrary-length inputs. Config rejects before scanning over
 4096 bytes or 64 logical lines; all scans decrease remaining input, header
  recursion decreases a 24-step bound, and filtering loops decrease list length.
  Exact valid/invalid config-format behavior also has empirical tests; no full
  separate config-format equivalence theorem is claimed beyond the required
  policy-safety invariant. Allocation/resource failure remains trusted.
- `Machine::with_policy`, `Machine::step`: valid policy is carried unchanged by
  every event. The active-state invariant and Dispatch contract use the same
  policy-aware `request_spec`, preserving whole-buffer, single-dispatch,
  terminality, buffer/event bounds and other Stage 8 properties below.

These are properties of the chosen grammar and policy, not RFC-wide HTTP or TLS
correctness. The independent differential oracle additionally compares accepted
method/path and raw header fields with httparse; production requests expose only
method/path. The unchanged trust boundary below still applies.

## Trusted, not proven

- The written grammar, its translation into `request_spec`, Verus/Z3 correctness,
  vstd's library specifications, Rust/LLVM/code generation and macro ghost erasure.
  Fixed byte literals use Verus's byte-sequence model.
- Allocation success and execution resources. Out-of-memory aborts, OS failure,
  stack exhaustion outside the verified recursive depth, and timing side channels
  are outside the verifier's safety model.
- The Stage 14 application core below proves rejection diagnostics, sanitization,
  routing, response plans and the log limiter. Shell wiring to IO remains trusted.
  Header values are never included in diagnostics.
- Socket acquisition, clock/deadline delivery and millisecond conversion, TLS
  framing, filesystem-derived root body, atomic/lock/stdout effects, response
  writes/flushes, healthcheck, kernel, Docker and hardware remain unverified.
  Stages 8 and 14 prove byte/event, formatter and application decisions below.
- Stage 5 TLS protocol state machine, record layer, certificate validation and
  custom provider adapters remain trusted except for the stage 8 helpers below. See TLS-PROVIDER.md for upstream primitive
  claims and unconfirmed release/target correspondence. No end-to-end TLS proof.

## Tool provenance

Pinned Verus `0.2026.10.26.d17fbd0`, commit
`d17fbd0c58abf0d5e86825ea64b96aba72b1b31f`, requires Rust 1.99.1 according to
the downloaded release's `version.json` and executable diagnostic, both read.
Release assets and their SHA256 digests were read from the
[GitHub release API](https://api.github.com/repos/verus-lang/verus/releases/tags/release%2F0.2026.10.26.d17fbd0).
The [installation instructions](https://github.com/verus-lang/verus/blob/d17fbd0c58abf0d5e86825ea64b96aba72b1b31f/INSTALL.md)
require matching Rust. Macro/vstd crates are pinned by Cargo.lock; vstd's
[manifest](https://github.com/verus-lang/verus/blob/d17fbd0c58abf0d5e86825ea64b96aba72b1b31f/source/vstd/Cargo.toml)
confirms its erased-library purpose and versions.

Historical stage 3 M4 coverage audit: see [TLS-COVERAGE.md](TLS-COVERAGE.md). AWS's published results
cover particular cryptographic implementations/APIs/CPUs, with caveats. They do
not establish a fully covered TLS suite on this ARM deployment. TLS was absent in stage 3;
no AWS-LC proofs were run here and no claim about this binary's crypto is made.

## Build integration (2026-10-27)

`./nix-build.sh check` passed on aarch64-linux, including `./verification/verify.sh` with the
same pinned Verus release (18 verified, 0 errors), tests and cargo-deny. Nix's
Rust 1.99.0 builds the erased application; a separately pinned Rust 1.99.1 runs
Verus. `nix/verus.nix` uses official Rust distribution SHA256 files and release
API digests, with `VERUS_USE_RUSTUP=0` supported by the pinned Verus launcher
[source](https://github.com/verus-lang/verus/blob/d17fbd0c58abf0d5e86825ea64b96aba72b1b31f/source/verus/src/main.rs#L71).
ELF loader/RPATH patching adapts the prebuilt tools to Nix; the parser and proof
artifacts are not rewritten. No proof skip/fallback was needed on ARM Linux.
x86_64-linux tooling is configured but untested. Runtime image inspection and
LAN tests are empirical checks, not additional formal proofs. Offline advisory
checks use the pinned RustSec snapshot; registry yanks are untested. Future
cached Nix check results do not imply the advisory snapshot is still current.

## Stage 4 (2026-10-27)

`./verification/verify.sh` passed with the nine-name specification: 18 verified, 0 errors.
All prior request/path/value/count limits and exact grammar/field equivalence
remain in force. No proof exemptions were introduced. Stage 4 Nix checks also
passed on aarch64-linux. Browser fixtures, diagnostic privacy, malformed inputs
and 100,000 differential cases are empirical tests, not additional proof claims.

## Stage 5 (2026-10-27)

The HTTP parser and nine-name grammar are unchanged. Native `./verification/verify.sh` passed
18 verified, 0 errors. TLS decrypts/authenticates before calling the same parser;
the TLS adapter does not extend HTTP acceptance. Finite handshake/corpus tests
exercise trusted provider code, not new formal proofs. Crypto provenance and
exact limitations are documented in [TLS-PROVIDER.md](../../specs/development/TLS-PROVIDER.md).

Stage 5 final aarch64-linux `./nix-build.sh check` also passed 18/0, Rust TLS
corpus tests and 100,000 differential cases. External OpenSSL interoperability
and negative protocol/suite/group/ALPN checks passed. These remain empirical
checks; they do not prove rustls or the cryptographic adapters.

## Stage 6 (2026-10-27)

`./verification/verify.sh` passed 18 verified,0 errors against the twelve-name specification
listed above. Only Sec-Fetch-Dest, Sec-Fetch-Site and Sec-Fetch-Mode were added;
all other grammar, field equivalence, limits and safety claims remain unchanged.
No proof exemptions introduced. New TLS fixtures use captured header-name order
and synthetic values; whole/1/7/1024 writes and malformed zero-dispatch tests
are empirical checks. The independent twelve-name differential suite passed
100,000 cases,0 mismatches. TLS/provider trust limits in TLS-PROVIDER.md remain.

## Stage 8 (2026-10-27): proven transport

`./verification/verify.sh` passes: parser **18 verified / 0 errors**, transport root
**43 verified / 0 errors**, standalone AEAD helpers **9 verified / 0 errors**.
The transport invocation includes the unchanged parser and AEAD module; counts
repeat those obligations, rather than representing 70 distinct properties.
All proofs are unbounded. Cargo compiles the same files with ghost code erased.

- `crates/transport-core/src/lib.rs:20`, `Machine::valid`: buffer <=4097,
  step count <=4098; active states have <=4096 bytes and no complete parse.
  `new` establishes the invariant; `step` (line 37) preserves it on every event.
  Every executable core function is checked for overflow, bounds and termination.
- `Machine::step`: Dispatch carries exactly `request_spec` for the entire current
  buffer. Dispatch/Reject/CloseSilently set terminal; all subsequent events return
  Done with unchanged bytes/count. Thus a connection cannot dispatch twice.
- `Machine::step`: timeout/IdleExpired with zero bytes is CloseSilently; with any
  bytes it is Reject(Slow). Excess input is clipped to 4097 and Reject(Size).
  EOF is Reject(Grammar); empty Bytes is an EOF-like Grammar rejection. Remaining
  input is parsed before checking CRLFCRLF; it can only ReadMore after a nonempty
  append and failed parse. Bytes received after dispatch cannot be observed.
- `Machine::rank` strictly decreases on every active step; `event_bound` (line 195)
  and its recursive induction lemma cover arbitrary traces satisfying step's
  contract: by 4098 events the state is terminal. Interrupted syscalls are shell
  retries outside this event stream. This proves an event bound, not wall time or
  delivery of events by the OS. Every finite prefix shorter than the bound can
  of course end while active if no next event is supplied.
- `response` (line 162): exact output equals a fixed three-status line, fixed header
  labels, decimal body length and fixed CRLF separators, then precisely the body
  iff HEAD is false. Body bytes never influence headers. `decimal` (line 148)
  proves decimal evaluation equals the input length and every variable header
  digit is 48..57, excluding CR/LF. HEAD retains identical headers/Content-Length
  while omitting the body. Arbitrary body lengths are covered: None is returned
  only if headers plus body exceed usize::MAX; HEAD needs only the headers.
- `crates/transport-core/src/aead.rs:10`, `nonce`: exactly IV XOR the 96-bit
  big-endian encoding of the 64-bit sequence (four zero bytes followed by eight
  sequence bytes). Rejects IV lengths other than 12. No sequence-uniqueness claim.
- `aead.rs:31`, `sealed_len`: exact payload+1+16, with overflow rejected.
  `aead.rs:39`, `plaintext_len`: accepts iff ciphertext has >=17 bytes; exact
  plaintext size is ciphertext-16, nonzero and in bounds. Constants match the
  selected 32-byte key,12-byte IV and 16-byte tag. This does not verify libcrux.
- `aead.rs:49`, `release`: given equal-length slices, a reported authentication
  failure returns false and preserves the destination; success copies exactly
  plaintext and returns true. `src/tls.rs` passes the actual decrypt result's
  `is_ok()` to this gate, returns DecryptError on false, then truncates/unpads only
  on true. The gate is proven; the rustls adapter's wiring is reviewed and tested.

### Remaining unverified boundary

`src/lib.rs` is the IO/action shell: read-error mapping, callback/deadline switch,
route selection, handler counter, dynamic root body, formatting-error mapping, writes,
flushes and name-only diagnostics. `src/transport.rs` still owns sockets, rustls
stream setup, clock calculations,15s/2s enforcement and close_notify.
`src/main.rs` still owns accept, threads and 64-worker admission. The shell must
create Machine with new and call step only with its invariant preserved; production
does so and never mutates its exposed storage fields directly.

`src/tls.rs` trait implementations are not formally verified. No contracts for
rustls/libcrux calls are fabricated. AEAD payload gathering, allocation, key slice
conversion, rustls AAD construction, payload slicing/truncation/unpadding, trait
`encrypted_payload_len` (rustls-bounded len+17), X25519 checks, hash/HMAC/HKDF input
length assumptions, signing/key decoding and signature/DER checks remain trusted.
Full slice/length verification of all those adapters is not achieved: it would
require models/contracts for upstream APIs and conversions; it is not implied by
verified arithmetic or Rust's runtime bounds checks. Authentication correctness
and error reporting of the primitive remain an upstream dependency assumption.
TLS policy, parser grammar and parser specification are unchanged. No end-to-end
TLS, wall-clock liveness, secrecy/side-channel, allocation success, release/target
proof correspondence or OS/toolchain proof is claimed. Independent acceptance,
physical Safari, another LAN device, x86_64 and upstream F* reruns remain outstanding.

## Stage 9 (2026-10-27): Firefox header refinement

`./verification/verify.sh` passed: parser **18 verified / 0 errors** against the thirteen-name
specification, transport root **43 / 0**, standalone AEAD **9 / 0** (overlapping
obligations as in stage 8). Only `Sec-Fetch-User` was added; all other grammar,
limits, transport core and TLS behavior are unchanged. No proof exemptions added.
Firefox fixtures retain captured header-name order with entirely synthetic values;
fragmentation and malformed zero-dispatch tests are empirical checks. The stage 8
trusted boundary remains unchanged.

## Stage 10 (2026-10-27): Chromium header refinement

`./verification/verify.sh` passed: parser **21 verified / 0 errors**, transport root **46 / 0**,
standalone AEAD **9 / 0** (overlapping obligations). The parser specification now
matches exactly eighteen names ignoring ASCII case and permits at most 24 headers.
`ascii_lower`/`lower` and `matches_header`/`equal_header` prove folded prefix
comparison; request syntax still uses the unchanged exact matcher. The only new
names are Cache-Control, Sec-GPC, Sec-CH-UA, Sec-CH-UA-Mobile and Sec-CH-UA-Platform.
All request/path/value/framing limits remain unchanged. No proof exemptions added.
Transport core and production TLS code are unchanged. The stage 8 trusted boundary
still applies; browser fixtures/differential/runtime tests are empirical evidence.

## Stage 11 (2026-10-27): Pragma header refinement

`./verification/verify.sh` passed: parser **21 verified / 0 errors**, transport root **46 / 0**,
standalone AEAD **9 / 0** (overlapping obligations). Exactly `Pragma` was added,
accepted ignoring ASCII case and ignored by the application. The specification
now lists nineteen names. All other grammar, limits (including 24 headers),
transport core and production TLS code are unchanged. No proof exemptions added.
The stage 8 trusted boundary remains; fixture/differential/runtime checks are
empirical evidence, not additional formal proofs.

## Stage 12 (2026-10-27): header policy

The amended strict equivalence deliberately excludes duplicate Host/Connection.
The existing Stage 8 transport proofs now carry policy as invariant state; TLS,
AEAD helpers, response formatting and deadline rules are unchanged. Policy-file
IO/startup wiring, name-only rejection diagnostics, sliding-minute log budgeting
and handler wiring remain trusted and are tested; parser/config/decision/filter
safety and acceptance properties above are verified. Unknown logging uses shared
storage for at most 64 names and suppresses additional names when full, so its
line count is bounded by min(configured limit,64) in every sliding 60-second
window. No header values are logged. No end-to-end TLS proof is claimed.

## Stage 13 (2026-10-27): specification intent

`./verification/verify.sh` passed: parser **211 verified / 0 errors**, transport root
**237 / 0**, standalone AEAD **9 / 0**. Groups overlap as before. Mutation
results: **15/15 killed**, no survivors or malformed mutants; thirteen focused
intent-proof failures and two default-policy fixture/config kills. The ghost-only
`crates/http-subset/src/intent.rs` adds 42 intent statements with 112 supporting
construction/induction lemmas. [INTENT.md](INTENT.md) catalogues every lemma.
The five browser witnesses preserve fixture name bytes and order, reducing
values to one synthetic printable byte. Rejection witnesses quantify over every
valid policy; the 4097-byte theorem covers every input of that length. Universal
claims cover method/version/size, policy classes and duplicates, path bytes,
printable values/header count, and the first and final double-CRLF boundary.
Helpers perform structural induction over scans and header budgets; independent
witness modules permit parallel verification. There is no finite model checker,
new axiom or executable parser change. Opaque byte literals are exposed with the
pinned verifier's standard `reveal_byteslit` operation.

Exact shipped config bytes and traversal-shaped paths are tested in
`tests/intent.rs`, alongside inherited policy and TLS tests. Filesystem isolation
for traversal additionally rests on source review of exact route dispatch;
this is not a formal proof of filesystem behavior. [MUTANTS.md](MUTANTS.md)
records the deliberately incorrect spec/executable pairs. Each must preserve all
preexisting refinement proofs before an intent/test failure counts as a kill.
Intent proofs and these fifteen fault classes improve confidence in spec
adequacy; they do not establish that all possible specification mistakes are
excluded. The Stage 12 trusted boundary otherwise remains unchanged.

## First-party proof coverage

First-party security-relevant code: **644 of 1,363 SLoC proven (47.25%)**, measured by
[formal-coverage v1.4](https://github.com/gumshoe2020/formal-coverage) at `2fa5eb7`;
300 tested, 419 unverified. Counting rule: package source SLoC, with the security
boundary defined by formal-coverage's scratch-server config. Shipped third-party
runtime dependencies: **336,654 SLoC (not proven)**, for aarch64-unknown-linux-musl.
This is the stage-13 baseline measurement, not a new measurement of this release pass.


## Stage 14 (2026-10-30): proven dispatch

`./verification/verify.sh` passes **211 verified / 0 errors** parser, **285 / 0** transport,
**9 / 0** standalone AEAD. Roots overlap as before. The existing transport root
now includes `crates/transport-core/src/app.rs`, with 48 additional verification
results. Cargo erases ghost code from the same source. No existing contract or
proof was weakened, and no exemption was introduced. These results supersede
the historical application-shell boundary statements above.

All locations below refer to `crates/transport-core/src/app.rs`:

- `route:25`, `route_status:39`, `status_code:45`, `status_agrees:49`: exact Health
  iff `/health`, Root iff `/`, NotFound otherwise for arbitrary bytes. The shared
  route_status maps both successful routes to Ok and all other paths to NotFound.
  Numeric status codes correspond exactly to fixed wire status lines. The shell
  selects a route once; `unknown_line:531` and `task_response:77` both use this
  same route and route_status function, connecting logged code to sent status.
- `plan:57`: Dispatch carries one counter increment, all other actions zero, and
  Task preserves the request or rejection reason exactly. `task_response:77`
  can yield a 200/404 response only for Dispatch. Every Reject uses status 400,
  fixed body `bad request\n`, Content-Length 12 and head=false, regardless of
  diagnostics. ReadMore/CloseSilently/Done return None (zero bytes to write).
  Dispatch HEAD retains the same headers/body length and emits no body. Dynamic
  root content is a shell-supplied arbitrary slice. Its only formatter failure
  is an unrepresentable total non-HEAD length, as in Stage 8. The shell's atomic
  invocation and actual successful write remain IO assumptions, not Verus claims.
- `scan_line:164`, `scan_colon:187`, `next_header_name:248`: exact refinement of
  `line_end`, `colon_end`, `next_from` and `next_spec`. Yields bounded ordered
  spans after the first request-line CRLF and no later than `stop:206`, the first
  empty diagnostic line. Names contain neither colon nor any CRLF pair. Bare
  CR or LF remains permitted under Andrew's resolution A (0cb7016). Every next
  cursor strictly advances; every scan is in bounds and terminates. Rescanning
  from the request line enforces the empty-line boundary for arbitrary cursors.
- `sanitize:133`, `filtered:124`, `sanitize_spec:130`: exact stable ASCII
  alphanumeric-or-hyphen filter followed by take 64, output length <=64, every
  output byte in that alphabet. This excludes CR/LF, spaces, equals and controls
  from client-supplied log content even for malformed diagnostic names.
- `decision:291`, `diagnose_from:300`, `diagnose:317`: exact first-match diagnostic
  walk. More than 24 names wins before classification; hard class 19..27 wins
  next, followed by duplicate Host/Connection, non-drop guarded/unknown policy,
  and finally Grammar. Count, Host and Connection state are explicit in the
  independent recursive specification. All arbitrary input bytes and all policy
  values are covered. It is a diagnostic model, not a new acceptance grammar.
- `diagnostic_line:370`, `prefix:348`, `diagnostic_spec:365`: output equals one of
  six closed prefixes plus precisely sanitize(name) where a name is carried.
  Span validity is required and supplied by diagnose. Every output byte is
  printable ASCII and length <=128. `rejection_line:497` preserves the fixed
  Size/Slow strings and delegates Grammar to that diagnostic model.
- `lowercase:409`, `prune:437`, `UnknownLog::new:461`, `should_log:464`: lowercase
  storage, exact stable expiry filter, cap=min(limit,64) preserved for the
  immutable server limit, case-insensitive deduplication, and true iff there is
  room and no folded name is already retained. A true result appends precisely
  one (name, now) entry to the retained old sequence; false only prunes. Exact
  subtraction is guarded by now>=timestamp; rollback retains future entries.
  Production names are accepted ASCII tokens, where raw ASCII folding equals
  the old String conversion. The core also safely handles arbitrary byte names.
- `retain_recent:569`, `window_accounting:593`, `sliding_window_bound:611`: induction
  over arbitrary finite traces of should_log's transition contract. Entries
  created within any half-open 60,000 ms window cannot be pruned inside it;
  counting emitted entries therefore bounds emissions by min(limit,64). Applying
  the theorem to contiguous calls in each window of non-decreasing now_ms gives
  the bound in every sliding minute, not just aligned buckets. No bounded model
  checking or finite-only proof is used. Rollback stays safe but has no liveness
  claim. The shell's requested millisecond clock resolution is trusted.
- `next_unknown:510`, `unknown_line:531`, `needs_root:494`: pure accepted-request
  log filtering, fixed status/name-only formatting with sanitized name, and root
  filesystem-work selection. Unknown output also has printable ASCII and <=128
  bounds. Actual stdout/lock errors remain shell effects.

### Still unverified shell and trust boundary

`src/lib.rs`: ServerState construction and immutable policy/clock ownership,
Instant sampling/conversion, Mutex and AtomicUsize effects, stdout acquisition
and writes, filesystem existence checks and root String formatting, socket
read/error mapping and callback delivery, write_all/flush and error propagation.
Test-only adapters and preserved oracles are empirical checks, not proven code.
`src/main.rs`, `src/transport.rs` and `src/tls.rs` are unchanged and retain their
previous trust boundary (including the proven AEAD helpers). Allocation success,
OS scheduling, real-time deadlines, upstream TLS/crypto correctness, codegen and
hardware remain trusted. No end-to-end TLS, filesystem or wall-clock proof is
claimed. Dudley's independent acceptance and coverage measurement remain separate.

The workspace suite additionally checks 100,000 malformed/browser-derived blocks
(100,000 name sequences and 300,000 diagnostic comparisons across three policies),
256 single-byte and 10,000 random sanitizer names, 84,008 paired limiter calls,
rollback boundaries, plus GET/HEAD routing and handler counts. These are empirical
equivalence checks, not substitutes for the universal Verus contracts above.
No new formal-coverage percentage is claimed; Dudley measures stage 14 separately.

Stage 14 empirical acceptance also passes: aarch64-linux Nix check/static image,
39 workspace tests and 15/15 spec mutants (all refinement phases preserved).
The hardened 8096 deployment passes the inherited TLS/curl/browser/malformed/
idle/slow/capacity suite; fourteen rejection responses and stdout suffixes match
8095 byte-for-byte. Static/key-free/shell-free image inspection is empirical,
not a new Verus claim. See NOTES.md and MUTANTS.md for receipts and limitations.

## Stage 15: record decisions

The new `crates/transport-core/src/record.rs` is included under the existing
transport root, using the same ghost-erased source for Cargo. No upstream call
is modeled with a fabricated contract. `./verification/verify.sh` passes **211/0 parser,
298/0 transport, 9/0 standalone AEAD**; roots overlap. The record module adds
13 verification results and seven executable helpers (57 runtime source SLoC).

- `record.rs:13`, `unpad`: exact pre-size precedence, trailing-zero removal,
  missing-type and CCS rejection, post-size check. Success identifies the last
  nonzero byte, with type neither zero nor 20, len < input length and <=16,384;
  all later bytes are zero. NoContentType iff input length <=16,385 and every
  byte is zero (`all_zero:10`). Oversized iff initial length >16,385. The second
  size check is retained in rustls order even though the initial bound makes it
  unreachable. Arbitrarily long inputs are rejected before scanning.
- `record.rs:46`, `aad`: fixed 0x17/0x03/0x03 prefix and exact truncated high/low bytes
  for every usize; their big-endian decoding equals len below 65,536. The
  bit-vector proof covers both Verus usize widths.
- `record.rs:57`, `inner_plaintext`: Some iff inherited `aead::sealed_len` accepts,
  exact payload concatenated with one type byte, output length + TAG_LEN equals
  that sealed length. Uses sealed_len's existing contract, without duplicating
  its implementation or weakening it. `encrypted_payload_len:75` equals the
  same length when representable and otherwise saturates at usize::MAX, the
  single authorized behavior change.
- `record.rs:81`, `x25519_peer`: Some iff exactly 32 bytes, identical contents.
  `p256_point_shape:95`: exactly length 65 and prefix 4. Neither proves curve
  arithmetic or validates the cryptographic point.
- `record.rs:98`, `offers`: true iff an element equals wanted, for every slice
  and u16 wanted. Shell converts SignatureScheme to u16 and requests 0x0403.

All loops terminate with explicit decreasing lengths; bounds, overflow and
panic freedom are verified under the existing allocation-success model.

### TLS shell still unverified

`src/tls.rs` retains every trait impl and upstream call. In particular:
RecordCipher's OutboundChunks gathering, Zeroizing ownership, allocation, key
conversion, libcrux authentication/encryption result wiring, rustls payload
truncation, and `unpad_message`'s error/type/version/range conversions remain
shell. The pure AAD, unpadding, inner construction and encrypted-length decisions
are now proven; their trait wiring is tested, not proven. Exchange's random
secret, clamp/public derivation, libcrux ecdh and all-zero rejection remain shell;
only peer shape/copy is proven. P256Verify retains public-key parsing, DER codec
and libcrux verification; P256SigningKey retains enum conversion, allocation and
signer selection wiring around proven membership. All hash/HMAC/HKDF adapters,
OsRandom, private-key loading/DER parsing, signing, provider/config construction
and loading remain unverified. Rustls's handshake/state machine and protocol
framing, upstream crypto correctness/provenance, allocation, cleanup, compiler,
OS and hardware remain trusted. No end-to-end TLS proof is claimed.

Stage 15 empirical checks additionally pass: 43 workspace tests (39 inherited),
100,000 unpadding, 80,001 AAD, 10,000 inner/encrypted-record, 20,019 length,
25,856 each peer/point and 20,000 membership comparisons, with no oracle
differences. Nix aarch64-linux repeats 211/298/9 with zero errors and 43 tests.
Hardened 8097 live TLS, browser fixtures, delay/idle, image inspection and eighteen
response/stdout comparisons with 8096 pass. These are tests, not extra proofs.

The stage 15 spec-mutant campaign passes **15/15 killed**, no survivors or
invalid runs. All pre-intent refinement phases pass, including the new record
core; thirteen focused intent lemma kills and fifteen workspace-test kills.
See MUTANTS.md and logs/stage15-mutants/results.json for empirical receipts.

## Stage 16: startup and I/O decisions

`./verification/verify.sh` passes **211 parser / 317 transport / 9 standalone AEAD verified,
zero errors** (overlapping roots). The new `crates/transport-core/src/io.rs`
adds 19 verification results, including five executable helpers (52 runtime
function SLoC). Existing contracts/lemmas/tests are retained. Cargo executes the
same source after ghost erasure. All claims quantify over all representable
inputs, not a finite model checker.

Locations below refer to `crates/transport-core/src/io.rs`:

- `read_timeout:26`, `request_end:13`, `timeout_spec:17`: exact 15-second idle
  subtraction before the first byte; thereafter the smaller of two seconds and
  the remaining ten-second request budget, with saturating deadline addition.
  None iff expired; every returned duration is positive, idle elapsed+duration
  <=15s, started duration<=2s and elapsed+duration<=saturated deadline.
  `deadline_bound:51` proves every scheduled expiration is <=t+10s (integer
  arithmetic) after first byte t, or <=15s without one. Applying this universally
  to successive reads with the same first-byte time prevents the schedule from
  extending the request clock. OS delivery and physical elapsed time are trusted.
- `classify:62`: exact Data(0)/Data(n>0)/Interrupted/Timeout/Other mapping to
  Eof/Bytes/Retry/ReadTimeout/Fail. The returned starts flag is true iff Bytes;
  EOF, timeout and error never raise the flag. Handshake-only IO is kept outside
  classification by the trusted rustls shell boundary, not a modeled TLS proof.
  The shell alone owns sampling and the write-once Cell<Option<u64>>; it calls
  the callback only on this flag and never overwrites an existing timestamp.
- `admit:84`, `release:90`, `admission_result:78`, `release_result:81`,
  `admission_round_trip:95`: guarded addition iff active<64, result in 1..=64,
  rejection for all larger counts including MAX, saturating subtraction without
  underflow, and the composition of the two result specifications equals the
  initial count after any successful admission. The shell uses fetch_update
  for both effects. The approved old debug-overflow difference is recorded in
  PROVEN-IO.md; the historical reproducer remains a passing evidence test.
- `parse_port:123`, `port_start:101`, `digit_range:104`, `port_form:107`,
  `decimal_prefix:112`: None input defaults to 8080. Present input succeeds iff
  one optional leading '+' is followed by at least one ASCII digit, whose
  mathematical decimal value is the returned u16. Rejection proves there is no
  u16 with that form/value. Prefix induction proves an overflowing prefix cannot
  become a valid bounded value later; leading zero runs of arbitrary length are
  accepted. Every loop terminates with bounds/overflow checked. Shell env::var
  maps unset and non-UTF-8 to the default; no UTF-8 claim is fabricated in core.
- `sizes_agree:180`: HASH_LEN equals aead::KEY_LEN (32), X25519_LEN=32 and
  P256_POINT_LEN=65. record.rs:82 uses X25519_LEN in its return array, length
  checks, loop bounds and exact-copy contract; record.rs:96 uses P256_POINT_LEN
  for the shape predicate. TLS output_len/hash_len/key_len return core constants
  and retain their unverified trait-method status.

The complete remaining handwritten first-party production shell is catalogued
function by function in TRUSTED-BASE.md: **63 functions / 618 SLoC**, with reasons,
finite test coverage and explicit gaps. No contracts for clocks, syscalls,
threads, atomics, rustls/libcrux calls, allocation or secret cleanup are invented.
Only the approved request deadline and admission-overflow fix change behavior.
No end-to-end TLS, secrecy/timing, syscall precision or wall-clock liveness proof
is claimed. Independent acceptance and compiled-files coverage are Dudley's.

Stage 16 empirical acceptance passes: 52 workspace tests on native and Nix ARM,
15/15 spec mutants (all refinement gates preserved), static/key-free/shell-free
image, hardened 8098 live suite, 18 byte-identical response/stdout comparisons
with 8097, and the comparative trickle test (400/slow at 10.058 s vs old 200 at
51.023 s). These are finite checks, not additional proofs. The live 64-slot
capacity fill is explicitly omitted under Andrew's prohibition; core admission
proofs and local atomic tests cover the cap. No physical-browser, separate-LAN
or x86_64 validation is claimed.
