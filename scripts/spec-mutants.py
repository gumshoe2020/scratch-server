#!/usr/bin/env python3
"""Mutate spec and executable together in isolated copies; never count a broken refinement proof.

Logs and JSON receipts stay under logs/stage13-mutants; copies/build artifacts stay
under target. --only permits development reruns; the default always runs all fifteen.
"""
import argparse
import hashlib
import json
import os
import platform
import re
from pathlib import Path
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SOURCE = Path("crates/http-subset/src/lib.rs")
MUTANTS = [
    "transfer-encoding", "expect-guarded", "headers25", "value257", "empty-values",
    "bare-lf", "lowercase-get", "http12", "duplicate-host-only", "unknown-reject",
    "percent-path", "strict-ignores-guarded", "space-token", "trailing-bytes", "cookie-drop",
]


INTENT_TARGETS = dict(zip(MUTANTS, [
    "hard_name_intent", "hard_name_intent", "reject_headers25", "reject_value257",
    "reject_empty_value", "header_framing_intent", "reject_lower_get", "reject_http12",
    "reject_duplicate_connection", "witness_brave_dnt", "reject_percent",
    "strict_checks_guarded", "reject_colon_space", "reject_body", "builtin_policy_intent",
]))


def intent_module(target):
    if target.startswith("witness_"):
        return "intent::" + target.removeprefix("witness_")
    if target.startswith("reject_"):
        return "intent::" + target
    return "intent"


def verifier(directory):
    if os.environ.get("VERUS"):
        return os.environ["VERUS"]
    system = platform.system(), platform.machine()
    suffix = {("Darwin", "arm64"): "arm64-macos", ("Darwin", "x86_64"): "x86-macos",
              ("Linux", "aarch64"): "arm64-linux", ("Linux", "x86_64"): "x86-linux"}[system]
    return str(directory / f".tools/verus-{suffix}/verus")


def replace(text, old, new, expected=None):
    count = text.count(old)
    if not count or (expected is not None and count != expected):
        raise ValueError("mutation anchor count changed")
    return text.replace(old, new)


def mutate(text, name):
    if name in ("transfer-encoding", "expect-guarded"):
        word, before, after = ("Transfer-Encoding", 20, 44) if name == "transfer-encoding" else ("Expect", 23, 28)
        for suffix in ('"@)', '")'):
            text = replace(text, f'b"{word}{suffix} {{ {before} }}', f'b"{word}{suffix} {{ {after} }}', 1)
    elif name == "headers25":
        text = replace(text, "MAX_HEADERS: usize = 24", "MAX_HEADERS: usize = 25", 1)
    elif name == "value257":
        text = replace(text, "MAX_VALUE: usize = 256", "MAX_VALUE: usize = 257", 1)
    elif name == "empty-values":
        text = replace(text, "h.start < h.end", "h.start <= h.end", 2)
        text = replace(text, "end <= start", "end < start", 4)
        text = replace(text, "s[start] == b' '", "(end > start && s[start] == b' ')", 4)
        text = replace(text, "s[end - 1] == b' '", "(end > start && s[end - 1] == b' ')", 4)
    elif name == "bare-lf":
        # Accept an actual bare LF on header lines and the final empty line.
        # Keep the request line unchanged, so offsets/method/path still agree.
        for fn in ("matches", "equal"):
            tag = "@" if fn == "matches" else ""
            old = f'{fn}(s, at, b"\\r\\n"{tag}) && at + 2 == s.len()'
            new = f'({old}) || ({fn}(s, at, b"\\n"{tag}) && at + 1 == s.len())'
            text = replace(text, old, new, 3 if fn == "matches" else 2)
            old = f'!{fn}(s, end, b"\\r\\n"{tag})'
            new = f'!({fn}(s, end, b"\\r\\n"{tag}) || {fn}(s, end, b"\\n"{tag}))'
            text = replace(text, old, new, 2)
        # Replace only recursive header positions (including the legacy-name proof).
        text = replace(text, "header_spec(s, end + 2,", "header_spec(s, end + ending_spec(s, end),", 2)
        text = replace(text, "stage11_header_names(s, end + 2,", "stage11_header_names(s, end + ending_spec(s, end),", 1)
        text = replace(text, "headers(s, end + 2,", "headers(s, end + ending(s, end),", 2)
        helpers = r'''
pub open spec fn ending_spec(s: Seq<u8>, at: int) -> int {
    if matches(s, at, b"\n"@) { 1 } else { 2 }
}
fn ending(s: &[u8], at: usize) -> (n: usize)
    requires at <= s.len(), matches(s@, at as int, b"\n"@) || matches(s@, at as int, b"\r\n"@),
    ensures n == ending_spec(s@, at as int), at + n <= s.len(),
{ if equal(s, at, b"\n") { 1 } else { 2 } }
'''
        text = text.replace("pub open spec fn bounded_stage11", helpers + "\npub open spec fn bounded_stage11", 1)
    elif name == "lowercase-get":
        text = replace(text, 'matches(s, 0, b"GET "@)', 'matches_header(s, 0, b"GET "@)', 2)
        text = replace(text, 'equal(s, 0, b"GET ")', 'equal_header(s, 0, b"GET ")', 2)
    elif name == "http12":
        text = replace(text, 'matches(s, end, b" HTTP/1.1\\r\\n"@)', '(matches(s, end, b" HTTP/1.1\\r\\n"@) || matches(s, end, b" HTTP/1.2\\r\\n"@))', 2)
        text = replace(text, 'equal(s, end, b" HTTP/1.1\\r\\n")', '(equal(s, end, b" HTTP/1.1\\r\\n") || equal(s, end, b" HTTP/1.2\\r\\n"))', 2)
    elif name == "duplicate-host-only":
        text = replace(text, "(h[i].name == 0 || h[i].name == 2)", "h[i].name == 0", 1)
        text = replace(text, "if connection { return false; } connection = true;", "connection = true;", 1)
    elif name in ("unknown-reject", "cookie-drop"):
        before, after = ("unknown_drop: true", "unknown_drop: false") if name == "unknown-reject" else ("guarded_drop: 0", "guarded_drop: 1")
        text = replace(text, before, after, 3)
        if name == "unknown-reject":
            text = replace(text, "!p.unknown_drop == false", "!p.unknown_drop == true", 1)
        else:
            text = replace(text, "p.guarded_drop == 0, p.log_limit == 60", "p.guarded_drop == 1, p.log_limit == 60", 1)
    elif name == "percent-path":
        text = replace(text, "|| c == b'.'", "|| c == b'.' || c == b'%'", 2)
    elif name == "strict-ignores-guarded":
        text = replace(text, "!self.unknown_drop && self.guarded_drop == 0", "!self.unknown_drop", 2)
    elif name == "space-token":
        text = replace(text, "|| c == b'~'", "|| c == b'~' || c == b' '", 2)
    elif name == "trailing-bytes":
        text = replace(text, "at + 2 == s.len()", "at + 2 <= s.len()", 5)
    else:
        raise ValueError("unknown mutant")
    return text


def safe_log_line(line):
    # Keep outcomes and source locations, never request literals, assertion values,
    # proptest counterexample bytes, or panic payloads containing header values.
    return bool(re.match(r"^(?:verification results::|error(?:\[.*?\])?:|note: verifying |running \d+ tests?|test result:|test [A-Za-z0-9_:]+ \.\.\. (?:ok|FAILED|ignored)|\s*--> [^:]+:\d+:\d+)", line)
                or re.match(r"^thread .* panicked at [^\n]+\.rs:\d+", line))


def run(command, directory, log, env):
    with log.open("w") as output:
        process = subprocess.Popen(command, cwd=directory, env=env, stdout=subprocess.PIPE,
                                   stderr=subprocess.STDOUT, text=True, errors="replace")
        for line in process.stdout:
            if safe_log_line(line):
                output.write(line)
        return process.wait()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--only", choices=MUTANTS, action="append")
    parser.add_argument("--refinement-only", action="store_true", help="development: check mutant consistency only")
    args = parser.parse_args()
    selected = args.only or MUTANTS
    logs = ROOT / "logs/stage13-mutants"
    logs.mkdir(parents=True, exist_ok=True)
    (ROOT / "target").mkdir(exist_ok=True)
    baseline_env = dict(os.environ, CARGO_HOME=str(ROOT / ".tools/cargo"),
                        RUSTUP_HOME=str(ROOT / ".tools/rustup"), TMPDIR=str(ROOT / "target"))
    if not args.refinement_only:
        print("baseline: proofs and workspace tests", flush=True)
        if run(["./verify.sh"], ROOT, logs / "baseline-proofs.log", baseline_env):
            raise SystemExit("baseline proofs failed; no mutants counted")
        if run(["cargo", "test", "--workspace"], ROOT, logs / "baseline-tests.log", baseline_env):
            raise SystemExit("baseline tests failed; no mutants counted")
    files = subprocess.check_output(["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=ROOT).decode().split("\0")
    files = [p for p in files if p and not p.endswith((".pem", ".key", ".crt"))]
    original = {p: hashlib.sha256((ROOT / p).read_bytes()).hexdigest() for p in files if p.endswith((".rs", ".http", ".conf"))}
    results = []
    for name in selected:
        print(f"{name}: running", flush=True)
        result = {"mutant": name}
        with tempfile.TemporaryDirectory(prefix="spec-mutant-", dir=ROOT / "target") as temp:
            directory = Path(temp)
            for file in files:
                dest = directory / file
                dest.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(ROOT / file, dest)
            (directory / ".tools").symlink_to(ROOT / ".tools", target_is_directory=True)
            env = dict(os.environ, CARGO_HOME=str(ROOT / ".tools/cargo"), RUSTUP_HOME=str(ROOT / ".tools/rustup"),
                       CARGO_TARGET_DIR=str(ROOT / "target/spec-mutants-build"), TMPDIR=str(directory / "target/tmp"))
            Path(env["TMPDIR"]).mkdir(parents=True)
            source = directory / SOURCE
            mutated = mutate(source.read_text(), name)
            # Stage 1 checks every preexisting proof, excluding the new intent module.
            source.write_text(mutated.replace("mod intent;", "// Intent checked in phase two."))
            result["refinement_exit"] = run(["./verify.sh"], directory, logs / f"{name}-refinement.log", env)
            source.write_text(mutated)
            # Never count a malformed mutant as a kill.
            if result["refinement_exit"]:
                result.update(status="malformed", killed_by=[])
            elif args.refinement_only:
                result.update(status="well-formed", killed_by=[])
            else:
                result["intent_target"] = INTENT_TARGETS[name]
                result["intent_exit"] = run([verifier(directory), "--crate-type", "lib", str(SOURCE), "--verify-only-module", intent_module(INTENT_TARGETS[name]), "--verify-function", INTENT_TARGETS[name]], directory, logs / f"{name}-intent.log", env)
                result["tests_exit"] = run(["cargo", "test", "--workspace", "--no-fail-fast"], directory, logs / f"{name}-tests.log", env)
                killed_by = []
                if result["intent_exit"]:
                    content = (logs / f"{name}-intent.log").read_text(errors="replace")
                    if "intent.rs:" in content and "panicked at" not in content and "resource limit" not in content:
                        killed_by.append("intent lemma")
                    else:
                        result["intent_inconclusive"] = True
                if result["tests_exit"]:
                    content = (logs / f"{name}-tests.log").read_text(errors="replace")
                    # Require actual assertion/test failures, never compilation failure.
                    if "test result: FAILED" not in content:
                        result.update(status="invalid-test-run", killed_by=[])
                        results.append(result)
                        continue
                    result["failed_tests"] = re.findall(r"^test (.*?) \.\.\. FAILED$", content, re.M)
                    killed_by.append("fixture/intent/differential tests")
                status = "killed" if killed_by else ("invalid-intent-run" if result.get("intent_inconclusive") else "survived")
                result.update(status=status, killed_by=killed_by)
            results.append(result)
        assert all(hashlib.sha256((ROOT / p).read_bytes()).hexdigest() == digest for p, digest in original.items()), "working tree changed"
        (logs / "results.json").write_text(json.dumps(results, indent=2) + "\n")
        print(f"{name}: {result['status']}", flush=True)
    (logs / "results.json").write_text(json.dumps(results, indent=2) + "\n")
    print(json.dumps({"total": len(results), "killed": sum(r["status"] == "killed" for r in results), "survivors": [r["mutant"] for r in results if r["status"] == "survived"], "invalid": [r["mutant"] for r in results if r["status"] not in ("killed", "survived", "well-formed")]}))
    return int(any(r["status"] not in ("killed", "survived", "well-formed") for r in results))


if __name__ == "__main__":
    raise SystemExit(main())
