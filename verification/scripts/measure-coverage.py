#!/usr/bin/env python3
"""Run formal-coverage v1 once per supported Verus root and union proven units.

v1 accepts one complete-file receipt, so no combined verifier log is invented.
All reports use the unchanged stage-7 config. This script only merges statuses
by exact unit identity from independently accepted, source-identical reports.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PARSER = argparse.ArgumentParser(description=__doc__)
PARSER.add_argument('formal_coverage', type=Path, help='path to a formal-coverage checkout')
ARGS = PARSER.parse_args()
TOOL = ARGS.formal_coverage.resolve() / 'target/debug/formal-coverage'
CONFIG = ARGS.formal_coverage.resolve() / 'examples/scratch-server/formal-coverage.toml'
if not TOOL.is_file() or not os.access(TOOL, os.X_OK) or not CONFIG.is_file():
    PARSER.error('formal-coverage checkout must contain an executable target/debug/formal-coverage and examples/scratch-server/formal-coverage.toml')
VERUS = ROOT / ".tools/verus-arm64-macos/verus"
ROOTS = ["crates/http-subset/src/lib.rs", "crates/transport-core/src/lib.rs", "crates/transport-core/src/aead.rs"]
ENV = dict(os.environ, CARGO_HOME=str(ROOT / ".tools/cargo"), RUSTUP_HOME=str(ROOT / ".tools/rustup"))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    files = {str(p.relative_to(ROOT)): sha(p) for d in ["src", "crates", "tests"] for p in (ROOT / d).rglob("*.rs")}
    version = subprocess.check_output([str(VERUS), "--version"], env=ENV, text=True).strip()
    reports = []
    for i, source in enumerate(ROOTS):
        log = ROOT / f"logs/stage13-coverage-verus-{i}.log"
        command = [str(VERUS), "--crate-type", "lib", source]
        with log.open("wb") as out:
            subprocess.run(command, cwd=ROOT, env=ENV, stdout=out, stderr=subprocess.STDOUT, check=True)
        receipt = dict(tool="verus", tool_version=version, command=command, exit_code=0,
                       files=files, log_sha256=sha(log), harnesses=[], covered_units={})
        Path(str(log) + ".evidence.json").write_text(json.dumps(receipt, indent=2) + "\n")
        report = ROOT / f"logs/stage13-coverage-after-{i}.json"
        with (ROOT / f"logs/stage13-coverage-after-{i}.log").open("wb") as out:
            subprocess.run([str(TOOL), "report", str(ROOT), "--config", str(CONFIG), "--verus-log", str(log),
                            "--json", str(report), "--markdown", str(report.with_suffix(".md"))],
                           cwd=ROOT, env=ENV, stdout=out, stderr=subprocess.STDOUT, check=True)
        reports.append(json.loads(report.read_text()))
        assert any("accepted caller-supplied run receipt" in w for w in reports[-1]["warnings"])
    assert all(sha(ROOT / path) == digest for path, digest in files.items())
    merged = json.loads(json.dumps(reports[0]))
    for report in reports[1:]:
        assert [(u["id"], u["file"], u["line"], u["sloc"] if "sloc" in u else u.get("sloc_weight")) for u in report["units"]] == [
            (u["id"], u["file"], u["line"], u["sloc"] if "sloc" in u else u.get("sloc_weight")) for u in merged["units"]]
        for dest, src in zip(merged["units"], report["units"]):
            if src["status"] == "proven":
                dest["status"], dest["proof_scope"] = src["status"], src["proof_scope"]
    # Emit units only: the tool's precomputed aggregate fields must not be mistaken
    # for merged aggregates. The compact summary is computed from these unit rows.
    output = {"method": "union of three source-identical formal-coverage v1 reports", "roots": ROOTS,
              "config_sha256": sha(CONFIG), "units": merged["units"]}
    (ROOT / "logs/stage13-coverage-union.json").write_text(json.dumps(output, indent=2) + "\n")
    baseline = json.loads((ROOT / "logs/stage8-coverage-before.json").read_text())
    def counts(units):
        security = [u for u in units if u["first_party"] and u["security"]]
        measured = sum(u["sloc"] for u in security)
        proven = sum(u["sloc"] for u in security if u["status"] == "proven")
        return dict(measured=measured, proven=proven, percent=round(100 * proven / measured, 2))
    summary = dict(before=counts(baseline["units"]), after_union=counts(output["units"]),
                   individual_reports=[counts(r["units"]) for r in reports],
                   limitation="v1 accepts one receipt; union is explicitly derived, not a single CLI report; dependency sizes unknown")
    (ROOT / "logs/stage13-coverage-summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
