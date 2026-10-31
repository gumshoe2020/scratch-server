# IDLE-CONNECTIONS.md: stage 7 (port 8094)

Branch `idle-connections`, built on `https-headers`. Authoritative for this stage.

## The bug (found by Andrew on a real iPhone, 2026-10-27)
iPhone Safari still got `400 Bad Request` from stage 6 on 8093, logged as a bare `status=400`
(no unknown header). Reproduced by Dudley:
- TLS connect and send a valid request immediately → `200 OK`.
- TLS connect, stay idle 3 seconds, then send the same request → `400 Bad Request`.

Cause: the server answers a read timeout with 400 (`src/lib.rs`, `TimedOut | WouldBlock` arm)
after a 2-second deadline. Safari opens connections ahead of time (preconnect, and reconnects
after the certificate prompt) and sends its request later, so it reads the early 400 as the
answer to its request. The parser is not involved.

## Change (transport only; parser, grammar, whitelist and TLS policy unchanged)
1. Two deadlines instead of one:
   - **Idle before the first request byte:** up to 15 seconds (TLS handshake included). If it
     expires with zero bytes of request received, close the connection silently: no response,
     no 400. A browser then simply opens a new connection.
   - **Once the first byte has arrived:** keep the existing short deadline (2 s between reads)
     and the 4096-byte cap, so a slow client still can't hold a worker by trickling bytes.
     Expiry here still gets 400.
2. Keep the 64-concurrent-connection cap. Note in NOTES.md that the longer idle deadline lets
   idle connections hold slots longer; choose and document what happens at the cap (refuse new
   connections rather than queue unboundedly).
3. Logging: every rejection names its class, e.g. `status=400 reason=grammar`,
   `reason=size`, `reason=slow` (deadline after first byte), `reason=unknown_header name=X`;
   silent idle closes log `closed reason=idle`. Never log values.
4. Apply to both the plain-HTTP path and the TLS path.
5. Port 8094, container `idle-connections`, image tag `idle-connections:static`, same TLS
   certificate mounted read-only (reuse; do not regenerate).

## Acceptance
- New tests, over TLS and plain TCP: connect, idle 3 s, send a valid request → 200; idle past
  the 15 s deadline with no bytes → connection closed with zero response bytes; send half a
  request, stall past the short deadline → 400 `reason=slow`.
- Everything from stage 6 still passes: `./verification/verify.sh` 18/0 with no escape hatches; Safari
  HTTPS fixture 200; malformed corpus 400 with 0 handler calls; 100,000 differential cases;
  `./nix-build.sh check` and `image-static`; static, shell-free, key-free image; stage 5/6
  openssl and curl policy checks on 8094.
- Live: `(sleep 3; printf 'GET / HTTP/1.1\r\nHost: x\r\n\r\n'; sleep 2) | openssl s_client
  -quiet -connect 203.0.113.28:8094 -tls1_3` returns 200.
- Hardened container `idle-connections` on 8094, left running. Do not touch `scratch-server`
  (stopped), 8089, 8090, 8091, 8092 or 8093 containers.
