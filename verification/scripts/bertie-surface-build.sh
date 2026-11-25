#!/bin/sh
# Stage 19 audit build (not a release/image acceptance build): the flake's server-static (aarch64-unknown-linux-musl,
# release profile) with only debug info kept. Same flake, lock, toolchain and RUSTFLAGS;
# overrideAttrs adds CARGO_PROFILE_RELEASE_DEBUG=2, STRIP=false and dontStrip, and keeps
# rustc's dep-info files. Uses the same pinned image and Nix store volume as nix-build.sh,
# in an ephemeral --rm container. Output: target/stage19-musl/{scratch-server,deps/*.d}.
set -eu
cd "$(dirname "$0")/../.."
repo_git_common=$(git rev-parse --path-format=absolute --git-common-dir)
mkdir -p logs target/stage19-musl
log="${NIX_LOG_DIR:-$PWD/logs}/${NIX_LOG_PREFIX:-stage19}-musl-build.log"
if ! docker run --rm --name "stage19-measure-$$" -i \
  -v "$PWD":/src -v "$PWD:$PWD" -w /src \
  -v "$repo_git_common:$repo_git_common:ro" \
  -v scratch-server-nix-store:/nix \
  -e GIT_CONFIG_COUNT=1 -e GIT_CONFIG_KEY_0=safe.directory -e GIT_CONFIG_VALUE_0=/src \
  nixos/nix@sha256:f159a58f390677f41e8e900a74b6e2f9c7f27b364122ae6acf7a652ce6c551c0 sh -s > "$log" 2>&1 <<'INNER'
set -eu
out=$(nix --extra-experimental-features 'nix-command flakes' build --impure --no-link --print-out-paths \
  --expr '
    let f = builtins.getFlake "git+file:///src";
        s = f.packages.${builtins.currentSystem}.server-static;
    in s.overrideAttrs (old: {
      pname = "scratch-server-stage19-debuginfo";
      CARGO_PROFILE_RELEASE_DEBUG = "2";
      CARGO_PROFILE_RELEASE_STRIP = "false";
      dontStrip = true;
      doCheck = false;
      postInstall = (old.postInstall or "") + "\n"
        + "mkdir -p $out/deps\n"
        + "find . -path \"*/release/deps/*.d\" -exec cp {} $out/deps/ \\;\n"
        + "echo \"$NIX_BUILD_TOP\" > $out/build-top\n";
    })')
rm -rf /src/target/stage19-musl/deps /src/target/stage19-musl/scratch-server
mkdir -p /src/target/stage19-musl/deps
cp "$out/bin/scratch-server" /src/target/stage19-musl/scratch-server
cp "$out"/deps/*.d /src/target/stage19-musl/deps/
cp "$out/build-top" /src/target/stage19-musl/build-top
chmod -R u+w /src/target/stage19-musl
echo "out=$out"
INNER
then
  tail -n 30 "$log"
  exit 1
fi
tail -n 5 "$log"
