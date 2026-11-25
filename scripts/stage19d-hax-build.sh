#!/usr/bin/env bash
# Reproduce the upstream sequential installer; no source patches or image changes.
set -euo pipefail
cd "$(dirname "$0")/.."
root="${BERTIE_HAX_TOOLCHAIN:?set BERTIE_HAX_TOOLCHAIN to the retained extraction toolchain}"
phase="${1:?usage: stage19d-hax-build.sh rust|ocaml}"
case "$phase" in rust|ocaml) ;; *) exit 2 ;; esac
docker run --rm --cpus 6 --memory 5g -v "$root:/tools" \
  -e HOME=/tools/home -e CARGO_HOME=/tools/cargo-home \
  -e CARGO_INSTALL_ROOT=/tools/hax-install -e OPAMROOT=/tools/home/.opam \
  -e OPAMJOBS=1 -w /tools/source/hax-5ccf89dd2e996c9f88f2267eaa2a64739c4ca41a \
  fc-verify-hax:v17 bash -c '
set -eu
export PATH=/tools/hax-install/bin:/tools/rust-install/bin:$PATH
case "$1" in
rust)
  source .utils/install-rust-binaries.sh
  install_rust_binaries
  cp /tools/hax-install/bin/* /tools/hax/
  ;;
ocaml)
  if [ ! -f "$OPAMROOT/config" ]; then opam init --bare --disable-sandboxing -y; fi
  if ! opam switch list --short | grep -qx hax; then
    opam switch create hax ocaml-base-compiler.5.3.0 -y -j1
  fi
  eval "$(opam env --switch=hax)"
  opam install ./engine --deps-only -y -j2
  cd engine
  dune build -j1 @install
  dune install --prefix /tools/hax-install
  cp /tools/hax-install/bin/* /tools/hax/
  ;;
esac
' stage19d "$phase"

if [ "$phase" = rust ]; then
  # Keep the published CLI's exact commit receipt; rebuilt helpers come from that source.
  tar -xf "$root/downloads/hax.tar.zst" -C "$root/hax"
fi
