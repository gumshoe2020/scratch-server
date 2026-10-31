# PRAGMA-HEADER.md: stage 11 (refinement of the HTTPS architecture, port 8091)

Branch `pragma-header`, built on `chromium-headers`. Same evidence rule (explicit whitelist from
observed requests). Authoritative for this stage.

## Evidence
With stage 10 on 8091, Brave on Andrew's <private> got 400, logged twice as
`status=400 reason=unknown_header name=Pragma` (2026-10-27 19:01 EDT). Andrew had DevTools open
with "Disable cache" on; Chromium then sends `Pragma: no-cache` and `Cache-Control: no-cache` on
every request (hard reloads do the same). Developers routinely browse this way.

## Change
1. Whitelist 18 → exactly 19 names: add `Pragma` (case-insensitive like the rest). Accepted and
   ignored. Nothing else changes: grammar, limits, 24-header cap, transport core, TLS.
2. Test on **8095**, container `pragma-header`; Dudley redeploys onto 8091.

## Acceptance
- `./verification/verify.sh` all proof groups pass with the 19-name spec; no assume/admit/external_body/
  external/unsafe first-party; PROOFS.md lists 19 names.
- A Brave "cache disabled" fixture (stage 10 Brave order plus `Pragma: no-cache` and
  `Cache-Control: no-cache`; values synthetic and marked) gets 200 over TLS whole and at
  1/7/1024-byte writes. Near-misses (`Pragmas`, `Pragm`) rejected with 0 handler calls. All
  existing fixtures and tests pass; 100,000 differential cases, 0 mismatches.
- `./nix-build.sh check` and `image-static` pass; formal-coverage re-measured as in stage 10.
- Live on 8095, hardened, cert read-only; stage 10 openssl/curl/idle checks pass. Don't touch
  `scratch-server`, `nix-scratch` (8089), `parser-http` (8090), `parser-https` (8091).
