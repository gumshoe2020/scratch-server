#!/usr/bin/env python3
"""Stage 17 M2: line coverage of rustls, rustls-webpki and rustls-pki-types reached by the
fuzz corpora, compared with the M1 in-binary line sets.

Usage: STAGE17_TLS_DIR=... scripts/stage17-coverage.py [--skip-build]
Needs the nightly toolchain on PATH (cargo fuzz coverage, llvm-tools) and
logs/stage17-m1-inbinary.json from scripts/rustls-surface.py.
Writes logs/stage17-coverage-<target>.lcov, logs/stage17-coverage.json and prints a summary.
"""
import argparse, glob, json, os, shutil, subprocess, sys, tempfile
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TARGETS = ["server_first_flight", "server_after_handshake", "proven_shell_pipeline"]
sys.path.insert(0, str(ROOT / "scripts"))
surface = __import__("rustls-surface")


def llvm_tool(name):
    sysroot = subprocess.run(["rustc", "--print", "sysroot"], capture_output=True, text=True,
                             check=True).stdout.strip()
    found = glob.glob(f"{sysroot}/lib/rustlib/*/bin/{name}")
    if not found:
        sys.exit(f"{name} missing: rustup component add llvm-tools-preview --toolchain nightly")
    return found[0]


def covered_lines(lcov, dirnames):
    """crate dir -> file relative to the crate root -> lines with a nonzero count."""
    out = {d: defaultdict(set) for d in dirnames}
    current = None
    for line in lcov.read_text().splitlines():
        if line.startswith("SF:"):
            path = line[3:]
            current = next(((d, path.split(f"/{d}/", 1)[1]) for d in dirnames if f"/{d}/" in path), None)
        elif line.startswith("DA:") and current:
            number, count = line[3:].split(",")[:2]
            if int(count) > 0:
                out[current[0]][current[1]].add(int(number))
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--skip-build", action="store_true", help="reuse existing profdata")
    args = ap.parse_args()
    inbinary = json.loads((ROOT / "logs" / "stage17-m1-inbinary.json").read_text())
    dirnames = list(inbinary)
    llvm_cov = llvm_tool("llvm-cov")
    per_target = {}
    for target in TARGETS:
        if not args.skip_build:
            with open(ROOT / "logs" / f"stage17-coverage-{target}.log", "w") as log:
                subprocess.run(["cargo", "fuzz", "coverage", "-O", target, f"fuzz/corpus/{target}",
                                f"fuzz/seeds/{target}"], cwd=ROOT, stdout=log, stderr=log, check=True)
        # cargo-fuzz puts coverage builds under the enclosing target dir.
        binary = (glob.glob(str(ROOT / f"target/*/coverage/*/release/{target}")) +
                  glob.glob(str(ROOT / f"fuzz/target/*/coverage/*/release/{target}")))[0]
        # cargo-fuzz's batched replay logs fewer inputs than the corpus holds (2217 of 2418
        # for server_first_flight), so replay every corpus and seed file in one -runs=0 pass.
        raw = Path(tempfile.mkdtemp(prefix=f"stage17-cov-{target}-"))
        with open(ROOT / "logs" / f"stage17-coverage-replay-{target}.log", "w") as log:
            subprocess.run([binary, "-runs=0", "-close_fd_mask=1", "-rss_limit_mb=0",
                            f"fuzz/corpus/{target}", f"fuzz/seeds/{target}"], cwd=ROOT, stdout=log,
                           stderr=log, check=True, env=dict(os.environ, LLVM_PROFILE_FILE=f"{raw}/%p.profraw"))
        profile = ROOT / "logs" / f"stage17-coverage-{target}.profdata"
        subprocess.run([llvm_tool("llvm-profdata"), "merge", "-sparse", *glob.glob(f"{raw}/*.profraw"),
                        "-o", str(profile)], check=True)
        shutil.rmtree(raw)
        lcov = ROOT / "logs" / f"stage17-coverage-{target}.lcov"
        with open(lcov, "w") as out:
            subprocess.run([llvm_cov, "export", "-format=lcov", f"-instr-profile={profile}", binary],
                           stdout=out, check=True)
        per_target[target] = covered_lines(lcov, dirnames)

    def count(sets, d, restrict=None):
        total = 0
        for f, lines in sets[d].items():
            total += len(lines & set(restrict.get(f, []))) if restrict is not None else len(lines)
        return total

    union = {d: defaultdict(set) for d in dirnames}
    for sets in per_target.values():
        for d in dirnames:
            for f, lines in sets[d].items():
                union[d][f] |= lines
    report = {"crates": {}, "targets": {}, "attacker_facing": []}
    for d in dirnames:
        m = sum(len(v) for v in inbinary[d].values())
        report["crates"][d] = {"inbinary_lines": m, "reached_inbinary": count(union, d, inbinary[d]),
                               "reached_any": count(union, d)}
    for target, sets in per_target.items():
        report["targets"][target] = {d: count(sets, d, inbinary[d]) for d in dirnames}
    package_dir = {p: d for p in surface.CRATES for d in dirnames if d.startswith(p + "-") and
                   d[len(p) + 1:][0].isdigit()}
    # The harnesses' own rustls client (handshakes in two targets) runs client-role code;
    # report the comparison without those files too, so harness coverage is not counted
    # as server attack-surface coverage.
    client_role = defaultdict(set)
    for name, specs in surface.UNUSED[:2]:
        for spec in specs:
            package, rel = spec.split("/", 1)
            client_role[package_dir[package]].add(rel)
    for d in dirnames:
        server_side = {f: v for f, v in inbinary[d].items() if f not in client_role[d]}
        report["crates"][d]["inbinary_lines_server_side"] = sum(len(v) for v in server_side.values())
        report["crates"][d]["reached_server_side"] = count(union, d, server_side)
        report["crates"][d]["reached_server_side_first_flight"] = count(
            per_target["server_first_flight"], d, server_side)
    for name, specs in surface.ATTACKER_FACING:
        reached = total = 0
        for spec in specs:
            package, rel = spec.split("/", 1)
            d = package_dir[package]
            lines = set(inbinary[d].get(rel, []))
            total += len(lines)
            reached += len(lines & union[d].get(rel, set()))
        report["attacker_facing"].append({"module": name, "inbinary": total, "reached": reached})
    (ROOT / "logs" / "stage17-coverage.json").write_text(json.dumps(report, indent=1) + "\n")
    print("| crate | in-binary lines | reached by fuzzing (in-binary) | % | reached (any line) |")
    print("|---|---|---|---|---|")
    for d, c in report["crates"].items():
        pct = 100 * c["reached_inbinary"] / c["inbinary_lines"] if c["inbinary_lines"] else 0
        print(f"| {d} | {c['inbinary_lines']} | {c['reached_inbinary']} | {pct:.1f} | {c['reached_any']} |")
    print("\n| crate | in-binary lines outside client-role files | reached (all targets) | reached (server_first_flight, no harness client) |")
    print("|---|---|---|---|")
    for d, c in report["crates"].items():
        print(f"| {d} | {c['inbinary_lines_server_side']} | {c['reached_server_side']} | "
              f"{c['reached_server_side_first_flight']} |")
    print("\n| target | " + " | ".join(dirnames) + " |")
    print("|---|" + "---|" * len(dirnames))
    for target, row in report["targets"].items():
        print(f"| {target} | " + " | ".join(str(row[d]) for d in dirnames) + " |")
    print("\n| attacker-facing module | in-binary | reached |")
    print("|---|---|---|")
    for row in report["attacker_facing"]:
        print(f"| {row['module']} | {row['inbinary']} | {row['reached']} |")


if __name__ == "__main__":
    main()
