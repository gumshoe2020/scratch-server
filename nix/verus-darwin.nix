{ pkgs }:
let
  version = "0.2026.10.26.d17fbd0";
  release = pkgs.fetchurl {
    url = "https://github.com/verus-lang/verus/releases/download/release/${version}/verus-${version}-arm64-macos.zip";
    sha256 = "5e62be446a5da782d64caeee41bb1490c8aa3c65c0afe380d1c1dcbe207571ff";
  };
  component = name: hash: pkgs.fetchurl {
    url = "https://static.rust-lang.org/dist/${name}-1.99.1-aarch64-apple-darwin.tar.xz";
    inherit hash;
  };
  rustc = component "rustc" "sha256-I5SKCpwWL9KVLe+z+wRixE6Pz9Gkq4onSh4b2yz4EKM=";
  std = component "rust-std" "sha256-/nnWq1oDId4mRjmZ2MESuK0+r7y8+CLWaiMAtF3eEDE=";
in pkgs.runCommand "coverage-verus-darwin-${version}" {
  nativeBuildInputs = [ pkgs.unzip pkgs.makeWrapper ];
} ''
  for archive in ${rustc} ${std}; do
    tar xf "$archive"
  done
  for installer in rust*-1.99.1-*/install.sh; do
    sh "$installer" --prefix="$out/rust" --disable-ldconfig
  done
  unzip -q ${release}
  mv verus-arm64-macos "$out/verus-release"
  makeWrapper "$out/verus-release/verus" "$out/bin/verus" \
    --set VERUS_USE_RUSTUP 0 --prefix PATH : "$out/rust/bin" \
    --prefix DYLD_LIBRARY_PATH : "$out/rust/lib" --add-flags "--sysroot $out/rust"
''
