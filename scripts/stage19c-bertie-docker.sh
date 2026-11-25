#!/usr/bin/env bash
# Isolated reuse of bertie-der's Docker method; original project remains read-only.
set -eu
cd "$(dirname "$0")/.."
step="$1"; shift
out="$PWD/target/stage19c-bertie"
toolchain="${BERTIE_HAX_TOOLCHAIN:?set BERTIE_HAX_TOOLCHAIN to the retained extraction toolchain}"
[ -d "$toolchain" ] || { echo "Missing existing hax/F* toolchain cache: $toolchain" >&2; exit 1; }
image=fc-verify-hax:v17
platform=()
if [ "$step" = typecheck ]; then
  image=fc-verify-fstar-amd64:v17
  platform=(--platform linux/amd64)
fi
docker run --rm --cpus 6 --memory 5g --network none "${platform[@]}" \
  --mount "type=bind,src=$toolchain,dst=$toolchain,readonly" \
  --mount "type=bind,src=$out,dst=$out" \
  -e V171_OUT="$out" -e V171_TOOLCHAIN="$toolchain" \
  "$image" bash -c 'rm -rf /out && ln -s "$V171_TOOLCHAIN" /out && source "$V171_OUT/container/env.sh" && bash "$V171_OUT/container/container-$0.sh" "$@"' "$step" "$@"
