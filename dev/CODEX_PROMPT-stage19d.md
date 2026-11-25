You are finishing stage 19 of scratch-server (`bertie-boundary`). Working directory:
~/src/scratch-server (branch `bertie-boundary`).

DECISION (Andrew, 2026-11-21): we no longer track upstream Cryspen Bertie. The Bertie at
~/src/bertie-deps (branch `deps/libcrux-advisories`, head 03374d4) is now OUR
maintained TLS library. Behaviour changes inside it are allowed where this prompt says so. This
replaces 19c's "no behaviour change" fence for Bertie only. scratch-server's fences are unchanged.

Read first: this prompt; dev/CODEX_PROMPT-stage19.md, -19b.md and -19c.md (their non-negotiables
still apply unless changed here); dev/BERTIE-DEPS-REPORT.md; the spec's latest Results section;
the tails of dev/PROGRESS.md and dev/NOTES.md; ~/src/bertie-der/scripts/ and
scripts/stage19c-bertie-docker.sh (read-only reference for how the hax/F* checks were run).

## D1: drop the obsolete Kyber draft; clear the KEM advisories (Bertie)
1. Work only in ~/src/bertie-deps on `deps/libcrux-advisories`. Commit with that
   repo's configured identity. No push. Do not touch other branches or worktrees.
2. Remove X25519Kyber768Draft00 (a pre-standard draft group) from Bertie completely: enum
   variant, named constants, identifier/string conversions, tests, benches, docs. Raise
   libcrux-kem to the smallest release that clears RUSTSEC-2026-0330/-0331, so the transitive
   libcrux-sha3 and libcrux-ecdh advisories also clear. Adapt X25519MLKEM768, X25519 and secp256r1
   to the new API with the minimum diff. These groups must keep working: wire identifiers, key
   share sizes and shared secrets must not change (add a test that pins X25519MLKEM768 key share
   and ciphertext lengths, and do a handshake per group).
3. Bertie gates: workspace tests, the handshake tests, and the existing negative tests. Record
   exact counts in dev/BERTIE-DEPS-REPORT.md (new section "D1"). Commit.

## D1b: cut Bertie down to what scratch-server uses (the goal: smallest trusted surface)
The point of scratch-server is a server whose only attack surface is our own code and its pinned
dependencies, with as much as possible proven. Tonight's blocker came from code we never use (the
Kyber draft). Now that Bertie is ours, remove what scratch-server cannot reach:
1. Keep exactly scratch-server's configuration: TLS 1.3 server side, TLS_CHACHA20_POLY1305_SHA256,
   groups X25519MLKEM768, X25519 and secp256r1, signature ecdsa_secp256r1_sha256, and whatever the
   ClientHello parser must still *recognise* in order to reject other offers safely.
2. Remove unused cipher suites, signature schemes (RSA, Ed25519 and so on), PSK/resumption if
   present and unused, and the crates only they need (e.g. libcrux-rsa, libcrux-ed25519). Removing
   an algorithm must make offers of it fail cleanly, never panic. Add negative tests for that.
3. Bertie's handshake tests need a client. Keep the client code only behind a non-default
   `client` (or `test-client`) feature that scratch-server's production build never enables, and
   prove that with `cargo tree -e features` on the production target.
4. Do not remove or weaken anything the D2 proofs cover. If a removal would touch proven code,
   remove only whole unreachable items and record which ones.
5. Measure before and after: Bertie in-binary lines (stage 17 method), the number of crates in the
   production dependency tree, and the cargo-deny result. Record in BERTIE-DEPS-REPORT.md "D1b".
   Commit in Bertie.

## D2: proof toolchain and Bertie proofs
The earlier hax/F* toolchain cache (<workspace>/projects/formal-coverage/
runs/v1.7/bertie) was deleted when code moved out of the private workspace on 2026-11-17. Rebuild it at
~/src/bertie-toolchain (new directory; nowhere else) with the same pins the bertie-der
scripts use: rust nightly-2025-11-08, hax 0.4.2, F* fstar-bin-2025.10.06 (see bertie-der
scripts/env.sh and docker.sh). The local images fc-verify-hax:v17 and fc-verify-fstar-amd64:v17
may already contain what you need; use them read-only, do not retag or delete them. Log every
download with its URL and checksum. Then run hax extraction and the F* check of Bertie.Der,
Bertie.Tls13formats and every module whose extracted F* changed because of D1. Zero errors; no
assume, admit, lax or new patches. If a check fails because of D1, record exactly where and stop
that item. Do not weaken it. Record the results in dev/BERTIE-DEPS-REPORT.md "D2". Commit any
script changes in scratch-server (not in Bertie).

## D3: scratch-server pin and gates
1. Move the pin to the D1 rev (same file:// URL, update Cargo.lock minimally).
2. cargo-deny: for proc-macro-error2 (RUSTSEC-2026-0173), first confirm with all-target,
   all-feature `cargo tree -i` that it is only a build-time proc-macro dependency, not linked into
   the server binary. If so, add one narrow ignore with a `reason` like the existing two. If not,
   do not add it and record why. No other ignores and no snapshot or clock change.
3. Add a short "Dependency (updated 2026-11-21)" note to the top of the spec: Bertie is now
   maintained by us, from cryspen/bertie ef65f76 plus our fixes; list the commits.
4. Rerun the native gates: verify.sh (counts may only go up from 211/347/9, hello 30), 20 mutants,
   `cargo test --locked --workspace --no-fail-fast`, the production cargo tree rustls check, the
   10,000-handshake loop, and cargo-deny. Commit.

## D4: release and live (only if the Nix cargo-deny gate passes)
Same as 19c's B3: Nix check and image-static, the hardened `bertie-boundary` container on 8099
(left running only if acceptance passes), a temporary `proven-io-ref` on 127.0.0.1:8098 from
`proven-io:static` for the 18 byte-identical requests (removed afterwards), the interop matrix
with certificate verification on, and the stage 17 in-binary audit of the shipped image. If any
gate fails, do not build or run an ungated image; record why.

## Records
Spec: add "Results — 2026-11-21 (our Bertie)" and mark the 19c results superseded. Add a
"Stage 19 — our Bertie" subsection to dev/PROGRESS.md with D1-D4 and Acceptance, and keep
timestamped dev/NOTES.md entries. scratch-server commits after D2, D3, D4 and final, as Dudley.
No pushes and no GitHub actions of any kind. Noisy output to logs/stage19d-*.log; read only tails.

Finish with a short summary: per-milestone result, dependency versions before/after, Bertie
test and proof results, toolchain provenance, cargo-deny result and exceptions, proof counts,
mutants, tests, handshake loop, Nix/image, container status, byte-identical result, interop table,
in-binary lines and dependency-crate count before/after D1b, and anything left for Andrew.
