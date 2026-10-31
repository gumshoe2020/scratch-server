# FIREFOX-HEADERS.md: stage 9 (refinement of the HTTPS architecture, port 8091)

Branch `firefox-headers`, built on `proven-transport`. Same evidence rule as stages 4 and 6
(Andrew: explicit whitelist from captured requests). Authoritative for this stage.

## Evidence
Firefox on Andrew's <private> got 400 from stage 8 on 8091, logged as
`status=400 reason=unknown_header name=Sec-Fetch-User`. A one-shot HTTPS catcher (same
certificate) then recorded header NAMES and value lengths only (no values), 2026-10-27:

`GET /` (451 bytes): Host, User-Agent, Accept, Accept-Language, Accept-Encoding, Connection,
Upgrade-Insecure-Requests, Sec-Fetch-Dest, Sec-Fetch-Mode, Sec-Fetch-Site, Sec-Fetch-User, Priority

`GET /favicon.ico` (465 bytes): Host, User-Agent, Accept, Accept-Language, Accept-Encoding,
Connection, Referer, Sec-Fetch-Dest, Sec-Fetch-Mode, Sec-Fetch-Site, Priority

The only name not already whitelisted: `Sec-Fetch-User` (value length 3, e.g. `?1`).

## Change
1. Whitelist grows from 12 to exactly 13 case-sensitive names: add `Sec-Fetch-User`. Nothing
   else; no other grammar, limit, transport-core or TLS change.
2. Port: test on **8095**, container `firefox-headers`; Dudley redeploys onto 8091 after
   acceptance.

## Acceptance
- `./verification/verify.sh` passes all proof groups (parser against the 13-name spec, transport core, AEAD
  helpers); no assume/admit/external_body/external/unsafe first-party. PROOFS.md lists 13 names.
- A Firefox fixture in the captured name order (values synthetic and marked so, e.g.
  `Sec-Fetch-User: ?1`) gets 200 over TLS whole and at 1/7/1024-byte writes; favicon variant 404.
- Wrong-case and near-miss cases for the new name (`sec-fetch-user`, `Sec-Fetch-Users`,
  `Sec-Fetch-Use`) are rejected with 0 handler calls; existing malformed corpus still 400.
- All existing tests (Safari HTTP/HTTPS fixtures, transport/idle/slow tests, 100,000 differential
  cases over 13 names) pass. `./nix-build.sh check` and `image-static` pass.
- Re-measure with formal-coverage exactly as stage 8 did (same config, same three-run union
  method, clearly labeled); record the number in NOTES.md.
- Live on 8095 (hardened, cert mounted read-only): openssl/curl checks from stage 8 pass, fixture
  replay 200. Don't touch `scratch-server`, `nix-scratch` (8089), `parser-http` (8090),
  `parser-https` (8091).
