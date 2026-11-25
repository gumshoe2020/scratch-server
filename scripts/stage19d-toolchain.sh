#!/usr/bin/env bash
# Pinned D2 bootstrap. All persistent state is in the authorized toolchain directory.
set -euo pipefail
root="${BERTIE_HAX_TOOLCHAIN:?set BERTIE_HAX_TOOLCHAIN to the retained extraction toolchain}"
mkdir -p "$root/downloads" "$root/hax" "$root/rust" "$root/source" "$root/fstar-bin-2025.10.06"
fetch() {
  local url="$1" name="$2" expected="$3" actual dest="$root/downloads/$2"
  if [ ! -f "$dest" ]; then curl --fail --location --silent --show-error "$url" -o "$dest"; fi
  actual=$(shasum -a 256 "$dest" | cut -d ' ' -f1)
  printf '%s  %s\n' "$actual" "$url" >> "$root/downloads.sha256"
  [ "$actual" = "$expected" ] || { echo "Checksum mismatch: $name" >&2; exit 1; }
}
fetch https://github.com/cryspen/hax/releases/download/cargo-hax-v0.4.2/cargo-hax-aarch64-unknown-linux-gnu.tar.zst hax.tar.zst 6ff3905349417637fc1aac7e88a12467c205903c3ee1215504dd9f705a517db4
fetch https://github.com/FStarLang/FStar/releases/download/v2025.10.06/fstar-v2025.10.06-Linux-x86_64.tar.gz fstar.tar.gz 940c187d7a6dc3e95d17fd5159b5eaec17d62981c7f135a7e6c531ce3b59442d
fetch https://static.rust-lang.org/dist/2025-11-08/rust-nightly-aarch64-unknown-linux-gnu.tar.xz rust.tar.xz 4780ffdde76aaf3e93b6942f994f0f3f9495f6b46623ad7f3de1a767505e58ea
# The original commit, independent of mutable release-tag metadata.
fetch https://github.com/cryspen/hax/archive/5ccf89dd2e996c9f88f2267eaa2a64739c4ca41a.tar.gz hax-pinned-source.tar.gz 682c6caa457a9b2167727b84c91b13aebaf259583fc09a162d0bb9ac42504475
fetch https://static.rust-lang.org/dist/2025-11-08/rustc-dev-nightly-aarch64-unknown-linux-gnu.tar.xz rustc-dev.tar.xz 9acf81a71929b8cdc41e0bf08240de5060ea8e88deb4bdda72c079577db970b8
tar -xf "$root/downloads/hax.tar.zst" -C "$root/hax"
tar -xzf "$root/downloads/hax-pinned-source.tar.gz" -C "$root/source"
tar -xzf "$root/downloads/fstar.tar.gz" -C "$root/fstar-bin-2025.10.06"
docker run --rm --cpus 6 --memory 5g --network none -v "$root:/tools" fc-verify-hax:v17 bash -c '
set -eu
for archive in rust rustc-dev; do tar -xJf /tools/downloads/$archive.tar.xz -C /tools/rust; done
/tools/rust/rust-nightly-aarch64-unknown-linux-gnu/install.sh --prefix=/tools/rust-install --disable-ldconfig
/tools/rust/rustc-dev-nightly-aarch64-unknown-linux-gnu/install.sh --prefix=/tools/rust-install --disable-ldconfig
/tools/hax/cargo-hax --version
'
# The release includes the CLI only. Build its matching frontend and legacy engine
# from the exact source using the commands recorded in BERTIE-DEPS-REPORT.md.
