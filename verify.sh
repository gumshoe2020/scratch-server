#!/bin/sh
# Pinned release and archive digests from GitHub's Verus release API.
# All downloads and Rust toolchain files stay in this repository's ignored .tools.
set -eu
cd "$(dirname "$0")"
version=0.2026.10.26.d17fbd0
case "$(uname -s):$(uname -m)" in
  Darwin:arm64) platform=arm64-macos; digest=5e62be446a5da782d64caeee41bb1490c8aa3c65c0afe380d1c1dcbe207571ff ;;
  Linux:aarch64) platform=arm64-linux; digest=55d0179000c2c78732825d47a8b8fed1aa4afbe6d24896acc6e060eddcd338e4 ;;
  Linux:x86_64) platform=x86-linux; digest=361674d05f0c1b2c9c3f4d5bc56a8fb2ab9241ed0cceca27780668c292bb4940 ;;
  Darwin:x86_64) platform=x86-macos; digest=24a23b0267f7a225c6ece4bc019930ca273bd57207fe0c1dd3cc7dc5726a37bc ;;
  *) echo 'Unsupported Verus release platform' >&2; exit 1 ;;
esac
mkdir -p .tools logs
if [ -z "${VERUS:-}" ]; then
  archive=".tools/verus-$version-$platform.zip"
  if [ ! -f "$archive" ]; then
    curl --fail --location --retry 3 "https://github.com/verus-lang/verus/releases/download/release/$version/verus-$version-$platform.zip" -o "$archive"
  fi
  if command -v sha256sum >/dev/null; then
    actual=$(sha256sum "$archive" | cut -d ' ' -f 1)
  else
    actual=$(shasum -a 256 "$archive" | cut -d ' ' -f 1)
  fi
  [ "$actual" = "$digest" ] || { echo 'Verus archive digest mismatch' >&2; exit 1; }
  if [ ! -x ".tools/verus-$platform/verus" ]; then unzip -q "$archive" -d .tools; fi
  VERUS="$PWD/.tools/verus-$platform/verus"
  export RUSTUP_HOME="$PWD/.tools/rustup" CARGO_HOME="$PWD/.tools/cargo"
  if ! rustup run 1.99.1 rustc --version >/dev/null 2>&1; then
    rustup toolchain install 1.99.1 --profile minimal > logs/rust-toolchain.log 2>&1
  fi
fi
"$VERUS" --version | grep -F "$version" >/dev/null
"$VERUS" --crate-type lib crates/http-subset/src/lib.rs
"$VERUS" --crate-type lib crates/transport-core/src/lib.rs
"$VERUS" --crate-type lib crates/transport-core/src/aead.rs
