#!/usr/bin/env bash
# One strict F* invocation per module, matching bertie-der's original method.
set -euo pipefail
cd "$(dirname "$0")/.."
root="${BERTIE_HAX_TOOLCHAIN:?set BERTIE_HAX_TOOLCHAIN to the retained extraction toolchain}"
out="$PWD/target/stage19e-bertie"
module="${1:?module basename required}"
case "$module" in Bertie.*.fst|Bertie.*.fsti) ;; *) exit 2 ;; esac
mkdir -p "$out/tc"
trap 'if [ -f "$out/tc/$module.out" ]; then cp "$out/tc/$module.out" "$PWD/logs/stage19e-proof-$module.log"; fi' EXIT
docker run --rm --cpus 4 --memory 5g --platform linux/amd64 --network none \
  -v "$root:/tools:ro" -v "$out:$out" -w "$out" \
  fc-verify-fstar-amd64:v17 bash -c '
set -u
F=/tools/fstar-bin-2025.10.06/fstar
export PATH="$F/bin:$F/lib/fstar/z3-4.13.3/bin:$PATH"
start=$(date +%s)
timeout 1500 fstar.exe --include libs/rust_primitives --include libs/core \
  --include libs/hax-lib --include libs/libcrux-models \
  --include ws/bertie/proofs/fstar/extraction \
  "ws/bertie/proofs/fstar/extraction/$1" > "tc/$1.body" 2>&1
rc=$?
{ cat "tc/$1.body"; echo "exit=$rc"; echo "$(( $(date +%s) - start ))s"; } > "tc/$1.out"
rm -f "tc/$1.body"
exit "$rc"
' stage19e "$module"
