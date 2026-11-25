#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
root="${BERTIE_HAX_TOOLCHAIN:?set BERTIE_HAX_TOOLCHAIN to the retained extraction toolchain}"
case_name="${1:-current}"
case "$case_name" in
current) out="$PWD/target/stage19e-bertie"; rev=e6ea574f698aadf8c56174b878f945c251a7fccb ;;
baseline) out="$PWD/target/stage19e-baseline"; rev=03374d49c29f5dad55252584fa362ade0ea3010e ;;
*) echo 'usage: stage19e-extract.sh current|baseline' >&2; exit 2 ;;
esac
trap 'if [ -f "$out/extract/extract.log" ]; then cp "$out/extract/extract.log" "$PWD/logs/stage19e-extract-$case_name.log"; fi' EXIT
mkdir -p "$out/ws/bertie" "$out/extract"
git -C "${BERTIE_SOURCE:?set BERTIE_SOURCE to the historical Bertie checkout}" archive "$rev" src tests test_certs benches proofs Cargo.toml | tar -x -C "$out/ws/bertie"
awk '/^\[workspace\]/{skip=1} /^\[lints/{skip=0} !skip' "$out/ws/bertie/Cargo.toml" > "$out/ws/bertie/Cargo.toml.tmp"
mv "$out/ws/bertie/Cargo.toml.tmp" "$out/ws/bertie/Cargo.toml"
printf '[workspace]\nmembers = ["bertie"]\nresolver = "2"\n' > "$out/ws/Cargo.toml"
printf '%s\n' "$rev" > "$out/BERTIE-REV"
# Clear generated source; no stale extraction can be counted as a new result.
rm -f "$out/ws/bertie/proofs/fstar/extraction/"*.fst "$out/ws/bertie/proofs/fstar/extraction/"*.fsti
# Retain the pinned original model libraries, without edits.
cp -R target/stage19c-bertie/libs "$out/"
docker run --rm --cpus 6 --memory 5g -v "$root:/tools" -v "$out:$out" \
  -e HOME=/tools/home -e CARGO_HOME=/tools/cargo-home -w "$out/ws" \
  fc-verify-hax:v17 bash -c '
set -eu
export PATH="/tools/hax:/tools/rust-install/bin:$PATH"
export LD_LIBRARY_PATH=/tools/rust-install/lib:/tools/rust-install/lib/rustlib/aarch64-unknown-linux-gnu/lib
export RUSTUP_HOME=/tools/rustup
export RUSTUP_TOOLCHAIN=nightly-2025-11-08
cargo hax --version
cargo hax -C -p bertie -p rand_core@0.9.5 --no-default-features --features defensive,std \; into \
 -i "-**::non_hax::** -bertie::stream::**" fstar \
 --interfaces "+** +!bertie::tls13crypto::** +!bertie::tls13utils::** +!bertie::tls13cert::**"
' > "$out/extract/extract.log" 2>&1
