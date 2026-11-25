You are finishing stage 19 of scratch-server (`bertie-boundary`). Working directory:
~/src/scratch-server. Read this prompt, dev/CODEX_PROMPT-stage19d.md (its non-negotiables
and fences still apply unless changed here), dev/BERTIE-DEPS-REPORT.md (D1-D4), and the tails of
dev/PROGRESS.md and dev/NOTES.md. Bertie (ours) is at ~/src/bertie-deps, branch
`deps/libcrux-advisories`, head e1af495. The proof toolchain is at ~/src/bertie-toolchain.

19d results: Nix cargo-deny now PASSES. Two blockers remain. Andrew decided both (2026-11-21).

## E1: repair the Bertie proof break (Bertie repo)
`Bertie.Tls13crypto.fsti:9`: namespace `Bertie.Tls13crypto.Non_hax` cannot be found. It was
introduced by 19d's RNG compatibility glue (e1af495) for the new libcrux random API. Fix it so the
extracted F* typechecks again: e.g. move the glue into a module hax doesn't extract and that the
.fsti doesn't reference, or give it a proper hax interface. Rules: no admit/assume/lax, no new
patches to proofs, no removal of proven items. If the glue ends up outside extraction, list it as
trusted (with the reason) in Bertie's TRUSTED notes and in dev/BERTIE-DEPS-REPORT.md. Then rerun the
full D2 set: Bertie.Der, Bertie.Tls13formats and every module whose extracted F* changed in
D1/D1b/E1. Zero errors. Commit in Bertie.

## E2: replace the authenticated healthcheck (scratch-server), Andrew chose option (b)
The production healthcheck used Bertie's client, which is off in production now. Replace it, in
scratch-server only, with a minimal probe that uses no TLS client library:
connect to the server on loopback, send one fixed, valid TLS 1.3 ClientHello (a constant byte array
that offers our suite, groups and signature scheme, with key shares), read the first record, and
report healthy only if it is a handshake record (content type 0x16, legacy version 0x0303,
plausible length) whose first handshake message is ServerHello (type 0x02). Hard timeouts,
bounded read, and a clean close. Then the process exits 0 or 1. No crypto is needed. Keep it small,
list it in verification/TRUSTED-BASE.md, and add tests: a healthy server passes, and a closed port,
a non-TLS responder, an alert and a timeout each fail. The Bertie `client` feature stays off in
production. Prove that with `cargo tree -e features` on the production target.

## E3: gates, release and live
Pin scratch-server to the E1 rev, then run all of 19d's D3 and D4 unchanged: verify.sh (counts may
only go up from 211/347/9, hello 30), 20 mutants, `cargo test --locked --workspace
--no-fail-fast`, the production rustls check, the 10,000-handshake loop, Nix cargo-deny, Nix check,
image-static, the hardened `bertie-boundary` container on 8099 (left running only if acceptance passes),
a temporary `proven-io-ref` for the 18 byte-identical requests (removed afterwards), the interop
matrix (certificate verification on), and the stage 17 in-binary audit of the shipped image
(Bertie lines and production package count, versus 1,200 lines and 112 packages before 19d).

## E4: clean up README.md (Andrew: "We should clean up the readme too")
README.md is canon: it describes what scratch-server is now, not how we got here. Rewrite it so
it reads cleanly for an outside reader:
- What it is and the goal (a web server whose only attack surface is our own code and its pinned
  dependencies, with as much as possible proven), how to build, run and verify it, current limits,
  and current coverage numbers, each labelled with its source and date.
- TLS: now provided by bertie-tls (our maintained fork of Cryspen's Bertie,
  github.com/gumshoe2020/bertie-tls), with a short credit to Cryspen.
- Move blocker narratives, stage history and "the former third exception" style notes OUT of
  the README into the stage docs, dev/NOTES.md or EVOLUTION.md links. Keep the facts accurate. Don't
  drop limits a user needs to know (TLS feature gaps, platforms checked, advisory snapshot).
- Keep it short. Plain language, no marketing words.
Only README.md and links that need fixing change in E4.

## Records
Spec: "Results — 2026-11-21 (E)" and mark 19d's results superseded. PROGRESS: a "Stage 19 — E"
subsection. Commits: Bertie E1; scratch-server after E2, E3 and E4/final, as Dudley. No pushes.
Noisy output to logs/stage19e-*.log; read only tails.

Finish with a short summary: per-item result, proof results, healthcheck design and tests,
proof counts, mutants, tests, handshake loop, cargo-deny, Nix/image, container status,
byte-identical result, interop table, Bertie lines and package count before/after, and anything left.
