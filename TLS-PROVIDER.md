# Stage 5 provider boundary

TCP → trusted rustls record/handshake code → libcrux primitive adapters →
authenticated plaintext → unchanged Verus HTTP parser → trusted dispatch.
TLS 1.3 only; TLS_CHACHA20_POLY1305_SHA256 only; X25519 only; ALPN http/1.1
only (clients without ALPN may connect). No TLS 1.2, QUIC, client authentication,
resumption, early data or ticket encryption. Signatures use ECDSA P-256/SHA-256.

## Primary-source coverage audit (2026-10-27)

Every linked published crate archive below was downloaded and its README,
manifest and selected API implementation read. All seven archives identify source
commit `42d68bd49b244b09bd02626c6ffa95e54dc64f91` in `.cargo_vcs_info.json`.
“Upstream verified” below means the publisher identifies HACL*-derived code,
verified in F* for memory safety, functional correctness and secret independence.
It does **not** mean our adapters or ARM executable have been formally verified.
Exact release-to-proof and compiler/target correspondence is **unconfirmed**;
no F* proofs or translation proof were rerun here.

| Operation actually used | Crate/version and primary source read | Verification scope/status |
| --- | --- | --- |
| TLS record AEAD seal/open, ChaCha20 + Poly1305 | [libcrux-chacha20poly1305 0.0.10](https://static.crates.io/crates/libcrux-chacha20poly1305/libcrux-chacha20poly1305-0.0.10.crate), README and src/impl_hacl.rs; transitive libcrux-poly1305 0.0.7/libcrux-hacl-rs 0.0.6 | Upstream verified primitives, F* properties above. Uses ordinary ChaCha20Poly1305, not the explicitly unverified XChaCha wrapper. Stage 8 proves nonce construction and a conditional plaintext-release copy gate; AAD and trait wiring remain unverified. |
| X25519 public derivation and shared secret, zero-result rejection | [libcrux-curve25519 0.0.9](https://static.crates.io/crates/libcrux-curve25519/libcrux-curve25519-0.0.9.crate), README, src/lib.rs and src/impl_hacl.rs | Upstream verified HACL curve25519_51 primitives, F* properties above. Length checks and adapter unverified. |
| SHA-256 transcript/one-shot hashing and hash forks | [libcrux-sha2 0.0.9](https://static.crates.io/crates/libcrux-sha2/libcrux-sha2-0.0.9.crate), README and src/impl_hacl.rs | Upstream verified HACL SHA2, F* properties above. Rust trait adapters unverified. Streaming state avoids accumulating the transcript. |
| HMAC-SHA256 Finished signing | [libcrux-hmac 0.0.9](https://static.crates.io/crates/libcrux-hmac/libcrux-hmac-0.0.9.crate), README and src/impl_hacl.rs | Upstream verified HACL HMAC, F* properties above; uses hmac_sha2_256. |
| HKDF-SHA256 extract/expand, TLS traffic-secret derivation | [libcrux-hkdf 0.0.9](https://static.crates.io/crates/libcrux-hkdf/libcrux-hkdf-0.0.9.crate), README and src/hkdf.rs | Upstream verified HACL SHA2 HKDF, F* properties above. Slice concatenation/classification and length adapters unverified. SHA3 implementations are not selected. |
| ECDSA P-256/SHA256 server signing, scalar/public-key validation and derivation, signature verification for same-provider client | [libcrux-ecdsa 0.0.9](https://static.crates.io/crates/libcrux-ecdsa/libcrux-ecdsa-0.0.9.crate), README and src/p256.rs; [libcrux-p256 0.0.9](https://static.crates.io/crates/libcrux-p256/libcrux-p256-0.0.9.crate), README | Upstream verified HACL P256 primitive, F* properties above. ECDSA random nonce wrapper and DER signature codec coverage unconfirmed. Both P256 and Ed25519 publish HACL-derived claims; chose P256 as specified for Safari compatibility. |
| Nonces, ephemeral scalars and protocol randomness | getrandom 0.3.4; OS source; rand_core 0.10.1 adapter for libcrux nonce generation | Trusted, not verified. Rejection sampling is libcrux's random_scalar; rand_core supplies traits, not another PRNG. |
| Key and signature encoding, SPKI and X.509 processing | pkcs8 0.10.2, sec1 0.7.3, rustls 0.23.46, rustls-pki-types 1.15.1, rustls-webpki 0.103.16; libcrux DER signature codec | Not verified locally; encoding/parser correctness trusted. Only PKCS8 id-ecPublicKey/prime256v1 keys accepted; startup checks certificate/key correspondence. |

The [HACL* manual](https://hacl-star.github.io/) and
[HACL* repository README](https://github.com/hacl-star/hacl-star) were read;
they describe F* verification and compilation to C. The
[translation paper abstract](https://arxiv.org/abs/2412.15042) was read; it describes
translation of structured C to safe Rust, not a proof of our TLS program.
Crate-level provenance is stronger evidence than matching algorithm names, but
is not independently reproduced proof coverage for this exact binary.

## Trusted components and limitations

rustls 0.23.46's state machine, record limits, nonce sequencing, TLSInnerPlaintext
validation, certificate handling and protocol framing remain trusted. Its
published crate source was read for provider traits, record adapters and suite
configuration. Provider trait glue in src/tls.rs remains tested; stage 8 uses separately proven nonce, length and release helpers. OS randomness,
X.509 generation via host OpenSSL, PKCS8/SEC1/PEM codecs, upstream wrappers,
allocation, secret-memory cleanup, Rust/compiler/LLVM, OS, Docker and hardware
remain trusted. Our owned secret buffers use zeroize; upstream transient key and
nonce copies are not claimed to be erased. The self-signed certificate offers no
public CA authentication; local clients explicitly trust its certificate, while
curl acceptance uses -k. No keys/certificates are build inputs or image payload.

cargo-deny retains one narrow informational advisory exception:
RUSTSEC-2026-0173 (unmaintained proc-macro-error 2 under cfg(hax), transitive
hax-lib-macros 0.3.7). This dependency is not built in our ordinary Cargo or Verus
configuration. No vulnerability advisory is ignored. BSD-3-Clause and ISC are
explicitly allowed for the added permissively licensed dependencies.

Default rustls crypto features are disabled. No active ring or aws-lc-rs backend
is permitted. Cargo.lock may list ring as an optional webpki dependency; check
`cargo tree --target aarch64-unknown-linux-musl -i ring` and the actual feature
graph, rather than treating that lockfile entry as a linked implementation.
Neither built-in provider is used by test-certificate generation: rcgen has all
default features disabled and calls our libcrux-backed signer in memory.

Stage 8 also proves the transport core, formatter and selected AEAD helpers.
See PROOFS.md for exact properties and the remaining adapter boundary. End-to-end
verified TLS is not claimed.
