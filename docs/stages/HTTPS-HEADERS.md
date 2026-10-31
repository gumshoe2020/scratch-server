# HTTPS-HEADERS.md: stage 6 (port 8093)

Branch `https-headers`, built on `https`. Same rule Andrew set for stage 4: an explicit
whitelist built from captured evidence. This file is authoritative for this stage.

## Evidence
Stage 5 on 8092 rejected iPhone Safari with `status=400 unknown_header=Sec-Fetch-Dest` (the log
names only the first unknown header). A one-shot HTTPS catcher (same certificate) then recorded
the header NAMES (values deliberately not recorded) that iPhone Safari 27.0.1 sends over HTTPS,
2026-10-27, in this order, for both `GET /` and `GET /favicon.ico`:

Host, Sec-Fetch-Dest, User-Agent, Accept, Referer, Sec-Fetch-Site, Sec-Fetch-Mode,
Accept-Language, Priority, Accept-Encoding, Connection

New relative to the stage 4 whitelist: `Sec-Fetch-Dest`, `Sec-Fetch-Site`, `Sec-Fetch-Mode`.
(`Upgrade-Insecure-Requests` is not sent over HTTPS but stays whitelisted for HTTP clients.)

## Change
1. Whitelist grows from 9 to exactly 12 case-sensitive names: add `Sec-Fetch-Dest`,
   `Sec-Fetch-Site`, `Sec-Fetch-Mode`. Nothing else; no other grammar or limit changes.
   Accepted and ignored by the application.
2. Everything else from stage 5 unchanged: TLS 1.3 / ChaCha20-Poly1305 / X25519 / ECDSA P-256
   on the libcrux provider, HTTP/1.1 ALPN only, certificate mounted read-only from
   <workspace> (reuse it; do not regenerate).
3. Port 8093, container `https-headers`, image tag `https-headers:static`.

## Acceptance
- `./verification/verify.sh` passes against the 12-name spec (still no assume/admit/external_body/unsafe);
  PROOFS.md lists the 12 names.
- A fixture with Safari's HTTPS header order above and realistic values (values were not
  captured; use standard ones such as `Sec-Fetch-Dest: document`, `Sec-Fetch-Site: none`,
  `Sec-Fetch-Mode: navigate`, and mark the fixture as name-order captured, values synthetic)
  gets 200 over TLS, whole and at 1/7/1024-byte writes; a favicon variant gets 404.
- Malformed corpus still 400 with 0 handler calls; add wrong-case and near-miss cases for the
  3 new names (`sec-fetch-dest`, `Sec-Fetch-Destination`, `Sec-Fetch-`).
- 100,000 differential cases over the 12 names, 0 mismatches.
- `./nix-build.sh check` and `./nix-build.sh image-static` pass; static, shell-free, no key in image.
- openssl/curl checks from stage 5 still pass on 8093 (same suite/group/signature; TLS 1.2,
  AES suites, h2 and plain HTTP refused); curl replay of the fixture returns 200.
- Hardened container `https-headers` on 8093, left running. Do not touch `scratch-server`
  (stopped), `nix-scratch` 8089, `verified-boundary` 8090, `browser-whitelist` 8091, `https` 8092.
