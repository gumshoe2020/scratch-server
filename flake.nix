{
  description = "scratch-server: verified HTTP subset in a static shell-free Nix image";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }:
    let
      systems = [ "aarch64-linux" "x86_64-linux" ];
      forAll = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      source = pkgs: pkgs.lib.fileset.toSource {
        root = ./.;
        fileset = pkgs.lib.fileset.unions [
          ./Cargo.toml ./Cargo.lock ./src ./crates ./tests ./config ./deny.toml
        ];
      };
      proofSource = pkgs: pkgs.lib.fileset.toSource {
        root = ./.;
        fileset = pkgs.lib.fileset.unions [ ./crates ./verify.sh ];
      };
      build = pkgs: rustPlatform: rustPlatform.buildRustPackage {
        pname = "scratch-server";
        version = "0.1.0";
        src = source pkgs;
        cargoLock.lockFile = ./Cargo.lock;
        # Network integration test uses loopback only; all tests run in the sandbox.
        doCheck = true;
      };
    in {
      packages = forAll (pkgs:
        let
          mkImage = tag: server: pkgs.dockerTools.buildLayeredImage {
            name = "proven-io";
            # Build-time gates: checks are dependencies, never image contents.
            extraCommands = ''
              test -e ${self.checks.${pkgs.stdenv.hostPlatform.system}.proofs}
              test -e ${self.checks.${pkgs.stdenv.hostPlatform.system}.dependencies}/passed
            '';
            inherit tag;
            config = {
              Entrypoint = [ "${server}/bin/scratch-server" ];
              User = "65532:65532";
              ExposedPorts."8080/tcp" = { };
              Healthcheck = {
                Test = [ "CMD" "${server}/bin/scratch-server" "--healthcheck" ];
                Interval = 10000000000;
                Timeout = 3000000000;
              };
            };
          };
        in rec {
          server = build pkgs pkgs.rustPlatform;
          server-static = build pkgs pkgs.pkgsStatic.rustPlatform;
          image = mkImage "glibc" server;
          image-static = mkImage "static" server-static;
          verus = import ./nix/verus.nix { inherit pkgs; };
          default = image-static;
        });

      checks = forAll (pkgs:
        let
          server = self.packages.${pkgs.stdenv.hostPlatform.system}.server;
          verus = self.packages.${pkgs.stdenv.hostPlatform.system}.verus;
          advisoryArchive = pkgs.fetchFromGitHub {
            owner = "RustSec";
            repo = "advisory-db";
            rev = "99670cfc0bc574433034b75876a4b791ccaa2efb";
            hash = "sha256-y2Y5TPInS05cpQWXH6wwfuJv8i6HG2kXMxOKcQko22w=";
          };
        in {
          tests = server;
          proofs = pkgs.runCommand "verified-boundary-proofs" { nativeBuildInputs = [ verus ]; } ''
            cp -R ${proofSource pkgs} source
            chmod -R u+w source
            cd source
            export VERUS=${verus}/bin/verus
            ./verify.sh
            touch "$out"
          '';
          dependencies = server.overrideAttrs (old: {
            pname = "verified-boundary-dependencies";
            nativeBuildInputs = (old.nativeBuildInputs or []) ++ [ pkgs.cargo-deny pkgs.libfaketime ];
            buildPhase = ''
              runHook preBuild
              # cargo-deny 0.20.2 hashes the lowercase URL using xxhash64 seed
              # 0xca80de71. Snapshot freshness is the upstream commit timestamp,
              # evaluated against the pinned snapshot clock, not the rebuild date.
              db=.advisory-db/advisory-db-3157b0e258782691
              mkdir -p "$db/.git"
              cp -R ${advisoryArchive}/. "$db/"
              chmod -R u+w "$db"
              touch -d '2026-10-27 10:02:42 UTC' "$db/.git/FETCH_HEAD"
              export CARGO_HOME="$TMPDIR/deny-cargo-home"
              mkdir -p "$CARGO_HOME"
              # Freeze the check clock to the reviewed snapshot date. The database
              # archive and timestamp must be refreshed together.
              # Keep FETCH_HEAD metadata real while faking only the check clock.
              export LD_PRELOAD=${pkgs.libfaketime}/lib/libfaketime.so.1
              export FAKETIME='2026-10-27 10:02:42' DONT_FAKE_MONOTONIC=1 NO_FAKE_STAT=1 TZ=UTC
              if [ "$(date -u '+%Y-%m-%d %H:%M:%S' 2>/dev/null)" != "$FAKETIME" ]; then
                echo 'advisory clock assertion failed: expected frozen UTC timestamp' >&2
                exit 1
              fi
              cargo deny --offline --all-features check
              runHook postBuild
            '';
            doCheck = false;
            installPhase = ''mkdir -p "$out"; touch "$out/passed"'';
          });
        });
    };
}
