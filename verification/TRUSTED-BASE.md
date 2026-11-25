# Trusted base at stage 19 E (2026-11-21)

Inventory of every handwritten first-party **production** function outside the Verus roots. Locations refer to this branch. SLoC counts physical nonblank/noncomment signatures and bodies, including braces, excluding imports/types and test-only code. Closures are counted within their enclosing function. Derived implementations and dependency functions belong to the macro/dependency trust boundary below. Calling a proven core function does not prove its shell caller. This is not a formal-coverage measurement.

Coverage means the listed finite test exercises the path, sometimes indirectly. It does not establish complete branch/error coverage. Explicit gaps below are not claimed tested. Tooling/test functions are unproven empirical evidence, outside the shipped runtime subtotal.

## src/main.rs

| Function | File:line | SLoC | Role | Why unproven | Test/live coverage |
| --- | --- | ---: | --- | --- | --- |
| `port#1` | src/main.rs:15 | 5 | Read environment and translate parser failure | environment syscall and error allocation | stage16_port_unset_non_utf8_and_error; stage16 port oracle |
| `main#1` | src/main.rs:21 | 12 | Select healthcheck or startup; print startup errors and exit | environment/process and stderr | live executable --healthcheck; normal startup; failure exits not exhaustively injected |
| `run#1` | src/main.rs:34 | 79 | Load policy/TLS, bind/listen/accept, update counters and spawn workers | filesystem, sockets, clock, atomic and threads; startup-only stdout | stage16 live startup/policy/requests; stage16_atomic_admission_and_release (decision wiring); thread failures not injected |
| `healthcheck#1` | src/main.rs:116 | 10 | Exit 0/1 for loopback ServerHello readiness | environment and process status wiring | healthcheck unit suite; executable/container check recorded in stage results |

**Subtotal: 4 functions, 106 SLoC.**

## src/transport.rs

| Function | File:line | SLoC | Role | Why unproven | Test/live coverage |
| --- | --- | ---: | --- | --- | --- |
| `read#1` | src/transport.rs:23 | 8 | Sample clock, set core timeout and read socket | Instant and socket syscalls | stage16 deadline oracle and plain/TLS deadline tests; inherited handshake budget test |
| `write#1` | src/transport.rs:34 | 3 | Forward socket write | socket syscall | live TLS responses and exact response comparison |
| `flush#1` | src/transport.rs:37 | 3 | Forward socket flush | socket syscall | live TLS responses and exact response comparison |
| `serve_tcp#1` | src/transport.rs:44 | 38 | Configure socket, initialize fixed first-byte clock, wire TLS/close_notify/shutdown | socket, Cell/Rc, Instant and Bertie wiring | new plain/TLS deadline tests; inherited idle/handshake tests; stage16 live suite/trickle |
| `elapsed_ns#1` | src/transport.rs:84 | 8 | Saturate elapsed Instant nanoseconds to u64 | clock syscall and std Duration conversion | new plain/TLS deadline tests; arithmetic limits tested by core deadline oracle |

**Subtotal: 5 functions, 60 SLoC.**

## src/lib.rs

| Function | File:line | SLoC | Role | Why unproven | Test/live coverage |
| --- | --- | ---: | --- | --- | --- |
| `default#1` | src/lib.rs:25 | 3 | Construct builtin ServerState | atomic, mutex and clock construction | workspace default-policy/browser tests; new deadline tests |
| `with_policy#1` | src/lib.rs:30 | 8 | Construct immutable policy and shared log/counter/clock state | atomic, mutex and clock construction | workspace policy and limiter tests; live header-policy suite |
| `log_unknown#1` | src/lib.rs:39 | 17 | Lock limiter, sample clock, print proven name-only line | mutex, clock and stdout syscall | stage14 oracle/limiter tests; live 1000-request log budget/privacy probe |
| `reply#1` | src/lib.rs:56 | 6 | Map allocation-length failure; write and flush response | socket IO and error conversion | dispatch routing tests; live exact response comparison; write failures not exhaustively injected |
| `root_body#1` | src/lib.rs:62 | 16 | Inspect local file existence and format root response | filesystem syscalls and runtime String formatting | dispatch routing tests; live root body and response equivalence |
| `serve_connection#1` | src/lib.rs:80 | 3 | Call request shell with empty callback | Read/Write traits are opaque IO | existing plain dispatch/corpus/fragmentation tests |
| `serve_with_started#1` | src/lib.rs:83 | 52 | Read, deliver classified events, execute proven plans and effects | IO traits, callback, stdout, atomics and filesystem | all dispatch/corpus/TLS tests; new request deadline tests; live comparison |
| `read_result#1` | src/lib.rs:137 | 11 | Translate IO result to the proven classification enum | std io::ErrorKind adapter outside Verus model | stage16_classification_all_error_kinds (all 44 kinds) |

**Subtotal: 8 functions, 116 SLoC.**

## src/tls.rs

Every function below is new production TLS glue. The old provider and its 46-function
production inventory are removed; their oracle code exists only under `cfg(test)`.

| Function | File:line | SLoC | Role | Why unproven | Test/live coverage |
| --- | --- | ---: | --- | --- | --- |
| `load_config#1` | src/tls.rs:18 | 9 | Read PEM key/certificate once and decode P-256 scalar | filesystem, PEM and p256 decoding | port tests use from_der; native production-binary startup/10,000 handshakes; malformed PEM branches not exhaustive |
| `load_certificate#1` | src/tls.rs:28 | 8 | Decode mounted certificate PEM to DER | filesystem and PEM decoding | native PEM startup/10,000-handshake detector |
| `invalid#1` | src/tls.rs:37 | 3 | Return a fixed sanitized TLS error | std error conversion outside proof roots | alert/corrupt-record tests; peer and handshake errors |
| `algorithms#1` | src/tls.rs:41 | 16 | Map proven group to fixed Bertie algorithms | Bertie enum/API wiring | X25519 TLS tests; E live interop passes all three groups |
| `read_record#1` | src/tls.rs:58 | 16 | Read bounded TLS record or boundary EOF | socket traits, allocation and framing outside hello proof | fragmented application tests; incomplete handshake deadline tests; corrupt-record test |
| `accept#1` | src/tls.rs:82 | 42 | Inspect hello and drive DB/CCS/handshake/Finished with OS RNG | Bertie, randomness, secrets and I/O wiring | TLS suite, unchanged canonical-signature regression and strict native 10,000-handshake detector pass |
| `close_notify#1` | src/tls.rs:126 | 8 | Consume state and send authenticated alert | Bertie encryption and I/O | successful TLS peers require authenticated EOF; E live unwrap passes for 200/404/400 |
| `read#1` | src/tls.rs:136 | 24 | Decrypt one bounded record and deliver buffered HTTP bytes | Bertie decryption, allocation and Read wiring | HTTP/corpus/fragmentation/deadline and corrupt-record/alert tests; received close_notify unsupported |
| `write#1` | src/tls.rs:162 | 13 | Encrypt bounded response fragments | Bertie encryption and Write wiring | route/HTTP response assertions; no exhaustive socket error injection |
| `flush#1` | src/tls.rs:175 | 3 | Forward flush to deadline socket | opaque I/O trait | TLS response tests |
| `healthcheck#1` | src/tls.rs:200 | 27 | Send constant hello, check bounded first ServerHello record and close | socket, clock and unauthenticated readiness framing | real Bertie server passes; closed port, non-TLS, alert, bad version/length/type, truncation and timeout fail |
| `healthcheck_read#1` | src/tls.rs:229 | 18 | Read exact bounded slice within one absolute deadline | socket timeout precision and clock outside Verus | partial-byte trickle cannot renew the two-second budget |

## Subtotals

| Group | Functions | SLoC |
| --- | ---: | ---: |
| src/main.rs | 4 | 106 |
| src/transport.rs | 5 | 60 |
| src/lib.rs | 8 | 116 |
| src/tls.rs | 12 | 187 |
| **Total unproven production shell** | **29** | **469** |

## Remaining assumptions and gaps

All executable functions in crates/http-subset and crates/transport-core are checked by the existing Verus roots; ghost specifications express the intended subset, rather than RFC-wide HTTP or full TLS. The new io.rs rules prove scheduled timeout bounds, not real-time scheduling, syscall timeout precision or prompt worker release. Callback delivery, first-byte ownership, atomic operations, thread spawning, locks, socket writes/errors and clocks remain shell responsibilities. The E live matrix checks the 64-slot cap; core proofs and the atomic admission/release test cover counts through 200 and MAX without opening sockets.

Verus/Z3, vstd contracts, ghost erasure, Rust/LLVM, dependency code and derived traits remain trusted. Allocation success, cleanup/zeroization effects, stack/resources, OS/Docker/hardware, upstream Bertie handshake and protocol correctness, primitive crypto correctness/release-target correspondence, entropy and secrecy/timing side channels are not proven here. Healthcheck, root_body, startup summary, PEM/DER loading and all TLS adapter implementations remain shell by explicit scope. No end-to-end TLS, wall-clock liveness, resource-success or independent acceptance claim is made.

Test-only source: src/io_tests.rs, src/io_deadline_tests.rs (included in tls::test_provider::tests), src/tls_test_provider.rs, src/dispatch_oracle.rs, src/record_tests.rs, cfg(test) blocks and tests/*.rs. Acceptance/build tools: scripts/*.py, verification/verify.sh, nix-build.sh, flake.nix and nix/*.nix. These are not shipped server functions. The new deadline test opens only two local slow connections concurrently; inherited tests are unchanged.


Stage 19 retains `record.rs` and `aead.rs` unchanged and verified, but **production no longer
calls them**; Bertie owns its record layer. These local proofs must not be attributed to
Bertie's encryption. `hello.rs` and `spec/hello.rs` are in the transport proof root and a
standalone hello root; no new external-body function is trusted there. Historical provider
oracles exercise these retained helpers only under cfg(test).

At Bertie e6ea574, seven fresh strict F* checks (DER, crypto interface, formats,
API, handshake and both key schedulers) discharge all VCs, with unchanged model
libraries and no new proof patches or assume/admit/lax directives. The original
abstract crypto models remain trusted. This is not a replay of every published
SSProve/ProVerif security claim or a proof of backend correspondence/MLKEM security.
Bertie's TRUSTED.md lists the three rand compatibility forwarding methods and two
KEM wrappers excluded from extraction; Algorithms::validate is trusted rejection
glue. Details and limits: [dependency report](../dev/BERTIE-DEPS-REPORT.md).
The unchanged canonical-signature regression and 10,000 strict native handshakes
pass. E live checks verify certificates, HTTP equivalence and authenticated close;
independent acceptance remains outstanding. This shell inventory adds one exact-
read helper and replaces the old authenticated healthcheck.

E2 changes the health contract by Andrew’s decision: this is readiness only. The
fixed 208-byte ClientHello offers ChaCha20/SHA256, ECDSA P-256 and all three
supported groups, with public X25519/P-256 basepoint shares. No MLKEM share is
needed to select X25519. It uses no TLS client library, randomness or certificate
loading. A forged loopback ServerHello can satisfy it; it does not authenticate
the peer, finish TLS, or exercise HTTP. Certificate-verified external interop and
HTTP comparison are separate release checks. Reads are bounded to 16,384 bytes;
connect/write/reads share a two-second absolute budget and shutdown closes both
directions. The constant data is excluded from the function SLoC inventory.
