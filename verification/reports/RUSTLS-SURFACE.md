# RUSTLS-SURFACE.md: stage 17 (measure and fuzz rustls as we use it; no port)

Branch `rustls-surface`, built on `proven-io` (`b700156`). Requested by Andrew 2026-10-31 ("Let's
attempt 3"). Authoritative for this stage.

## Why
rustls is the largest unproven piece in front of our proven code. ATTACK-SURFACE.md calls it
"~42K lines", which is the whole crate. We compile one configuration: TLS 1.3 server only, one
cipher suite (ChaCha20-Poly1305), one key exchange (X25519), ECDSA P-256 certificates, our custom
libcrux provider, `default-features = false, features = ["std", "custom-provider"]`. This stage
replaces "~42K" with measured numbers for that configuration, and fuzzes exactly that configuration.
It changes **no runtime behaviour** and deploys nothing.

## M1: measure what we actually ship
Measure for `rustls`, `rustls-webpki` and `rustls-pki-types`, at the versions pinned in Cargo.lock,
for the shipped target (aarch64-unknown-linux-musl, release profile as the Nix image builds it; a
native aarch64 build with the same profile and features is acceptable if the musl build is
impractical, but log that choice).
1. **Crate total:** physical nonblank, noncomment Rust lines in each crate's `src/` (tests and
   benches excluded). Use one counting tool (tokei or the same counter formal-coverage uses) and name
   it.
2. **Compiled:** the same count restricted to source files the compiler actually reads for our
   feature set (from rustc's dep-info `.d` files for that crate).
3. **In the binary:** distinct (file, line) pairs from these crates that have machine code in the
   final shipped binary, taken from DWARF line tables (`llvm-dwarfdump --debug-line` or `addr2line`
   over the text section) of a build with debug info but otherwise identical codegen settings. Also
   report the number of distinct functions from these crates in the symbol table. State plainly that
   this is an upper bound on reachable code: dead-code elimination keeps code the linker cannot prove
   unused, and inlining moves lines around.
4. **Attacker-reachable before authentication:** list the rustls modules that parse bytes from the
   peer in our server role (record layer, message framing, handshake message codecs, extension
   codecs, alerts, key update) and give the in-binary line count per module from item 3.
5. **Unused features:** report any rustls feature or code path still compiled that our configuration
   can never use (for example TLS 1.2 types, client-only code, logging). Report only; do not change
   Cargo.toml.

## M2: fuzz our exact configuration
Add a `fuzz/` cargo-fuzz project (libFuzzer; a nightly toolchain via rustup is allowed, installed
for this user only) with these targets, each built from the production `ServerConfig` construction
code path (our provider, one suite, one group, the same certificate type) rather than a re-written
copy. Use a throwaway ECDSA P-256 test key generated into `$TMPDIR` or `fuzz/` (never the deployed
key under the TLS directory), and never commit generated keys:
1. `server_first_flight`: arbitrary bytes fed to a fresh `ServerConnection` via `read_tls` in
   fuzzer-chosen chunk sizes, then `process_new_packets`, until error or input exhaustion. This
   covers the ClientHello path.
2. `server_after_handshake`: complete a real handshake with an in-process rustls client using our
   provider, then feed arbitrary bytes as further client records (encrypted with the client's real
   traffic keys where the fuzzer chooses "valid framing", raw otherwise). This covers record
   decryption, unpadding, alerts, KeyUpdate and post-handshake messages.
3. `proven_shell_pipeline`: arbitrary bytes through our real `serve_*` read path (the TLS shell and
   the proven parser together), over an in-memory socket, so we fuzz the glue as well as rustls.

Seed corpora: ClientHellos captured from `openssl s_client`, `curl` and the rustls client against a
local instance (store only the hello bytes, never keys or secrets). Run each target for **30
minutes** wall clock with at most 8 jobs in total across targets, so the other Codex run on this Mac
keeps some CPU. Then measure line coverage of the three crates (llvm-cov on the fuzz corpus) and
report it against M1 item 3: "N of M in-binary rustls lines reached by fuzzing".

Any crash, panic, hang over 1 s, or memory growth beyond 256 MB is a finding. **Stop fuzzing that
target, minimise the input, commit it under `fuzz/findings/` with a NOTES.md entry, and do not fix
it.** Then continue with the other targets.

## Acceptance
- M1 numbers reproduce from committed scripts (`verification/scripts/rustls-surface.py` or similar) with one
  command each, and the results table is written into a "Results" section at the end of this file.
- `cargo fuzz build` succeeds for all three targets; the fuzz run logs and coverage summary exist
  under `logs/stage17-*` (not committed; `logs/` is ignored).
- Existing checks still pass unchanged: `./verification/verify.sh` (211/317/9, zero errors) and
  `cargo test --workspace` (52 passed). The fuzz crate is outside the workspace members, or is
  excluded, so it cannot change those results.
- No change to anything under `src/`, `crates/`, `config/`, `flake.nix`, `flake.lock`, `Cargo.toml`
  or `Cargo.lock` except, if strictly needed, a `#[doc(hidden)] pub` re-export or a `pub(crate)` →
  `pub` change so the fuzz targets can reach the production config builder. Each such change is
  listed in NOTES.md with the reason.

## Out of scope
Fixing anything rustls does; changing features, versions or the provider; deployment or containers;
formal-coverage; EVOLUTION.md and ATTACK-SURFACE.md (Dudley updates them after acceptance).

## Results (2026-10-31)

### M1: what we ship
Measured on the shipped target: the flake's `server-static` (aarch64-unknown-linux-musl,
release profile, opt-level z, fat LTO, one codegen unit, panic=abort) rebuilt with debug info
kept and stripping off (`verification/scripts/rustls-surface-build.sh`; ephemeral `docker run --rm`
Nix build, about 3 minutes). Its `.text` has the same size (706072 bytes) and address as the
binary in image-static.tar.gz; it is not byte-identical, but 99.67% of instruction words
match once PC-relative immediates are masked (same code, different function layout).
Numbers: `python3 verification/scripts/rustls-surface.py` (reruns give identical output). Counter: tokei
15.0.0 code lines (physical, nonblank, noncomment). Crate total excludes test/bench-only files
under src/; inline `#[cfg(test)]` modules stay in it.

| crate | total lines (files) | compiled lines (files) | in-binary lines (files) | functions in binary |
|---|---|---|---|---|
| rustls 0.23.46 | 33055 (101) | 24891 (78) | 4152 (61) | 841 |
| rustls-webpki 0.103.16 | 6542 (18) | 6105 (16) | 693 (14) | 123 |
| rustls-pki-types 1.15.1 | 2659 (5) | 2659 (5) | 250 (5) | 39 |
| **all three** | **42256** | **33655** | **5095** | **1003** |

"In binary" is the number of distinct (file, line) pairs with machine code in the DWARF line
tables. It is an **upper bound on reachable code**: the linker keeps whatever it cannot prove
unused (trait-object vtables keep whole impls alive), and inlining moves lines around. It
counts the lines where code lands, not the lines of every function that was inlined.
Functions are defined text symbols whose outermost source frame lies in the crate.

Attacker-facing modules (peer bytes parsed in our server role; no client authentication, so
everything a client sends is pre-authentication). In-binary lines:

| module | files (in-binary lines) | in-binary lines |
|---|---|---|
| record layer | record_layer.rs 40, crypto/cipher.rs 18 | 58 |
| message framing | msgs/deframer/{mod 15, buffers 26, handshake 62}, msgs/message/{mod 35, inbound 7}, conn.rs 152 | 297 |
| handshake and extension codecs | msgs/handshake.rs 580, msgs/codec.rs 99, msgs/base.rs 38, msgs/enums.rs 66, enums.rs 453, msgs/macros.rs 58 | 1294 |
| alerts and change-cipher-spec | msgs/alert.rs 3, msgs/ccs.rs 1, common_state.rs 172 | 176 |
| server handshake state machine and KeyUpdate | server/hs.rs 130, server/tls13.rs 351, server/server_conn.rs 24, hash_hs.rs 49, tls13/key_schedule.rs 239, check.rs 17 | 810 |
| SNI name validation (pki-types) | server_name.rs 106 | 106 |
| **total** | | **2741** |

Extension codecs are inside msgs/handshake.rs and counted there. rustls-webpki parses no peer
bytes in our server role (it parses only our own certificate at startup, and the server's
certificate in `--healthcheck`).

Unused features and code paths (report only; nothing changed). Enabled rustls features:
`std`, `custom-provider`. No `log` crate is compiled (logging off) and no TLS 1.2 files are
compiled. Still compiled, though our configuration never uses it:

| code path | compiled lines | in-binary lines |
|---|---|---|
| ECH client and HPKE (client/ech.rs, crypto/hpke.rs) | 1075 | 149 |
| client-certificate verification and CRLs (no client auth, no CRLs) | 1464 | 105 |
| resumption and tickets (NoServerSessionStorage, 0 tickets) | 1230 | 99 |
| QUIC (quic.rs) | 696 | 0 |
| kTLS and the unbuffered API | 501 | 0 |
| certificate compression (no codec features) | 506 | 22 |
| FFDHE groups (X25519 only) | 289 | 1 |
| key logging (KeyLog never set) | 181 | 4 |

The rustls **client role is in the binary but is used**: 4021 compiled and 751 in-binary
lines (client/*, webpki/server_verifier.rs), reached only by `--healthcheck`, which connects
to our own endpoint. It is not on the accept path. PEM/base64 parsing in pki-types (1012
compiled, 120 in-binary lines) runs only at startup on our own files.

### M2: fuzzing our exact configuration
Three cargo-fuzz targets (`fuzz/`, own workspace and lock file; same rustls pins) build their
server config through the production `tls::load_config` with a throwaway P-256 key in
$TMPDIR. Seeds: ClientHellos captured from openssl s_client, Homebrew curl (OpenSSL), system
curl and our rustls client against a local instance (`scripts/stage17-capture-hellos.py`),
plus structured post-handshake and HTTP seeds (`fuzz/examples/seed_corpus.rs`). Runs:
`scripts/stage17-fuzz.sh <target>`, all three at once with 2 forked workers each (6 in
total), 30 minutes each, `-timeout=1 -rss_limit_mb=256`, crashes/timeouts/OOMs fatal.

| target | executions | corpus (files, size) | fuzzing time | crashes / timeouts / OOMs |
|---|---|---|---|---|
| server_first_flight | 29,848,149 | 2410, 9.4 MB | 1815 s (1817 s wall) | 0 / 0 / 0 |
| server_after_handshake | 595,019 | 476, 2.9 MB | 1817 s (1819 s wall) | 0 / 0 / 0 |
| proven_shell_pipeline | 671,188 | 1780, 7.0 MB | 1820 s (1822 s wall) | 0 / 0 / 0 |

**No findings.** `fuzz/findings/` is not created. (A 20-second trial run before the real runs
reported an OOM. It was ASan's 256 MB free-quarantine counting toward RSS, not growth in the
target; see NOTES.md.)

Coverage (`python3 verification/scripts/stage17-coverage.py`: llvm-cov over every corpus and seed file,
intersected with the M1 in-binary line sets):

| crate | in-binary lines | reached by fuzzing | % |
|---|---|---|---|
| rustls | 4152 | 1972 | 47.5 |
| rustls-webpki | 693 | 271 | 39.1 |
| rustls-pki-types | 250 | 164 | 65.6 |

**1972 of 4152 in-binary rustls lines reached by fuzzing.** Two of the harnesses run a rustls
client in-process, so part of this is harness code. Leaving out client-role files gives 1639
of 3252 server-side in-binary rustls lines. `server_first_flight` alone, which has no client,
reaches 1252 of them. Every webpki line reached comes from loading our certificate and from the
harness client verifying it, not from server parsing of peer bytes.

| attacker-facing module | in-binary lines | reached |
|---|---|---|
| record layer | 58 | 41 |
| message framing | 297 | 259 |
| handshake and extension codecs | 1294 | 498 |
| alerts and change-cipher-spec | 176 | 141 |
| server handshake state machine and KeyUpdate | 810 | 440 |
| SNI name validation | 106 | 69 |

Caveat on the codec row: enums.rs and msgs/enums.rs (519 in-binary lines) are almost all
`enum_builder!` invocations. DWARF attributes the expanded code to the invocation lines, but
llvm-cov attributes it to the macro, so 509 of those lines read as unreached regardless. The
reached counts above are therefore a lower bound.

Existing checks are unchanged: `./verification/verify.sh` 211/317/9 verified, 0 errors; `cargo test
--workspace` 52 passed. No file under src/, crates/, config/, flake.nix, flake.lock,
Cargo.toml or Cargo.lock changed.
