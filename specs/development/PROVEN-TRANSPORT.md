# PROVEN-TRANSPORT.md: stage 8 (refinement of the HTTPS architecture, port 8091)

Branch `proven-transport`, built on `idle-connections`. Approved by Andrew 2026-10-27 ("Let's
attack this attack surface first": item 2 of ATTACK-SURFACE.md on main). Authoritative.

## Goal
Raise the proven share of first-party security-relevant code (formal-coverage v1 baseline at
stage 7: 17% of 737 first-party SLoC, formal-coverage v1, 2026-10-27; historical) by proving our own connection code. The parser, grammar, 12-name
whitelist and TLS policy do not change.

## Approach: functional core, imperative shell
Split connection handling into:
- **a pure core**, written for Verus in a crate of its own (e.g. `crates/transport-core`): a
  state machine that takes events (`Bytes(&[u8])`, `ReadTimeout`, `Eof`, `IdleExpired`) and
  returns actions (`ReadMore`, `Dispatch(Request)`, `Reject(Reason)`, `CloseSilently`), plus
  response formatting into a byte buffer;
- **a thin shell** (sockets, rustls calls, clock, threads) that only moves bytes and events
  between the OS and the core. Keep it as small as possible; it stays unverified and is listed.

Prove for the core, for all inputs (no bounded checking for these):
1. The buffer never exceeds MAX_REQUEST + 1 bytes; no panic, overflow or out-of-bounds.
2. `Dispatch` happens only when the proven parser accepts the *entire* buffer, at most once per
   connection; after `Dispatch`, `Reject` or `CloseSilently` the machine is terminal.
3. `CloseSilently` happens only when zero request bytes were received; a deadline after at least
   one byte yields `Reject(Slow)`; oversize yields `Reject(Size)`; EOF before a complete request
   yields `Reject(Grammar)`.
4. Every event sequence reaches a terminal action in at most MAX_REQUEST + 2 steps.
5. Response formatting: status line from a fixed set; `Content-Length` equals the body length;
   header bytes contain no CR/LF from any variable input (no header injection); `HEAD` responses
   carry the same headers and no body.

**libcrux ↔ rustls glue (src/tls.rs):** prove what is provable without modelling the crypto:
AEAD nonce construction (IV XOR big-endian sequence number) equals its spec; every slice, length
and tag-size check is in bounds and matches the algorithm's sizes; decrypt-failure maps to an
error, never to plaintext. If a part can't be proven within reason, leave it unverified and say
which part and why in NOTES.md and PROOFS.md. Do not change crypto behavior.

## Measurement
Use formal-coverage (read-only, <workspace>;
build it there with `CARGO_HOME="$PWD/.cargo-home" cargo build`, don't modify that repo) on this
branch with the same scratch-server config it used for the stage-7 baseline
(`examples/scratch-server/`). Record before/after first-party security-relevant SLoC and proven %
in NOTES.md. Target: a substantial increase; report the real number whatever it is.

## Acceptance
- `./verification/verify.sh` covers both the parser and the new proofs and passes (N verified, 0 errors); no
  assume/admit/external_body/external/unsafe anywhere first-party. PROOFS.md lists each new
  property with file:line, and an updated "unverified" list for the shell.
- All existing tests pass unchanged in meaning: Safari fixtures (HTTP and HTTPS), malformed corpus
  400 with 0 handler calls, idle/slow/silent-close tests, 100,000 differential cases. Add tests
  that drive the shell with event sequences matching each proven property.
- `./nix-build.sh check` and `./nix-build.sh image-static` pass; static, shell-free, key-free.
- Live on a test port: container `proven-transport` on **8095** (do not touch 8091), hardened as
  before, TLS cert mounted read-only; stage 5/6/7 openssl and curl checks pass, including the
  3-second delayed request → 200 and 17-second idle → closed with 0 bytes.
- Do not touch containers `scratch-server` (stopped), `nix-scratch` (8089), `parser-http` (8090)
  or `parser-https` (8091). Dudley redeploys onto 8091 after independent acceptance.

Historical first-party figures use package source SLoC and the security boundary
defined by [formal-coverage](https://github.com/gumshoe2020/formal-coverage)'s
scratch-server config. See README for the current v1.4 stage-13 baseline.
