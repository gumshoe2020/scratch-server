{ pkgs, formal-coverage, verus, receiptArgs ? {}, withReceipt ? true }:
let
  inherit (pkgs) lib;
  system = pkgs.stdenv.hostPlatform.system;
  manifest = builtins.fromTOML (builtins.readFile ../Cargo.toml);
  lock = builtins.fromTOML (builtins.readFile ../Cargo.lock);
  dep = manifest.dependencies.bertie-tls;
  locked = lib.findSingle (p: p.name == "bertie-tls") null null lock.package;
  rev = lib.last (lib.splitString "#" locked.source);
  src = builtins.fetchGit { url = dep.git; inherit rev; submodules = true; };
  receipt = formal-coverage.lib.${system}.coverageTotals ({
    inherit src rev;
    package = locked.name;
    url = dep.git;
    features = dep.features ++ lib.optional (dep.default-features or true) "default";
    cargoHash = "sha256-a1xBSkY5iNzsWEbWjqyz6AVjXeRivbHKQvShzcP2zzo=";
    verify = "python3 scripts/verify-fstar.py";
    reportArgs = [ "--fstar-log" "target/ci-logs/fstar/fstar.log" ];
  } // receiptArgs);
  source = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [ ../Cargo.toml ../Cargo.lock ../src ../crates ../tests ../verification ../formal-coverage.toml ];
  };
  proofLogs = pkgs.runCommand "scratch-server-coverage-verus-logs" {
    nativeBuildInputs = [ pkgs.python3 ];
    VERUS = "${verus}/bin/verus";
  } ''
    cp -R ${source} source
    chmod -R u+w source
    cd source
    ./verification/verify.sh
    mv logs "$out"
  '';
  vendor = pkgs.rustPlatform.importCargoLock {
    lockFile = ../Cargo.lock;
    allowBuiltinFetchGit = true;
  };
in pkgs.runCommand "scratch-server-coverage" {
  nativeBuildInputs = [ formal-coverage.packages.${system}.default pkgs.python3 pkgs.cargo pkgs.rustc pkgs.stdenv.cc pkgs.git ];
  passthru = { inherit receipt; };
  CARGO_NET_OFFLINE = "true";
} ''
  cp -R ${source} source
  chmod -R u+w source
  cd source
  export HOME=$TMPDIR/home CARGO_HOME=$TMPDIR/cargo-home
  mkdir -p "$HOME" "$CARGO_HOME" "$out"
  substitute ${vendor}/.cargo/config.toml "$CARGO_HOME/config.toml" --replace-fail '"cargo-vendor-dir"' '"${vendor}"'
  ${lib.optionalString withReceipt ''test "$(cat ${receipt}/verify.exit)" = 0''}
  cp -R ${proofLogs} logs
  formal-coverage report . \
    --verus-log logs/verus-http-subset-lib.log \
    --verus-log logs/verus-transport-core-lib.log \
    --verus-log logs/verus-transport-core-aead.log \
    --verus-log logs/verus-transport-core-hello.log \
    --check-target-dir "$TMPDIR/check-target" \
    ${lib.optionalString withReceipt "--dependency-totals ${receipt}/totals.json"} \
    --json "$out/report.json" --markdown "$out/report.md"
''
