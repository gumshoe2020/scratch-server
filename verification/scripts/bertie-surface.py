#!/usr/bin/env python3
"""Stage-17 DWARF method on the stage-19 audit build; not deployment evidence."""
import importlib.util
import json
import os
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('surface', ROOT / 'verification/scripts/rustls-surface.py')
surface = importlib.util.module_from_spec(spec)
spec.loader.exec_module(surface)
surface.BUILD = ROOT / 'target/stage19-musl'
surface.BINARY = surface.BUILD / 'scratch-server'


def main():
    if not surface.BINARY.exists():
        raise SystemExit('run verification/scripts/bertie-surface-build.sh first')
    depfiles = list((surface.BUILD / 'deps').glob('bertie_tls-*.d'))
    if len(depfiles) != 1:
        raise SystemExit('expected exactly one production Bertie dep-info file')
    paths = depfiles[0].read_text().splitlines()[0].split(': ', 1)[1].split()
    lib = next(p for p in paths if p.endswith('/src/lib.rs'))
    dirname = lib.rsplit('/src/', 1)[0].rsplit('/', 1)[1]
    ranges = surface.exec_ranges()
    hits = surface.dwarf_lines([dirname], ranges)
    owners = surface.function_owners([dirname], ranges)
    files = {f: sorted(lines) for (d, f), lines in hits.items() if d == dirname}
    remaining = {}
    for package, crate in surface.CRATES.items():
        remaining[package] = {'compiled_dep_info': len(list((surface.BUILD / 'deps').glob(f'{crate}-*.d')))}
    report = {'method': 'stage17 executable-section DWARF source pairs',
              'binary': str(surface.BINARY.relative_to(ROOT)), 'deployed': False,
              'bertie': {'source_dir': dirname, 'inbinary_lines': sum(map(len, files.values())),
                         'functions': len(owners[dirname]), 'files': files},
              'remaining_tls': remaining,
              'stage17': {'rustls': 4152, 'webpki': 693, 'pki_types': 250, 'total': 5095},
              'limitation': 'audit build with debug info only; reachable-code upper bound, not proof coverage'}
    (ROOT / 'logs' / (os.environ.get('NIX_LOG_PREFIX', 'stage19') + '-surface.json')).write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k: v for k, v in report.items() if k != 'bertie'}, indent=2))
    print('Bertie in-binary lines:', report['bertie']['inbinary_lines'], 'functions:', report['bertie']['functions'])
    if not files:
        raise SystemExit('no Bertie DWARF lines found; measurement invalid')


if __name__ == '__main__':
    main()
