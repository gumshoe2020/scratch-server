#!/bin/sh
# Scan tracked file contents without disclosing the matched private text.
set -eu
if [ "$#" -ne 1 ]; then
  echo 'usage: scripts/check-public.sh DENYLIST' >&2
  exit 2
fi
cd "$(dirname "$0")/.."
python3 - "$1" <<'PY'
from pathlib import Path
import subprocess
import sys
try:
    rules = [line.strip().lower() for line in Path(sys.argv[1]).read_bytes().splitlines() if line.strip()]
except OSError as error:
    raise SystemExit(f"cannot read denylist: {error.strerror}")
rules.append(b'-' * 5 + b'begin')
hits = 0
for raw in subprocess.check_output(['git', 'ls-files', '-z']).split(b'\0'):
    if not raw:
        continue
    path = Path(raw.decode())
    if path.is_symlink():
        continue
    try:
        lines = path.read_bytes().splitlines()
    except FileNotFoundError:
        continue  # staged deletions
    for number, line in enumerate(lines, 1):
        for index, rule in enumerate(rules, 1):
            if rule in line.lower():
                print(f'{path}:{number}: public scan rule {index}')
                hits += 1
print(f'public scan: {hits} hits')
sys.exit(bool(hits))
PY
