# HTTPS.md: stage 5 (port 8092)

Branch `https`, built on `browser-whitelist`. Approved by Andrew 2026-10-27 ("Let's fork a new
branch for HTTPS now"). Direction: option 1 from the stage-3 M4 blocker (build TLS on HACL*-derived
verified crypto via Cryspen's libcrux), falling back to option 2 (best available crypto, labeled
honestly) for any piece that has no verified implementation. This file is authoritative.

## Grounded facts (probed 2026-10-27; re-verify from primary sources)
- Stage 3's TLS-COVERAGE.md: AWS-LC's published proofs don't cover a complete TLS 1.3 suite on
  ARM. Don't use aws-lc-rs for this stage.
- crates.io has libcrux crates: libcrux-chacha20poly1305 0.0.10, libcrux-curve25519 0.0.9,
  libcrux-sha2 0.0.9, libcrux-hkdf 0.0.9, libcrux-hmac 0.0.9, libcrux-p256 0.0.9,
  libcrux-ed25519 0.0.10, libcrux-ecdsa 0.0.9. No `rustls-libcrux-provider` crate exists.
- libcrux-chacha20poly1305's description says it is safe Rust compiled from HACL* C that is
  verified in F* for memory safety, functional correctness and secret independence. Confirm this,
  and check each other crate individually: some libcrux code may be portable/unverified.
- Host is an arm64 Mac; Linux builds run in `nixos/nix` via ./nix-build.sh.

## Design
1. **TLS 1.3 only, one suite:** `TLS13_CHACHA20_POLY1305_SHA256`, key exchange X25519. ALPN
   offers `http/1.1` only (no HTTP/2), so decrypted bytes still go to the proven parser.
2. **A custom rustls `CryptoProvider` built from libcrux crates** for: AEAD (ChaCha20-Poly1305),
   key exchange (X25519), hash (SHA-256), HMAC/HKDF (SHA-256), and the server's signature scheme.
   For the signature, choose ECDSA P-256 (best Safari compatibility) unless libcrux's P-256 is
   unverified and its Ed25519 is verified, in which case document the trade-off and pick one.
   Randomness comes from the OS (getrandom); it is trusted.
3. **Order of trust on the wire:** TCP → rustls record layer and handshake (trusted, not proven)
   → libcrux crypto (verified where TLS-PROVIDER.md says so) → plaintext → the proven HTTP parser,
   unchanged from stage 4. The 9-name header whitelist does not change in this stage.
4. **Certificates:** self-signed, generated outside the repo into
   `<workspace>` (create it; mode 700; key 600)
   with `scripts/make-lan-cert.py` (adapt key type to the chosen signature scheme; include
   `203.0.113.28` as an IP SAN). Mounted read-only into the container at runtime. Never committed,
   never baked into the image.
5. Port 8092, container `https`, image tag `https:static`. HTTPS only on this port.

## Expected and intended
Safari over HTTPS will warn about the self-signed certificate (tap through). Safari likely sends
extra headers on HTTPS (e.g. `Sec-Fetch-*`), which will get 400 and be logged by name. Do NOT add
them: Andrew's rule is an explicit whitelist built from captured evidence. Dudley will read the
names from `docker logs https` after Andrew visits, and they go into a later stage.

## Acceptance
- **TLS-PROVIDER.md:** a table of every crypto operation the provider uses → crate + version →
  verified? (what property, by what tool, citing the primary source URL you read) or "not
  verified"/"unconfirmed". Plus what stays trusted: rustls handshake/record layer, X.509
  encoding of our cert, OS randomness, provider glue code.
- Rust tests: a rustls client using the same provider completes a handshake with the server and
  gets 200 for `/`; the captured Safari request bytes (stage 4 fixture) sent over TLS get 200;
  the malformed corpus over TLS still gets 400 with 0 handler calls.
- From outside: `openssl s_client -connect 203.0.113.28:8092 -tls1_3` shows protocol TLSv1.3,
  cipher TLS_CHACHA20_POLY1305_SHA256 and X25519; `-tls1_2` fails; requesting another TLS 1.3
  suite (e.g. `-ciphersuites TLS_AES_128_GCM_SHA256`) fails; `curl -k https://203.0.113.28:8092/`
  returns the page; plain HTTP to 8092 is refused.
- `./verification/verify.sh` still passes (18/0, no escape hatches). 100,000 differential cases still pass.
- `./nix-build.sh check` passes (proofs, tests, cargo-deny); `./nix-build.sh image-static` builds;
  image static, shell-free, no cert/key inside (`docker export` shows only the binary).
- Hardened container `https` on 8092 (`--read-only --cap-drop ALL --security-opt
  no-new-privileges`, cert dir mounted read-only); left running.
- Do not touch containers `scratch-server` (stopped), `nix-scratch` (8089), `verified-boundary`
  (8090), `browser-whitelist` (8091).
- If a full libcrux provider is not achievable, stop, record the exact blocker and which pieces
  worked in NOTES.md, and do not substitute aws-lc-rs or ring silently.
