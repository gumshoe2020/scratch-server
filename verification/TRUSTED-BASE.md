# Trusted base after stage 16

Inventory of every handwritten first-party **production** function outside the Verus roots. Locations refer to this branch. SLoC counts physical nonblank/noncomment signatures and bodies, including braces, excluding imports/types and test-only code. Closures are counted within their enclosing function. Derived implementations and dependency functions belong to the macro/dependency trust boundary below. Calling a proven core function does not prove its shell caller. This is not a formal-coverage measurement.

Coverage means the listed finite test exercises the path, sometimes indirectly. It does not establish complete branch/error coverage. Explicit gaps below are not claimed tested. Tooling/test functions are unproven empirical evidence, outside the shipped runtime subtotal.

## src/main.rs

| Function | File:line | SLoC | Role | Why unproven | Test/live coverage |
| --- | --- | ---: | --- | --- | --- |
| `port#1` | src/main.rs:16 | 5 | Read environment and translate parser failure | environment syscall and error allocation | stage16_port_unset_non_utf8_and_error; stage16 port oracle |
| `main#1` | src/main.rs:22 | 12 | Select healthcheck or startup; print startup errors and exit | environment/process and stderr | live executable --healthcheck; normal startup; failure exits not exhaustively injected |
| `run#1` | src/main.rs:35 | 79 | Load policy/TLS, bind/listen/accept, update counters and spawn workers | filesystem, sockets, clock, atomic and threads; startup-only stdout | stage16 live startup/policy/requests; stage16_atomic_admission_and_release (decision wiring); thread failures not injected |
| `healthcheck#1` | src/main.rs:117 | 40 | Local authenticated TLS client and health response check | socket/filesystem IO and rustls client wiring | live docker exec --healthcheck |

**Subtotal: 4 functions, 136 SLoC.**

## src/transport.rs

| Function | File:line | SLoC | Role | Why unproven | Test/live coverage |
| --- | --- | ---: | --- | --- | --- |
| `read#1` | src/transport.rs:23 | 8 | Sample clock, set core timeout and read socket | Instant and socket syscalls | stage16 deadline oracle and plain/TLS deadline tests; inherited handshake budget test |
| `write#1` | src/transport.rs:34 | 3 | Forward socket write | socket syscall | live TLS responses and exact response comparison |
| `flush#1` | src/transport.rs:37 | 3 | Forward socket flush | socket syscall | live TLS responses and exact response comparison |
| `serve_tcp#1` | src/transport.rs:44 | 31 | Configure socket, initialize fixed first-byte clock, wire TLS/close_notify | socket, Cell/Rc, Instant and rustls wiring | new plain/TLS deadline tests; inherited idle/handshake tests; stage16 live suite/trickle |
| `elapsed_ns#1` | src/transport.rs:77 | 4 | Saturate elapsed Instant nanoseconds to u64 | clock syscall and std Duration conversion | new plain/TLS deadline tests; arithmetic limits tested by core deadline oracle |

**Subtotal: 5 functions, 49 SLoC.**

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

| Function | File:line | SLoC | Role | Why unproven | Test/live coverage |
| --- | --- | ---: | --- | --- | --- |
| `provider#1` | src/tls.rs:21 | 12 | Register crypto adapters and allowed suite/group | rustls trait wiring | workspace and live TLS policy negotiation/rejection tests |
| `fill#1` | src/tls.rs:48 | 3 | Fill a byte slice using OS randomness and map failure | getrandom syscall and RNG trait wiring | TLS key generation/handshake/signing tests; entropy failure not injected; not every RNG entry point directly exercised |
| `try_next_u32#1` | src/tls.rs:54 | 5 | Fill four random bytes and decode little-endian u32 | getrandom syscall and RNG trait wiring | TLS key generation/handshake/signing tests; entropy failure not injected; not every RNG entry point directly exercised |
| `try_next_u64#1` | src/tls.rs:59 | 5 | Fill eight random bytes and decode little-endian u64 | getrandom syscall and RNG trait wiring | TLS key generation/handshake/signing tests; entropy failure not injected; not every RNG entry point directly exercised |
| `try_fill_bytes#1` | src/tls.rs:64 | 3 | Fill caller bytes through the RNG trait | getrandom syscall and RNG trait wiring | TLS key generation/handshake/signing tests; entropy failure not injected; not every RNG entry point directly exercised |
| `start#1` | src/tls.rs:74 | 3 | Allocate a new libcrux SHA256 context | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `hash#1` | src/tls.rs:77 | 3 | Compute libcrux SHA256 and package the digest | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `output_len#1` | src/tls.rs:80 | 3 | Return the shared HASH_LEN constant | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `algorithm#1` | src/tls.rs:83 | 3 | Return the fixed SHA256 algorithm identifier | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `fork_finish#1` | src/tls.rs:88 | 5 | Finalize the current hash snapshot into rustls output | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `fork#1` | src/tls.rs:93 | 3 | Clone and allocate the hash context | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `finish#1` | src/tls.rs:96 | 3 | Finalize a boxed hash context via fork_finish | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `update#1` | src/tls.rs:99 | 3 | Update the upstream hash state | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `extract_from_zero_ikm#1` | src/tls.rs:107 | 3 | Delegate HKDF extraction using a zero input key | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `extract_from_secret#1` | src/tls.rs:110 | 11 | Allocate a protected PRK and call libcrux HKDF extract | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `expander_for_okm#1` | src/tls.rs:122 | 3 | Copy an output key block into a protected expander | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `hmac_sign#1` | src/tls.rs:125 | 5 | Compute HMAC-SHA256 and package the authentication tag | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `expand_slice#1` | src/tls.rs:132 | 9 | Concatenate info, expand with libcrux HKDF, map length errors | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `expand_block#1` | src/tls.rs:141 | 6 | Allocate and expand a hash-sized output block | libcrux calls, allocation/secret ownership and rustls trait wiring | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `hash_len#1` | src/tls.rs:147 | 3 | Return the shared HASH_LEN constant | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | hkdf_rfc5869_case1_and_hash_fork; all TLS handshakes (indirect for trait accessors) |
| `start#2` | src/tls.rs:159 | 8 | Generate/clamp an X25519 secret and derive its public key | libcrux call, RNG/secret ownership and rustls trait wiring | TLS handshake/group-rejection tests; aead_rejects_tampered_record_and_x25519_rejects_low_order; stage15 key-shape oracle |
| `name#1` | src/tls.rs:167 | 3 | Return the fixed X25519 named group | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | TLS handshake/group-rejection tests; aead_rejects_tampered_record_and_x25519_rejects_low_order; stage15 key-shape oracle |
| `complete#1` | src/tls.rs:172 | 7 | Validate/copy peer shape through core, perform ECDH and reject invalid shares | libcrux call, RNG/secret ownership and rustls trait wiring | TLS handshake/group-rejection tests; aead_rejects_tampered_record_and_x25519_rejects_low_order; stage15 key-shape oracle |
| `pub_key#1` | src/tls.rs:179 | 3 | Borrow the generated public-key bytes | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | TLS handshake/group-rejection tests; aead_rejects_tampered_record_and_x25519_rejects_low_order; stage15 key-shape oracle |
| `group#1` | src/tls.rs:182 | 3 | Return the fixed X25519 group | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | TLS handshake/group-rejection tests; aead_rejects_tampered_record_and_x25519_rejects_low_order; stage15 key-shape oracle |
| `encrypter#1` | src/tls.rs:193 | 3 | Allocate an outbound RecordCipher retaining key and IV | rustls trait/message/error wiring, allocation and protected ownership (core decisions are separately proven) | stage15 record oracles; tampered-record test; workspace/live TLS round trips |
| `decrypter#1` | src/tls.rs:196 | 3 | Allocate an inbound RecordCipher retaining key and IV | rustls trait/message/error wiring, allocation and protected ownership (core decisions are separately proven) | stage15 record oracles; tampered-record test; workspace/live TLS round trips |
| `key_len#1` | src/tls.rs:199 | 3 | Return the core AEAD KEY_LEN constant | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | stage15 record oracles; tampered-record test; workspace/live TLS round trips |
| `extract_keys#1` | src/tls.rs:202 | 7 | Package traffic secret types (unused feature) | rustls trait wiring and secret ownership | not directly exercised: secret extraction is disabled/not requested by this server; normal AEAD key wiring tested by record/handshake tests |
| `encrypt#1` | src/tls.rs:211 | 33 | Gather payload, construct proven inner plaintext/AAD/nonce, encrypt and package record | libcrux call, rustls message/error wiring, allocation and Zeroizing | stage15 record oracles; tampered-record test; workspace/live TLS round trips |
| `encrypted_payload_len#1` | src/tls.rs:244 | 3 | Delegate encrypted length to the proven saturating helper | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | stage15 record oracles; tampered-record test; workspace/live TLS round trips |
| `decrypt#1` | src/tls.rs:249 | 26 | Copy/decrypt ciphertext, gate authenticated release and unpad via core | libcrux call, rustls message/error wiring, allocation and Zeroizing | stage15 record oracles; tampered-record test; workspace/live TLS round trips |
| `unpad_message#1` | src/tls.rs:279 | 10 | Map core unpadding errors and construct the rustls type/version/payload range | rustls trait/message/error wiring, allocation and protected ownership (core decisions are separately proven) | stage15 record oracles; tampered-record test; workspace/live TLS round trips |
| `public_key_alg_id#1` | src/tls.rs:293 | 3 | Return the fixed P256 public-key algorithm identifier | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | workspace certificate generation/TLS handshakes; stage15 scheme/point-shape oracle; live ECDSA negotiation (indirect for metadata accessors) |
| `signature_alg_id#1` | src/tls.rs:296 | 3 | Return the fixed ECDSA/SHA256 signature identifier | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | workspace certificate generation/TLS handshakes; stage15 scheme/point-shape oracle; live ECDSA negotiation (indirect for metadata accessors) |
| `verify_signature#1` | src/tls.rs:299 | 21 | Validate point shape, decode public key/signature and verify ECDSA | libcrux call, DER/error conversion and rustls trait wiring | workspace certificate generation/TLS handshakes; stage15 scheme/point-shape oracle; live ECDSA negotiation (indirect for metadata accessors) |
| `fmt#1` | src/tls.rs:330 | 3 | Print fixed type name without key material | fmt trait wiring | not directly exercised; fixed-string source review only; no secret-bearing Debug format is emitted |
| `load_private_key#1` | src/tls.rs:335 | 31 | Decode PKCS8/SEC1, validate curve/scalar, derive public key | DER crates, libcrux call, Zeroizing and rustls trait wiring | workspace TLS configs/handshakes and live mounted-key startup; malformed-key branches not exhaustive |
| `fmt#2` | src/tls.rs:369 | 3 | Print fixed type name without key material | fmt trait wiring | not directly exercised; fixed-string source review only; no secret-bearing Debug format is emitted |
| `choose_scheme#1` | src/tls.rs:374 | 5 | Convert scheme enums, consult proven membership and allocate a signer | rustls trait/message/error wiring, allocation and protected ownership (core decisions are separately proven) | workspace certificate generation/TLS handshakes; stage15 scheme/point-shape oracle; live ECDSA negotiation (indirect for metadata accessors) |
| `algorithm#2` | src/tls.rs:379 | 3 | Return the fixed ECDSA signing algorithm | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | workspace certificate generation/TLS handshakes; stage15 scheme/point-shape oracle; live ECDSA negotiation (indirect for metadata accessors) |
| `public_key#1` | src/tls.rs:382 | 3 | Clone the stored public SPKI into the trait return type | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | workspace certificate generation/TLS handshakes; stage15 scheme/point-shape oracle; live ECDSA negotiation (indirect for metadata accessors) |
| `scheme#1` | src/tls.rs:387 | 3 | Return the fixed ECDSA_NISTP256_SHA256 scheme | rustls trait wiring (explicit scope exclusion; no trait model in the Verus roots) | workspace certificate generation/TLS handshakes; stage15 scheme/point-shape oracle; live ECDSA negotiation (indirect for metadata accessors) |
| `sign#1` | src/tls.rs:390 | 13 | Decode protected scalar, produce randomized ECDSA signature and encode DER | libcrux call, DER/error conversion and rustls trait wiring | workspace certificate generation/TLS handshakes; stage15 scheme/point-shape oracle; live ECDSA negotiation (indirect for metadata accessors) |
| `load_config#1` | src/tls.rs:405 | 8 | Read PEM certificate/key and build TLS configuration | filesystem syscalls and PEM/rustls wiring | live startup and authenticated healthcheck; malformed file failures not exhaustively injected |
| `server_config#1` | src/tls.rs:414 | 12 | Build TLS1.3-only provider, ALPN and resumption settings | rustls trait/config wiring | TLS positive/negative workspace tests and live protocol/suite/group/ALPN probes |

**Subtotal: 46 functions, 317 SLoC.**

## Subtotals

| Group | Functions | SLoC |
| --- | ---: | ---: |
| src/main.rs | 4 | 136 |
| src/transport.rs | 5 | 49 |
| src/lib.rs | 8 | 116 |
| src/tls.rs | 46 | 317 |
| **Total unproven production shell** | **63** | **618** |

## Remaining assumptions and gaps

All executable functions in crates/http-subset and crates/transport-core are checked by the existing Verus roots; ghost specifications express the intended subset, rather than RFC-wide HTTP or full TLS. The new io.rs rules prove scheduled timeout bounds, not real-time scheduling, syscall timeout precision or prompt worker release. Callback delivery, first-byte ownership, atomic operations, thread spawning, locks, socket writes/errors and clocks remain shell responsibilities. Tests do not fill the 64-slot cap: Andrew explicitly prohibits that live probe. Core proofs and the atomic admission/release test cover counts through 200 and MAX without opening sockets.

Verus/Z3, vstd contracts, ghost erasure, Rust/LLVM, dependency code and derived traits remain trusted. Allocation success, cleanup/zeroization effects, stack/resources, OS/Docker/hardware, upstream rustls handshake and protocol correctness, primitive crypto correctness/release-target correspondence, entropy and secrecy/timing side channels are not proven here. Healthcheck, root_body, startup summary, PEM/DER loading and all TLS trait implementations remain shell by explicit scope. No end-to-end TLS, wall-clock liveness, resource-success or independent acceptance claim is made.

Test-only source: src/io_tests.rs, src/io_deadline_tests.rs (included in tls::tests), src/dispatch_oracle.rs, src/record_tests.rs, cfg(test) blocks and tests/*.rs. Acceptance/build tools: scripts/*.py, verification/verify.sh, nix-build.sh, flake.nix and nix/*.nix. These are not shipped server functions. The new deadline test opens only two local slow connections concurrently; inherited tests are unchanged.
