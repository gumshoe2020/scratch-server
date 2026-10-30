#!/bin/sh
# All Linux/Nix work uses this entry point and the existing cache volume.
# Build/check logs are retained without streaming compiler output.
set -eu
cd "$(dirname "$0")"
target="${1:-image-static}"
case "$target" in image-static|image|server|server-static|check|versions|proof-log|test-log|deny-log|log) ;; *) echo 'unknown Nix target' >&2; exit 2 ;; esac
mkdir -p logs
log="$PWD/logs/nix-$target.log"
if ! docker run --rm --name "release-nix-$$" -i \
  -v "$PWD":/src -w /src \
  -v scratch-server-nix-store:/nix \
  -e GIT_CONFIG_COUNT=1 -e GIT_CONFIG_KEY_0=safe.directory -e GIT_CONFIG_VALUE_0=/src \
  nixos/nix@sha256:f159a58f390677f41e8e900a74b6e2f9c7f27b364122ae6acf7a652ce6c551c0 sh -s -- "$target" "${2:-}" > "$log" 2>&1 <<'INNER'
set -eu
case "$1" in
  log)
    case "$2" in /nix/store/*.drv) ;; *) echo 'expected Nix derivation path' >&2; exit 2 ;; esac
    nix --extra-experimental-features 'nix-command flakes' log "$2" ;;
  check) nix --extra-experimental-features 'nix-command flakes' flake check --no-update-lock-file ;;
  proof-log|test-log|deny-log)
    system=$(nix --extra-experimental-features 'nix-command flakes' eval --raw --impure --expr builtins.currentSystem)
    case "$1" in proof-log) check=proofs ;; test-log) check=tests ;; deny-log) check=dependencies ;; esac
    nix --extra-experimental-features 'nix-command flakes' log ".#checks.$system.$check" ;;
  versions)
    nix --extra-experimental-features 'nix-command flakes' eval --json --impure --expr '
      let p = (builtins.getFlake (toString ./.)).inputs.nixpkgs.legacyPackages.${builtins.currentSystem};
      in { rustc = p.rustc.version; cargo-deny = p.cargo-deny.version;
           cargo-fuzz = p.cargo-fuzz.version; verus = p ? verus; kani = p ? kani; }' ;;
  image*)
    nix --extra-experimental-features 'nix-command flakes' build ".#$1" --no-update-lock-file
    cp -L result "/src/$1.tar.gz" ;;
  *) nix --extra-experimental-features 'nix-command flakes' build ".#$1" --no-update-lock-file ;;
esac
INNER
then
  tail -n 30 "$log"
  exit 1
fi
case "$target" in
  image*) docker load < "$target.tar.gz" >> "$log" 2>&1 ;;
esac
tail -n 30 "$log"
