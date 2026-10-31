# PROVEN-RECORD.md: stage 15 (refinement of the HTTPS architecture, port 8091)

Branch `proven-record`, built on `proven-dispatch` at `13c8124`. Approved by Andrew 2026-10-30
("Yes for stage 15 too"), including the property 3 saturation. Authoritative for this stage.

## Why
formal-coverage v1.5 (aarch64-linux-musl, compiled-files) puts first-party security code at
**1,011 / 1,617 SLoC proven (62.52%)** at `13c8124`. The largest unproven block left is
`src/tls.rs`: 305 SLoC, none of it proven. That file is the rustls crypto provider built on
libcrux.

Stage 8 already proved the record-layer arithmetic that faces attackers (`transport-core/src/aead.rs`:
nonce = IV XOR sequence, sealed and plaintext lengths, and plaintext released only after
authentication). Most of what remains in `tls.rs` calls into libcrux (verified upstream) or rustls
(trusted). Verus cannot reason across those calls without `external_body`, which is banned, so
those lines stay in the shell. This stage proves the few decisions left in `tls.rs`, or delegated
from it to rustls, that act on **bytes a client controls**:

| Decision | Where today | Attacker input |
|---|---|---|
| TLS 1.3 inner-plaintext unpadding (content type, padding, size, CCS) | rustls `into_tls13_unpadded_message`, called from `RecordCipher::decrypt` | decrypted record plaintext; the client holds the keys, so it chooses these bytes freely |
| Record AAD encoding | rustls `make_tls13_aad`, called from `encrypt` and `decrypt` | record length |
| Inner plaintext construction and `encrypted_payload_len` | `RecordCipher::encrypt` | payload length |
| X25519 peer key-share shape | `Exchange::complete` | ClientHello key share |
| P-256 public-key shape | `P256Verify::verify_signature` | certificate key (unused today: no client auth) |
| Signature-scheme choice | `P256SigningKey::choose_scheme` | ClientHello `signature_algorithms` |

This is a deliberately small stage. Expected gain: roughly +40–60 proven SLoC (about +2–4 points).
Report what actually happens.

## Approach
Same pattern as stages 8 and 14: a pure core in Verus and a thin shell. Add
`crates/transport-core/src/record.rs`, declared `pub mod record;` in `crates/transport-core/src/lib.rs`,
inside `verus!`, so `./verify.sh` and formal-coverage pick it up with no tool or config change. Reuse
`aead.rs` specs; don't duplicate them. **Do not edit the formal-coverage repository.**

`src/tls.rs` keeps the trait impls, the libcrux and rustls calls, `Zeroizing`, randomness and config
loading. It converts rustls types to plain integers and slices before calling the core, and converts
the results back.

## Properties to prove (all inputs, no bounded checking)

1. **Unpadding.** `unpad(plain: &[u8]) -> Result<Unpadded { typ: u8, len: usize }, UnpadError>`, with
   `UnpadError { Oversized, NoContentType, ChangeCipherSpec }`, mirroring rustls 0.23.46
   `into_tls13_unpadded_message` exactly, **check order included**:
   1. `plain.len() > 16_385` gives `Oversized`;
   2. strip trailing zero bytes; if none are nonzero, give `NoContentType`;
   3. the last nonzero byte is `typ`; if `typ == 20` (change_cipher_spec), give `ChangeCipherSpec`;
   4. if the remaining length is `> 16_384`, give `Oversized`;
   5. otherwise `Ok`.

   Prove: `Ok` implies `len < plain.len()`, `plain[len] == typ`, `typ != 0`, `typ != 20`, `len <= 16_384`
   and every byte of `plain[len + 1..]` is zero (so `len` is the *last* nonzero position, not just any
   one). Also prove `NoContentType` iff `plain.len() <= 16_385` and every byte is zero. The shell
   replaces the `msg.into_tls13_unpadded_message()` call with `unpad`. It maps each error to the
   **identical** rustls `Error` value (`PeerSentOversizedRecord`,
   `PeerMisbehaved::IllegalTlsInnerPlaintext`, `PeerMisbehaved::IllegalMiddleboxChangeCipherSpec`) and
   on `Ok` builds `InboundPlainMessage { typ: ContentType::from(typ), version: TLSv1_3, payload:
   &payload[..len] }`.
2. **AAD.** `aad(len: usize) -> [u8; 5]` = `[0x17, 0x03, 0x03, (len >> 8) as u8, len as u8]`. Prove
   the fixed prefix, and that for `len < 65_536` the last two bytes are `len` big-endian (decode == len).
   It replaces both `make_tls13_aad` calls.
3. **Inner plaintext.** `inner_plaintext(payload: &[u8], typ: u8) -> Option<Vec<u8>>`: `Some` iff
   `aead::sealed_len(payload.len())` is `Some`; the output is `payload ++ [typ]` and
   `out.len() + TAG_LEN == sealed_len(payload.len())`. `encrypted_payload_len(len)` is proven equal
   to `sealed_len(len)` whenever that is `Some`. Today it is unchecked `len + 17`. For
   `len > usize::MAX - 17` it must saturate to `usize::MAX`, not wrap or panic. This is the only
   permitted behaviour change. It can't happen in practice, since rustls passes at most 16,384. Log it
   in NOTES.md.
4. **Key share.** `x25519_peer(peer: &[u8]) -> Option<[u8; 32]>`: `Some` iff `peer.len() == 32`, with
   identical contents. It replaces the `try_into` in `Exchange::complete`. The libcrux `ecdh` call
   (all-zero shared-secret rejection included) stays in the shell.
5. **P-256 point shape.** `p256_point_shape(public: &[u8]) -> bool` iff `public.len() == 65 &&
   public[0] == 4`. It replaces the inline check in `verify_signature`.
6. **Scheme choice.** `offers(offered: &[u16], wanted: u16) -> bool` iff some element equals `wanted`.
   The shell converts `SignatureScheme` to `u16` and passes `0x0403` (ECDSA_NISTP256_SHA256).

## Behaviour must not change
Handshakes, records, alerts and error values must be identical to `13c8124` for every input, except
the saturation in property 3. Oracle tests call the rustls originals directly (they stay in the
dependency), so nothing old has to be copied:
- **Unpad:** 100,000 generated plaintexts through rustls `InboundOpaqueMessage::into_tls13_unpadded_message`
  and through the new shell path, with results (typ, version, payload) or `Error` values identical.
  Include all-zero buffers, every content-type byte 0–255, 20 (CCS) with and without padding, lengths
  0, 1, 16,384, 16,385, 16,386 and 16,385 + padding, long zero runs, and random bytes.
- **AAD:** every `len` in `0..=70_000`, plus 10,000 random `usize` values, against `make_tls13_aad`.
- **Inner plaintext / length:** random payloads up to 16,384 bytes and every content type, against
  the old construction; `encrypted_payload_len` against `len + 17` for `len <= usize::MAX - 17`.
- **Key share, point shape, scheme choice:** lengths 0–100 and random lists against the old code.
- End to end: the existing TLS tests (handshake, record round-trip, rejected suites/versions/groups)
  unchanged and passing.
If an equivalence test exposes a difference you believe is a real bug in the old code (or in rustls),
**stop, commit the evidence, and explain it in NOTES.md.** Don't fix it; a fix needs Andrew's approval.

## Acceptance
- `./verify.sh` passes with 0 errors. Report the verified count per root. There must be no `assume`,
  `admit`, `external_body`, `external` or `unsafe` anywhere first-party (scan and show it).
  PROOFS.md lists each new property with file:line and an updated "still unverified" list for `tls.rs`.
- `cargo test --workspace` passes: the 39 existing tests unchanged in meaning, plus the new
  equivalence tests.
- The stage 13 spec-mutant run still catches 15 of 15.
- `./nix-build.sh check` and `./nix-build.sh image-static` pass; the image is static, shell-free
  and key-free.
- Live on a test port: container `proven-record` on **8097**, hardened as before (read-only, no
  capabilities, non-root), with the TLS cert mounted read-only. The stage 5–8 openssl/curl checks pass
  (TLS 1.3 / ChaCha20-Poly1305 / X25519 / ECDSA P-256 negotiated; TLS 1.2, AES suites, P-256 key
  exchange and h2 refused; plain HTTP refused; the 3-second delayed request returns 200; the 17-second
  idle connection closes with 0 bytes). Safari, Firefox and Brave fixture replays return 200, and
  responses and log lines for a scripted request set are byte-identical to `proven-dispatch` on 8096.
- Do not touch the containers `scratch-server` (stopped), `nix-scratch` (8089), `parser-http` (8090),
  `parser-https` (8091), `header-policy` (8095) or `proven-dispatch` (8096).

## Measurement
Dudley measures with formal-coverage v1.5 at acceptance. Codex does **not** run formal-coverage.
In NOTES.md, list every function moved or added with old and new location and proven-or-shell status,
and give the SLoC of each `tls.rs` function before and after.

## Out of scope
HKDF, HMAC, SHA-256, X25519 and P-256 arithmetic, and ChaCha20-Poly1305 (libcrux, verified
upstream); the `OsRandom` adapter; private-key loading (`load_private_key`: an operator-supplied file
read once at startup, not network input; its PKCS#8/SEC1 DER parsing stays with the `pkcs8`/`sec1`
crates); `provider`, `server_config` and `load_config`; rustls itself (handshake parsing remains the
largest unproven attack surface and is not addressed here); `src/lib.rs`, `src/main.rs` and
`src/transport.rs`; the parser, policy, grammar and existing proofs (extend them only if needed, never
weaken them); EVOLUTION.md and ATTACK-SURFACE.md (Dudley updates these after acceptance); pushing.
