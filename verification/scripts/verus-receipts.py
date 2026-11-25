#!/usr/bin/env python3
"""Run each complete Verus root and retain source-bound coverage-kit receipts."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOTS = ["crates/http-subset/src/lib.rs", "crates/transport-core/src/lib.rs",
         "crates/transport-core/src/aead.rs", "crates/transport-core/src/hello.rs"]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    verus = sys.argv[1]
    version = subprocess.check_output([verus, "--version"], text=True).strip()
    sources = sorted(p for root in ["src", "crates", "tests"] for p in Path(root).rglob("*.rs"))
    files = {p.as_posix(): sha(p) for p in sources}
    for source in ROOTS:
        log = Path("logs") / ("verus-" + Path(source).parent.parent.name + "-" + Path(source).stem + ".log")
        receipt = Path(str(log) + ".evidence.json")
        receipt.unlink(missing_ok=True)
        command = [verus, "--crate-type", "lib", source]
        with log.open("wb") as output:
            result = subprocess.run(command, stdout=output, stderr=subprocess.STDOUT)
        print(log.read_text(), end="", flush=True)
        result.check_returncode()
        if files != {p.as_posix(): sha(p) for p in sources}:
            raise SystemExit("source changed during verification")
        receipt.write_text(json.dumps(dict(tool="verus", tool_version=version, command=command,
                                         exit_code=result.returncode, files=files, log_sha256=sha(log),
                                         harnesses=[], covered_units={}), indent=2) + "\n")


if __name__ == "__main__":
    main()
