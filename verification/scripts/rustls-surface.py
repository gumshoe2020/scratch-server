#!/usr/bin/env python3
"""Stage 17 M1: measure rustls, rustls-webpki and rustls-pki-types as we ship them.

Usage: verification/scripts/rustls-surface.py [--json logs/stage17-m1.json] [--lines-out DIR]
Needs target/stage17-musl/ from verification/scripts/rustls-surface-build.sh (the flake's server-static
aarch64-unknown-linux-musl release build with debug info kept), tokei, and LLVM tools.

Crate sources come from the .crate archives in ~/.cargo/registry/cache, checked against the
Cargo.lock checksums and unpacked into a temporary directory. Line counts are tokei "code"
lines (physical, nonblank, noncomment). "In binary" is the set of distinct (file, line) pairs
in the DWARF line tables whose sequences lie in executable sections; it is an upper bound on
reachable code (the linker keeps what it cannot prove unused; inlining moves lines).
"""
import argparse, glob, hashlib, json, os, re, shutil, subprocess, sys, tarfile, tempfile
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BUILD = ROOT / "target" / "stage17-musl"
BINARY = BUILD / "scratch-server"
LLVM = os.environ.get("LLVM_BIN", "/opt/homebrew/opt/llvm/bin")
# Cargo package name -> rustc crate name (dep-info file prefix).
CRATES = {"rustls": "rustls", "rustls-webpki": "webpki", "rustls-pki-types": "rustls_pki_types"}
# Test- and bench-only files inside src/ (only compiled under cfg(test)/cfg(bench)).
TEST_FILE = re.compile(r"(^|/)(test|tests|test_macros|[^/]*_test|[^/]*_tests|[^/]*bench)\.rs$")

# M1 item 4: rustls modules that parse peer bytes in our server role, before authentication
# of the client (we never authenticate clients; everything a client sends is attacker input).
ATTACKER_FACING = [
    ("record layer", ["rustls/src/record_layer.rs", "rustls/src/crypto/cipher.rs"]),
    ("message framing", ["rustls/src/msgs/deframer/mod.rs", "rustls/src/msgs/deframer/buffers.rs",
                         "rustls/src/msgs/deframer/handshake.rs", "rustls/src/msgs/message/mod.rs",
                         "rustls/src/msgs/message/inbound.rs", "rustls/src/conn.rs"]),
    ("handshake + extension codecs", ["rustls/src/msgs/handshake.rs", "rustls/src/msgs/codec.rs",
                                      "rustls/src/msgs/base.rs", "rustls/src/msgs/enums.rs",
                                      "rustls/src/enums.rs", "rustls/src/msgs/macros.rs"]),
    ("alerts and change-cipher-spec", ["rustls/src/msgs/alert.rs", "rustls/src/msgs/ccs.rs",
                                        "rustls/src/common_state.rs"]),
    ("server handshake state machine, KeyUpdate", ["rustls/src/server/hs.rs", "rustls/src/server/tls13.rs",
                                                    "rustls/src/server/server_conn.rs", "rustls/src/server/common.rs",
                                                    "rustls/src/hash_hs.rs", "rustls/src/tls13/key_schedule.rs",
                                                    "rustls/src/check.rs"]),
    ("SNI name validation", ["rustls-pki-types/src/server_name.rs"]),
]

# M1 item 5: compiled code our configuration can never use (TLS 1.3 only, no
# client auth, no resumption or tickets, no QUIC, no kTLS/unbuffered API, no ECH, no
# certificate compression codecs, no key logging). Report only.
UNUSED = [
    ("NOT unused: client role, used only by --healthcheck against our own endpoint",
     ["rustls/src/client/builder.rs", "rustls/src/client/client_conn.rs",
      "rustls/src/client/common.rs", "rustls/src/client/handy.rs",
      "rustls/src/client/hs.rs", "rustls/src/client/tls13.rs",
      "rustls/src/webpki/server_verifier.rs"]),
    ("ECH / HPKE (client ECH never configured)", ["rustls/src/client/ech.rs", "rustls/src/crypto/hpke.rs"]),
    ("client-certificate verification and CRLs (no client auth, no CRLs)", ["rustls/src/webpki/client_verifier.rs",
                                                           "rustls-webpki/src/crl/mod.rs", "rustls-webpki/src/crl/types.rs"]),
    ("resumption and tickets (disabled: NoServerSessionStorage, 0 tickets)",
     ["rustls/src/msgs/persist.rs", "rustls/src/ticketer.rs", "rustls/src/limited_cache.rs",
      "rustls/src/server/handy.rs"]),
    ("QUIC (quic: None)", ["rustls/src/quic.rs"]),
    ("kTLS / unbuffered API (we use StreamOwned)", ["rustls/src/conn/kernel.rs", "rustls/src/conn/unbuffered.rs"]),
    ("certificate compression (no codec features)", ["rustls/src/compress.rs"]),
    ("FFDHE groups (X25519 only)", ["rustls/src/msgs/ffdhe_groups.rs"]),
    ("key logging (KeyLog never set)", ["rustls/src/key_log.rs", "rustls/src/key_log_file.rs"]),
    ("PEM base64 (startup-only cert/key loading, not peer input)", ["rustls-pki-types/src/pem.rs",
                                                                     "rustls-pki-types/src/base64.rs"]),
]


def run(*args, **kw):
    return subprocess.run(args, check=True, capture_output=True, text=True, **kw).stdout


def lock_versions():
    text = (ROOT / "Cargo.lock").read_text()
    out = {}
    for block in text.split("[[package]]"):
        name = re.search(r'^name = "([^"]+)"', block, re.M)
        if name and name.group(1) in CRATES:
            out[name.group(1)] = (re.search(r'^version = "([^"]+)"', block, re.M).group(1),
                                  re.search(r'^checksum = "([^"]+)"', block, re.M).group(1))
    return out


def unpack(work, package, version, checksum):
    archive = glob.glob(os.path.expanduser(f"~/.cargo/registry/cache/*/{package}-{version}.crate"))
    if not archive:
        sys.exit(f"missing {package}-{version}.crate; run cargo fetch")
    data = Path(archive[0]).read_bytes()
    if hashlib.sha256(data).hexdigest() != checksum:
        sys.exit(f"{package}-{version}.crate does not match Cargo.lock checksum")
    with tarfile.open(archive[0]) as tar:
        tar.extractall(work, filter="data")
    return Path(work) / f"{package}-{version}"


def tokei_code(src):
    """Map path relative to the crate root -> tokei code lines, for every .rs under src/."""
    report = json.loads(run("tokei", "--output", "json", "--files", "--types", "Rust", str(src / "src")))
    out = {}
    for entry in report.get("Rust", {}).get("reports", []):
        out[os.path.relpath(entry["name"], src)] = entry["stats"]["code"]
    return out


def dep_info_files(crate, dirname):
    files = glob.glob(str(BUILD / "deps" / f"{crate}-*.d"))
    if len(files) != 1:
        sys.exit(f"expected one dep-info file for {crate}, found {files}")
    first = Path(files[0]).read_text().splitlines()[0]
    out = set()
    for path in first.split(": ", 1)[1].split():
        if f"/{dirname}/" in path and path.endswith(".rs"):
            out.add(path.split(f"/{dirname}/", 1)[1])
    return out


def exec_ranges():
    ranges = []
    for line in run(f"{LLVM}/llvm-readelf", "-SW", str(BINARY)).splitlines():
        m = re.match(r"\s*\[\s*\d+\]\s+(\S+)\s+\S+\s+([0-9a-f]+)\s+[0-9a-f]+\s+([0-9a-f]+)\s+\S+\s+(\S+)", line)
        if m and "X" in m.group(4):
            start = int(m.group(2), 16)
            ranges.append((start, start + int(m.group(3), 16)))
    return ranges


def comp_dirs():
    """DWARF 4 line tables name directories relative to their CU's DW_AT_comp_dir."""
    out, stmt = {}, None
    for line in run(f"{LLVM}/llvm-dwarfdump", "--debug-info", "--recurse-depth=0", str(BINARY)).splitlines():
        m = re.search(r"DW_AT_stmt_list\s+\((0x[0-9a-f]+)\)", line)
        if m:
            stmt = int(m.group(1), 16)
        m = re.search(r'DW_AT_comp_dir\s+\("(.*)"\)', line)
        if m and stmt is not None:
            out[stmt] = m.group(1)
            stmt = None
    return out


def dwarf_lines(dirnames, ranges):
    """(crate dir, file relative to crate root) -> set of lines with code in executable sections."""
    cus = comp_dirs()
    proc = subprocess.Popen([f"{LLVM}/llvm-dwarfdump", "--debug-line", str(BINARY)],
                            stdout=subprocess.PIPE, text=True, bufsize=1 << 20)
    hits = defaultdict(set)
    dirs, files, seq = {}, {}, []
    in_exec = lambda a: any(lo <= a < hi for lo, hi in ranges)
    row = re.compile(r"^0x([0-9a-f]+)\s+(\d+)\s+\d+\s+(\d+)\s+\d+\s+\d+\s+\d+\s*(.*)$")
    for line in proc.stdout:
        m = re.match(r"debug_line\[(0x[0-9a-f]+)\]", line)
        if m:
            comp = cus[int(m.group(1), 16)]
            dirs, files, seq = {0: comp}, {}, []
            continue
        m = re.match(r'include_directories\[\s*(\d+)\] = "(.*)"', line)
        if m:
            d = m.group(2)
            dirs[int(m.group(1))] = d if d.startswith("/") else f"{comp}/{d}"
            continue
        m = re.match(r'file_names\[\s*(\d+)\]:', line)
        if m:
            current = int(m.group(1))
            continue
        m = re.match(r'\s+name: "(.*)"', line)
        if m:
            files[current] = [m.group(1), 0]
            continue
        m = re.match(r'\s+dir_index: (\d+)', line)
        if m:
            files[current][1] = int(m.group(1))
            continue
        m = row.match(line)
        if not m:
            continue
        addr, ln, fidx, flags = int(m.group(1), 16), int(m.group(2)), int(m.group(3)), m.group(4)
        if "end_sequence" in flags:
            if seq and in_exec(seq[0][0]):
                for _, l, f in seq:
                    if l and f in files:
                        name, d = files[f]
                        path = name if name.startswith("/") else f"{dirs.get(d, '')}/{name}"
                        for dirname in dirnames:
                            if f"/{dirname}/" in path:
                                hits[(dirname, path.split(f"/{dirname}/", 1)[1])].add(l)
            seq = []
        else:
            seq.append((addr, ln, fidx))
    proc.wait()
    return hits


def function_owners(dirnames, ranges):
    """Distinct function symbols, attributed by the source file of their outermost frame."""
    syms = {}
    for line in run(f"{LLVM}/llvm-nm", "--defined-only", "-n", str(BINARY)).splitlines():
        parts = line.split(None, 2)
        if len(parts) == 3 and parts[1] in "tTwW":
            addr = int(parts[0], 16)
            if any(lo <= addr < hi for lo, hi in ranges):
                syms.setdefault(addr, parts[2])
    query = "".join(f"0x{a:x}\n" for a in syms)
    out = subprocess.run([f"{LLVM}/llvm-symbolizer", "--obj", str(BINARY), "--inlines",
                          "--output-style=JSON"], input=query, capture_output=True, text=True, check=True).stdout
    owners = defaultdict(set)
    for line in out.splitlines():
        record = json.loads(line)
        frames = record.get("Symbol") or []
        if not frames:
            continue
        path = frames[-1].get("FileName", "")
        addr = int(record["Address"], 16)
        for dirname in dirnames:
            if f"/{dirname}/" in path:
                owners[dirname].add(syms[addr])
    return owners


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--json", type=Path, default=ROOT / "logs" / "stage17-m1.json")
    ap.add_argument("--lines-out", type=Path, default=ROOT / "logs" / "stage17-m1-inbinary.json",
                    help="in-binary (file, line) sets, for the fuzz coverage comparison")
    args = ap.parse_args()
    if not BINARY.exists():
        sys.exit("run verification/scripts/rustls-surface-build.sh first")
    versions = lock_versions()
    work = tempfile.mkdtemp(prefix="stage17-crates-")
    try:
        dirnames, results, code = {}, {}, {}
        for package, crate in CRATES.items():
            version, checksum = versions[package]
            dirname = f"{package}-{version}"
            dirnames[package] = dirname
            src = unpack(work, package, version, checksum)
            code[package] = tokei_code(src)
            compiled = dep_info_files(crate, dirname)
            missing = compiled - set(code[package])
            if missing:
                sys.exit(f"dep-info files absent from crate source: {sorted(missing)}")
            total_files = [f for f in code[package] if not TEST_FILE.search(f)]
            results[package] = {
                "version": version,
                "total_files": len(total_files),
                "total_lines": sum(code[package][f] for f in total_files),
                "compiled_files": len(compiled),
                "compiled_lines": sum(code[package][f] for f in compiled),
                "compiled": sorted(compiled),
            }
        ranges = exec_ranges()
        hits = dwarf_lines(list(dirnames.values()), ranges)
        owners = function_owners(list(dirnames.values()), ranges)
        inbinary = {}
        for package, dirname in dirnames.items():
            per_file = {f: sorted(lines) for (d, f), lines in hits.items() if d == dirname}
            stray = set(per_file) - set(results[package]["compiled"])
            if stray:
                sys.exit(f"{package}: in-binary files not in dep-info: {sorted(stray)}")
            inbinary[package] = per_file
            results[package]["inbinary_files"] = len(per_file)
            results[package]["inbinary_lines"] = sum(len(v) for v in per_file.values())
            results[package]["functions"] = len(owners[dirname])

        short = {"rustls": "rustls", "rustls-webpki": "rustls-webpki", "rustls-pki-types": "rustls-pki-types"}
        def lookup(spec):
            package, rel = spec.split("/", 1)
            return package, rel
        def row(spec):
            package, rel = lookup(spec)
            return {"file": spec, "compiled": rel in results[package]["compiled"],
                    "code_lines": code[package].get(rel, 0),
                    "inbinary_lines": len(inbinary[package].get(rel, []))}
        attacker = [{"module": name, "files": [row(s) for s in specs]} for name, specs in ATTACKER_FACING]
        unused = [{"use": name, "files": [row(s) for s in specs]} for name, specs in UNUSED]
        features = run("cargo", "tree", "--locked", "--offline", "-e", "features", "-i", "rustls",
                       "--target", "aarch64-unknown-linux-musl", "--prefix", "none", cwd=ROOT)
        rustls_features = sorted(set(re.findall(r'rustls feature "([^"]+)"', features)))
        report = {
            "binary": str(BINARY.relative_to(ROOT)),
            "exec_sections": [[hex(a), hex(b)] for a, b in ranges],
            "counter": run("tokei", "--version").strip(),
            "crates": {p: {k: v for k, v in r.items() if k != "compiled"} for p, r in results.items()},
            "attacker_facing": attacker,
            "unused_compiled": unused,
            "rustls_features_enabled": rustls_features,
            "log_crate_compiled": bool(glob.glob(str(BUILD / "deps" / "log-*.d"))),
            "tls12_files_compiled": sorted(f for f in results["rustls"]["compiled"] if "tls12" in f),
        }
        args.json.parent.mkdir(parents=True, exist_ok=True)
        args.json.write_text(json.dumps(report, indent=1) + "\n")
        args.lines_out.write_text(json.dumps({dirnames[p]: v for p, v in inbinary.items()}) + "\n")
        print_report(report)
    finally:
        shutil.rmtree(work, ignore_errors=True)


def print_report(r):
    print(f"counter: {r['counter']}; binary: {r['binary']}")
    print("| crate | version | total lines (files) | compiled lines (files) | in-binary lines (files) | functions |")
    print("|---|---|---|---|---|---|")
    for p, c in r["crates"].items():
        print(f"| {p} | {c['version']} | {c['total_lines']} ({c['total_files']}) | {c['compiled_lines']} "
              f"({c['compiled_files']}) | {c['inbinary_lines']} ({c['inbinary_files']}) | {c['functions']} |")
    print("\n| attacker-facing module | files | in-binary lines |")
    print("|---|---|---|")
    total = 0
    for m in r["attacker_facing"]:
        n = sum(f["inbinary_lines"] for f in m["files"])
        total += n
        files = ", ".join(f"{f['file'].split('/src/', 1)[1]} {f['inbinary_lines']}" for f in m["files"])
        print(f"| {m['module']} | {files} | {n} |")
    print(f"| **total** | | {total} |")
    print("\n| unused compiled code | compiled code lines | in-binary lines |")
    print("|---|---|---|")
    for u in r["unused_compiled"]:
        comp = sum(f["code_lines"] for f in u["files"] if f["compiled"])
        n = sum(f["inbinary_lines"] for f in u["files"])
        print(f"| {u['use']} | {comp} | {n} |")
    print(f"\nrustls features enabled: {', '.join(r['rustls_features_enabled'])}; "
          f"log crate compiled: {r['log_crate_compiled']}; tls12 files compiled: {r['tls12_files_compiled'] or 'none'}")


if __name__ == "__main__":
    main()
