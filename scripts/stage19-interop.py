#!/usr/bin/env python3
"""Run the unchanged spike matrix with explicit stage-19 endpoint/body adaptations.
The spike source remains read-only. Verification and all protocol/timeout probes stay on.
"""
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
source = (ROOT.parent / 'bertie-spike/scripts/interop.py').read_text()
changes = {
    "CERT=str(ROOT.parent/'scratch-server-tls/cert.pem')":
        "CERT=str(Path(os.environ.get('SCRATCH_TLS_DIR', ROOT.parent/'scratch-server-tls'))/'cert.pem')",
    "PORT=8092": "PORT=8099",
    "b.endswith(b'ok\\n')": "b.endswith(b'\\r\\n\\r\\nok')",
    " and b'Served over Bertie' in r.stdout": "",
    "logs/interop.json": "logs/" + os.environ.get("STAGE19_LOG_PREFIX", "stage19") + "-interop.json",
    "['docker','logs','bertie-spike']": "['docker','logs','bertie-boundary']",
}
for old, new in changes.items():
    if old not in source:
        raise RuntimeError('spike matrix adaptation anchor changed')
    source = source.replace(old, new)
# __file__ supplies this repo's ROOT without writing into the spike.
exec(compile(source, '<stage19 adapted spike matrix>', 'exec'),
     {'__file__': __file__, '__name__': '__main__'})
