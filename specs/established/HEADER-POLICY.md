# HEADER-POLICY.md: stage 12 (refinement of the HTTPS architecture, port 8091)

Branch `header-policy`, built on `pragma-header`. Approved by Andrew 2026-10-27 ("yes please").
Authoritative for this stage. This replaces the stage 3–11 evidence rule (explicit whitelist of
observed names) with a configurable, proven header policy.

## Evidence
Stage 11 on 8091 still got 400 from Brave: `status=400 reason=unknown_header name=dnt`
(2026-10-27 19:10 EDT). This is the fourth browser- or extension-specific header in one day
(Sec-Fetch-User, client hints, Pragma, DNT). Most likely an extension (Privacy Badger) adds it, so
the set of headers browsers send depends on per-visitor settings and can't be enumerated. The page
code reads no header values, so ignoring a well-formed header adds almost no attack surface. The
danger lies in headers that change how a request is framed or what it means.

## Design
Every header line is still parsed by the proven grammar. Each name then gets one policy:

- **reject**: 400, 0 handler calls (exactly as today's `unknown_header`).
- **drop**: accepted, logged (subject to the log limit), removed. Nothing downstream of the parser
  can see it.
- **use**: the handler may read it. This set is fixed in code, and today it is empty, because the
  handler reads only method and path. Config can't grant `use`. The request type exposes only the
  values of `use` headers, so code can't read a dropped header even by mistake.

Name classes:

1. **Known** (the 19 stage-11 names): drop, without logging. The stage-11 whitelist is kept as the
   "quiet" list.
2. **Hard-reject** (fixed in code; config can't change it): `Content-Length`, `Transfer-Encoding`,
   `TE`, `Trailer`, `Expect`, `Upgrade`, `HTTP2-Settings`, `Proxy-Connection`, `Keep-Alive`. The
   server has no body or protocol-switching code, so any of these is either a smuggling attempt or
   a misdirected client.
3. **Guarded** (default reject; config can set each to `drop`): `Cookie`, `Authorization`,
   `Proxy-Authorization`, `Range`, `If-Range`, `If-Match`, `If-None-Match`, `If-Modified-Since`,
   `If-Unmodified-Since`, `Forwarded`, `X-Forwarded-For`, `X-Forwarded-Host`,
   `X-Forwarded-Proto`, `X-Real-IP`, `Via`, `Max-Forwards`.
4. **Unknown** (every other well-formed name): the config's `unknown` policy, default `drop` with
   logging. `unknown reject` reproduces the stage-11 behaviour exactly ("strict mode").

All name matching is ASCII case-insensitive, as in stage 10.

Additional always-on rules:
- A duplicate `Host` or `Connection` (any case) is rejected. Other duplicates follow their policy.
- Header name grammar for any name: RFC 9110 `token` characters, 1–64 bytes. Values: unchanged
  (1–256 printable ASCII, no leading or trailing space). Total size 4096, path 256 and the
  24-header cap are unchanged.

## Configuration
- File path from env `SCRATCH_HEADER_POLICY`. If the variable is unset, use the built-in default
  policy and log `policy=builtin`. If it is set, the file must load; otherwise exit non-zero
  before binding the port (fail closed).
- The format is plain text, one directive per line; `#` starts a comment and blank lines are
  ignored:
  ```
  unknown drop|reject
  guarded <Name> drop|reject      # Name must be in the guarded list
  log-unknown-per-minute <0..600>
  ```
- Refuse to start on: an unknown directive, a name outside the guarded list (including any
  hard-reject or known name), a repeated directive for the same name, a bad number, invalid
  UTF-8/ASCII, a file over 4096 bytes, or more than 64 lines.
- Built-in default = `unknown drop`, every guarded name `reject`, `log-unknown-per-minute 60`.
- At startup, log one line with the effective policy: unknown policy, the guarded names set to
  drop, and the log limit. No other config content is logged.
- Ship `config/header-policy.conf` (the built-in default, written out explicitly with comments)
  and `config/header-policy-strict.conf` (`unknown reject`).

## Logging
- For a dropped unknown name: `status=<code> note=unknown_header name=<sanitized>`, logged at most
  once per distinct name per minute, with at most `log-unknown-per-minute` lines in any
  minute in total, and at most 64 distinct names tracked (bounded memory). Names are sanitised as
  today. Values are never logged.
- Rejects log as today (`reason=unknown_header` becomes `reason=hard_reject`, `reason=guarded`,
  `reason=unknown_header` (strict mode) or `reason=duplicate`, with the name). Fix the
  stage-10 cosmetic issue: 25 headers logs `reason=too_many_headers`, not `reason=grammar`.

## Proofs (Verus)
The policy parser and the policy decision both go in the proven crate. They must hold **for every
policy value the config parser can return**, not just the default:

1. **Config parser**: for every input byte sequence, the parser either fails or returns a policy in
   which every hard-reject name is `reject` and the `use` set is the code-fixed set (empty).
2. **Request**: for every input and every valid policy, the request is accepted only if no header is
   hard-reject, there is no duplicate Host or Connection, and every guarded or unknown name's policy
   is `drop`. The returned request exposes no dropped-header bytes.
3. **Strict-mode equivalence**: with `unknown reject` and every guarded name `reject`, a request is
   accepted if and only if stage 11's `request_spec` accepts it **and** it has no duplicate `Host`
   or `Connection` (any case). Accepted requests have the same method and path as stage 11. The
   duplicate rule is a deliberate tightening, so this is the only difference allowed.
   (Amended 2026-10-27 after Codex found the original wording contradicted the always-on
   duplicate rule.)
4. All existing properties stay true: no overflow, out-of-bounds access or panic, and all loops
   terminate. Never use assume, admit, external_body, external or first-party unsafe.

## Acceptance
- `./verification/verify.sh` passes all proof groups. PROOFS.md documents the policy model, the four new
  properties, and the hard-reject and guarded lists.
- Default policy over TLS (whole and at 1/7/1024-byte writes): the stage-11 Safari, Firefox,
  Brave and Brave-cache-disabled fixtures each get 200. A new Brave+DNT fixture (stage 11
  cache-disabled plus `DNT: 1` and `Sec-GPC: 1`; values synthetic and marked) gets 200 and logs
  `note=unknown_header name=DNT` once.
- Every hard-reject and guarded name, in mixed case, gets 400 with 0 handler calls under the
  default policy. Guarded `Cookie` set to `drop` gets 200. No config can make `Content-Length` or
  `Transfer-Encoding` pass (tested with each name in the config: startup refused).
- Strict config: the DNT fixture gets 400 `reason=unknown_header`. All stage-11 tests pass
  unchanged, except tests that assert duplicate Host/Connection is accepted; those now assert 400
  `reason=duplicate`. The differential oracle in strict mode is stage 11's oracle plus the
  duplicate rule.
- Duplicate `Host` → 400 `reason=duplicate`. 24 headers → 200, 25 → 400 `reason=too_many_headers`.
- A config fuzz test runs 10,000 generated configs; every one either loads with hard-reject intact
  or is refused. There is a malformed-config table (each refusal case above), and startup exits
  non-zero for each.
- Log limit: 1,000 requests carrying 100 distinct unknown names give a bounded line count (the
  test states the bound), and memory stays bounded.
- 100,000 differential cases against httparse, 0 mismatches, with the oracle extended for the
  policy (strict mode must match stage 11's oracle exactly).
- `./nix-build.sh check` and `image-static` pass. formal-coverage is re-measured as in stage 10,
  and the number is reported.
- Live on **8095**, container `header-policy`, hardened as 8091 is (read-only rootfs, cap-drop ALL,
  user 65532, `/tls` read-only). Mount `config/header-policy.conf` read-only at `/config` with
  `SCRATCH_HEADER_POLICY` set. Stage-10 openssl/curl/idle checks pass. Don't touch
  `scratch-server`, `nix-scratch` (8089), `parser-http` (8090), `parser-https` (8091). Dudley
  redeploys onto 8091.

## Out of scope
Any `use` header, request bodies, keep-alive, HTTP/2, new routes and TLS changes.
