# BROWSER-WHITELIST.md: stage 4 (port 8091)

Branch `browser-whitelist`, built on `verified-boundary`. Approved by Andrew 2026-10-27:
"white list headers is good". This file is authoritative for this stage.

## Why
Stage 3 rejects every real browser with 400: browsers send headers outside the four-name
whitelist. Andrew chose an explicit whitelist (not "any well-formed header"), so each accepted
header is deliberate.

## Evidence: what iPhone Safari actually sends (captured 2026-10-27, iOS Safari 27.0.1, plain HTTP)
```
GET / HTTP/1.1
Host: 203.0.113.28:8099
User-Agent: Mozilla/5.0 (iPhone; CPU iPhone OS 18_7 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/27.0.1 Mobile/15E148 Safari/604.1
Upgrade-Insecure-Requests: 1
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: en-US,en;q=0.9
Priority: u=0, i
Accept-Encoding: gzip, deflate
Connection: keep-alive

GET /favicon.ico HTTP/1.1
Host: 203.0.113.28:8099
Referer: http://203.0.113.28:8099/
User-Agent: (same)
Accept: */*
Accept-Language: en-US,en;q=0.9
Priority: u=3, i
Accept-Encoding: gzip, deflate
Connection: keep-alive
```

## Change
1. Extend the case-sensitive header-name whitelist in `crates/http-subset` from 4 to 9 names:
   existing `Host`, `Accept`, `Connection`, `User-Agent`, plus `Accept-Language`,
   `Accept-Encoding`, `Upgrade-Insecure-Requests`, `Priority`, `Referer`. Nothing else.
   All other grammar rules and limits stay exactly as they are (printable ASCII values,
   1–256 bytes, no leading/trailing space, ≤16 headers, ≤4096 bytes, GET/HEAD, path charset).
   Note the captured `Referer` value contains `:` and `/`; it is printable and within limits.
2. The new headers are accepted and ignored by the application (no behavior depends on them).
   The server keeps closing every connection (`Connection: keep-alive` is accepted, not honored).
3. Observability (trusted code, outside the proof): when a request is rejected, write one
   stdout line with the status and, if the rejection is an unknown header name, that name only,
   sanitized to `[A-Za-z0-9-]` and truncated to 64 bytes. Never log values (they can hold
   cookies or tokens). This lets the next missing browser header be seen in `docker logs`.
4. Port 8091, container name `browser-whitelist`, image tag `browser-whitelist:static`.

## Acceptance
- `./verification/verify.sh`: all proofs pass, with the spec function updated to the 9-name whitelist;
  still no `assume`/`admit`/`external_body`/`unsafe`. PROOFS.md updated (list of names).
- Exact bytes of both captured requests above (with `Host` pointed at 8091) parse and get
  200 (`/`) and 404 (`/favicon.ico`) in a test, both whole and split into 1/7/1024-byte reads.
- The 48-case malformed corpus still gets 400 with 0 handler calls. Add cases: each new header
  name with wrong case (e.g. `accept-language`), with an empty value, with a 257-byte value,
  and a header name that is a prefix/extension of a whitelisted one (`Accept-Languages`,
  `Refere`), all rejected. A rejection of `Cookie: x` logs `Cookie` and not `x`.
- Differential test (M3) updated to generate the 9 names; 100,000 cases, 0 mismatches.
- `./nix-build.sh check` passes; `./nix-build.sh image-static` builds; image stays static,
  shell-free, under 8 MB.
- Container `browser-whitelist` on 8091 with `--read-only --cap-drop ALL --security-opt
  no-new-privileges`; a curl replay of the captured Safari request returns 200. Leave it running.
- Do not touch containers `nix-scratch` (8089), `verified-boundary` (8090) or the stopped
  `scratch-server` (8088). TLS stays out of scope (still blocked; see TLS-COVERAGE.md).
