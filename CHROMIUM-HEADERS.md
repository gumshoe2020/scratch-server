# CHROMIUM-HEADERS.md: stage 10 (refinement of the HTTPS architecture, port 8091)

Branch `chromium-headers`, built on `firefox-headers`. Approved by Andrew 2026-10-27 ("go") on
three decisions. Authoritative for this stage.

## Evidence
Brave (Chromium) on Andrew's <private> got 400 from stage 9 on 8091:
`status=400 reason=unknown_header name=sec-ch-ua` and `... name=sec-ch-ua-platform`.
A one-shot HTTPS catcher recorded names (exact case) and value lengths only, first visit and
reload (identical), 2026-10-27:

`GET /` (667 bytes, 15 headers): Host, Connection, Cache-Control, sec-ch-ua, sec-ch-ua-mobile,
sec-ch-ua-platform, Upgrade-Insecure-Requests, User-Agent, Accept, Sec-GPC, Sec-Fetch-Site,
Sec-Fetch-Mode, Sec-Fetch-User, Sec-Fetch-Dest, Accept-Encoding, Accept-Language

`GET /favicon.ico` (610 bytes, 14 headers): Host, Connection, sec-ch-ua-platform, User-Agent,
sec-ch-ua, sec-ch-ua-mobile, Accept, Sec-GPC, Sec-Fetch-Site, Sec-Fetch-Mode, Sec-Fetch-Dest,
Referer, Accept-Encoding, Accept-Language

All values printable ASCII; longest User-Agent 112 bytes.

## Decisions (Andrew, 2026-10-27)
1. **Header names match case-insensitively** (ASCII case folding only), as HTTP defines them.
   The whitelist stays explicit: a name is accepted iff it equals a whitelisted name ignoring
   ASCII case. Near-misses (`Sec-Fetch-Users`, `Sec-Fetch-`, `sec-ch-ua-mobil`) are still
   rejected. Names never whitelisted (`Content-Length`, `Transfer-Encoding`, `Cookie`, ...) are
   rejected in every case form.
2. **Whitelist 13 → 18**, adding exactly: `Cache-Control`, `Sec-GPC`, `Sec-CH-UA`,
   `Sec-CH-UA-Mobile`, `Sec-CH-UA-Platform`. Accepted and ignored.
3. **Header-count cap 16 → 24.** All other limits unchanged (4096-byte request, 256-byte path,
   1–256-byte printable values, no leading/trailing space).

## Acceptance
- `./verify.sh`: the parser spec function states the case-insensitive, 18-name, 24-header grammar
  and all three proof groups pass; no assume/admit/external_body/external/unsafe first-party.
  PROOFS.md updated (names, folding rule, cap).
- Tests that previously asserted wrong-case rejection now assert acceptance for whitelisted
  names in any case mix; near-miss and non-whitelisted names (any case) still rejected with 0
  handler calls. Boundary tests: exactly 24 headers accepted, 25 rejected.
- Brave fixtures (both captured name orders; values synthetic and marked so) get 200 / 404 over
  TLS, whole and at 1/7/1024-byte writes. All Safari and Firefox fixtures still pass.
- Differential test updated (httparse already matches names case-insensitively): 100,000 cases,
  0 mismatches, generator covering mixed-case names and 0–24 headers.
- Rejection log keeps the sanitized name exactly as sent (never values).
- `./nix-build.sh check` and `image-static` pass; formal-coverage re-measured exactly as stage 9.
- Live on 8095, container `chromium-headers`, hardened, cert mounted read-only; stage 8/9 openssl,
  curl and idle checks pass; Brave fixture replay 200. Don't touch `scratch-server`,
  `nix-scratch` (8089), `parser-http` (8090), `parser-https` (8091).
