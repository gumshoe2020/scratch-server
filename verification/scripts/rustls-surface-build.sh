#!/bin/sh
# Stage 17 M1 measurement build: the flake's server-static (aarch64-unknown-linux-musl,
# release profile) with only debug info kept. Same flake, lock, toolchain and RUSTFLAGS;
# overrideAttrs adds CARGO_PROFILE_RELEASE_DEBUG=2, STRIP=false and dontStrip, and keeps
# rustc's dep-info files. Uses the same pinned image and Nix store volume as nix-build.sh,
# in an ephemeral --rm container. Output: target/stage17-musl/{scratch-server,deps/*.d}.
set -eu
cd "$(dirname "$0")/../.."
mkdir -p logs target/stage17-musl
log="$PWD/logs/stage17-musl-build.log"
if ! docker run --rm --name "stage17-measure-$$" -i \
  -v "$PWD":/src -w /src \
  -v scratch-server-nix-store:/nix \
  -e GIT_CONFIG_COUNT=1 -e GIT_CONFIG_KEY_0=safe.directory -e GIT_CONFIG_VALUE_0=/src \
  nixos/nix@sha256:f159a58f390677f41e8e900a74b6e2f9c7f27b364122ae6acf7a652ce6c551c0 sh -s > "$log" 2>&1 <<'INNER'
set -eu
out=$(nix --extra-experimental-features 'nix-command flakes' build --impure --no-link --print-out-paths \
  --expr '
    let f = builtins.getFlake (toString /src);
        s = f.packages.${builtins.currentSystem}.server-static;
    in s.overrideAttrs (old: {
      pname = "scratch-server-stage17-debuginfo";
      CARGO_PROFILE_RELEASE_DEBUG = "2";
      CARGO_PROFILE_RELEASE_STRIP = "false";
      dontStrip = true;
      doCheck = false;
      postInstall = (old.postInstall or "") + "\n"
        + "mkdir -p $out/deps\n"
        + "find . -path \"*/release/deps/*.d\" -exec cp {} $out/deps/ \\;\n"
        + "echo \"$NIX_BUILD_TOP\" > $out/build-top\n";
    })')
rm -rf /src/target/stage17-musl/deps /src/target/stage17-musl/scratch-server
mkdir -p /src/target/stage17-musl/deps
cp "$out/bin/scratch-server" /src/target/stage17-musl/scratch-server
cp "$out"/deps/*.d /src/target/stage17-musl/deps/
cp "$out/build-top" /src/target/stage17-musl/build-top
chmod -R u+w /src/target/stage17-musl
echo "out=$out"
INNER
then
  tail -n 30 "$log"
  exit 1
fi
tail -n 5 "$log"
