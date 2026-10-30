# Binary toolchain pinned to the Verus release, not nixpkgs' different Rust version.
# Digests: GitHub release API and static.rust-lang.org .sha256 files (read 2026-10-27).
{ pkgs }:
let
  version = "0.2026.10.26.d17fbd0";
  platform = {
    aarch64-linux = {
      name = "arm64-linux";
      rust = "aarch64-unknown-linux-gnu";
      verusHash = "55d0179000c2c78732825d47a8b8fed1aa4afbe6d24896acc6e060eddcd338e4";
      rustcHash = "acdcdfe09ae1f103aa9a2b7f88903a9588bc2e99288af10e6fbf5514e2e49f25";
      stdHash = "e25668477083d6a9ddb87b9beb9c00742225f38233ccb4bfba3afa7a07427f52";
    };
    x86_64-linux = {
      name = "x86-linux";
      rust = "x86_64-unknown-linux-gnu";
      verusHash = "361674d05f0c1b2c9c3f4d5bc56a8fb2ab9241ed0cceca27780668c292bb4940";
      rustcHash = "2ff05ff9c08a90861e7f6e4065b40a8b87463b4576186fd613871c3e37f2091f";
      stdHash = "109b074225bf5b9ea3820890876bfdf4ed161adbfaf94b2b4b5984c85ee5d958";
    };
  }.${pkgs.stdenv.hostPlatform.system};
  rustcArchive = pkgs.fetchurl {
    url = "https://static.rust-lang.org/dist/rustc-1.99.1-${platform.rust}.tar.xz";
    sha256 = platform.rustcHash;
  };
  stdArchive = pkgs.fetchurl {
    url = "https://static.rust-lang.org/dist/rust-std-1.99.1-${platform.rust}.tar.xz";
    sha256 = platform.stdHash;
  };
  rust = pkgs.stdenv.mkDerivation {
    pname = "verus-rust";
    version = "1.99.1";
    dontUnpack = true;
    nativeBuildInputs = [ pkgs.autoPatchelfHook ];
    buildInputs = [ pkgs.stdenv.cc.cc.lib pkgs.zlib ];
    installPhase = ''
      tar -xf ${rustcArchive}
      sh rustc-1.99.1-${platform.rust}/install.sh --prefix="$out" --disable-ldconfig
      tar -xf ${stdArchive}
      sh rust-std-1.99.1-${platform.rust}/install.sh --prefix="$out" --disable-ldconfig
    '';
  };
in pkgs.stdenv.mkDerivation {
  pname = "verus";
  inherit version;
  src = pkgs.fetchurl {
    url = "https://github.com/verus-lang/verus/releases/download/release/${version}/verus-${version}-${platform.name}.zip";
    sha256 = platform.verusHash;
  };
  nativeBuildInputs = [ pkgs.unzip pkgs.autoPatchelfHook pkgs.makeWrapper ];
  buildInputs = [ rust pkgs.stdenv.cc.cc.lib pkgs.zlib ];
  sourceRoot = "verus-${platform.name}";
  installPhase = ''
    mkdir -p "$out/libexec/verus" "$out/bin"
    cp verus rust_verify z3 version.json vstd.vir \
      libvstd.rlib libverus_builtin.rlib \
      libverus_builtin_macros.so libverus_state_machines_macros.so \
      "$out/libexec/verus/"
    makeWrapper "$out/libexec/verus/verus" "$out/bin/verus" \
      --set VERUS_USE_RUSTUP 0 \
      --prefix PATH : "${rust}/bin" \
      --prefix LD_LIBRARY_PATH : "${rust}/lib" \
      --add-flags "--sysroot ${rust}"
  '';
  # Keep the released proof artifacts; ELF loader/RPATH patching is the only binary change.
  dontStrip = true;
}
