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

`./verify.sh` on 2026-10-27 (Stage 12): parser **56 verified/0 errors**,
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
- Rejection diagnostics (status and sanitized/truncated unknown header name only)
  are trusted transport code, covered by tests rather than a formal proof. Header
  values are never included in diagnostics.
- Socket acquisition, clock/deadline delivery, TLS framing, application route/body
  selection, response writes/flushes, healthcheck, kernel, Docker and hardware remain
  unverified. Stage 8 proves the byte/event core and formatter below.
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

`./nix-build.sh check` passed on aarch64-linux, including `./verify.sh` with the
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

`./verify.sh` passed with the nine-name specification: 18 verified, 0 errors.
All prior request/path/value/count limits and exact grammar/field equivalence
remain in force. No proof exemptions were introduced. Stage 4 Nix checks also
passed on aarch64-linux. Browser fixtures, diagnostic privacy, malformed inputs
and 100,000 differential cases are empirical tests, not additional proof claims.

## Stage 5 (2026-10-27)

The HTTP parser and nine-name grammar are unchanged. Native `./verify.sh` passed
18 verified, 0 errors. TLS decrypts/authenticates before calling the same parser;
the TLS adapter does not extend HTTP acceptance. Finite handshake/corpus tests
exercise trusted provider code, not new formal proofs. Crypto provenance and
exact limitations are documented in [TLS-PROVIDER.md](TLS-PROVIDER.md).

Stage 5 final aarch64-linux `./nix-build.sh check` also passed 18/0, Rust TLS
corpus tests and 100,000 differential cases. External OpenSSL interoperability
and negative protocol/suite/group/ALPN checks passed. These remain empirical
checks; they do not prove rustls or the cryptographic adapters.

## Stage 6 (2026-10-27)

`./verify.sh` passed 18 verified,0 errors against the twelve-name specification
listed above. Only Sec-Fetch-Dest, Sec-Fetch-Site and Sec-Fetch-Mode were added;
all other grammar, field equivalence, limits and safety claims remain unchanged.
No proof exemptions introduced. New TLS fixtures use captured header-name order
and synthetic values; whole/1/7/1024 writes and malformed zero-dispatch tests
are empirical checks. The independent twelve-name differential suite passed
100,000 cases,0 mismatches. TLS/provider trust limits in TLS-PROVIDER.md remain.

## Stage 8 (2026-10-27): proven transport

`./verify.sh` passes: parser **18 verified / 0 errors**, transport root
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

`./verify.sh` passed: parser **18 verified / 0 errors** against the thirteen-name
specification, transport root **43 / 0**, standalone AEAD **9 / 0** (overlapping
obligations as in stage 8). Only `Sec-Fetch-User` was added; all other grammar,
limits, transport core and TLS behavior are unchanged. No proof exemptions added.
Firefox fixtures retain captured header-name order with entirely synthetic values;
fragmentation and malformed zero-dispatch tests are empirical checks. The stage 8
trusted boundary remains unchanged.

## Stage 10 (2026-10-27): Chromium header refinement

`./verify.sh` passed: parser **21 verified / 0 errors**, transport root **46 / 0**,
standalone AEAD **9 / 0** (overlapping obligations). The parser specification now
matches exactly eighteen names ignoring ASCII case and permits at most 24 headers.
`ascii_lower`/`lower` and `matches_header`/`equal_header` prove folded prefix
comparison; request syntax still uses the unchanged exact matcher. The only new
names are Cache-Control, Sec-GPC, Sec-CH-UA, Sec-CH-UA-Mobile and Sec-CH-UA-Platform.
All request/path/value/framing limits remain unchanged. No proof exemptions added.
Transport core and production TLS code are unchanged. The stage 8 trusted boundary
still applies; browser fixtures/differential/runtime tests are empirical evidence.

## Stage 11 (2026-10-27): Pragma header refinement

`./verify.sh` passed: parser **21 verified / 0 errors**, transport root **46 / 0**,
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

`./verify.sh` passed: parser **211 verified / 0 errors**, transport root
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
