#!/usr/bin/env python3
"""Validate spec ownership, tier paths and prose coverage without executing Rust."""
import collections
from pathlib import Path
import re
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[2]

def canonical(path):
    return isinstance(path, str) and bool(path) and not Path(path).is_absolute() and ".." not in Path(path).parts and Path(path).as_posix() == path

def main():
    entries = tomllib.loads((ROOT / 'specs/SPEC-CATALOG.toml').read_text()).get('spec', [])
    errors, ids = [], set()
    owners, prose = collections.defaultdict(list), set()
    for entry in entries:
        name = entry.get('id', '')
        if not name or name in ids:
            errors.append(f'duplicate or empty id: {name}')
        ids.add(name)
        status = entry.get('status')
        if status not in ('established', 'development'):
            errors.append(f'{name}: invalid status {status}')
        if not isinstance(entry.get('reason'), str) or not entry['reason'].strip():
            errors.append(f'{name}: empty reason')
        doc = entry.get('prose', '')
        prose.add(doc)
        if not canonical(doc) or not doc.startswith(f'specs/{status}/') or not doc.endswith('.md'):
            errors.append(f'{name}: prose/tier mismatch: {doc}')
        if not (ROOT / doc).is_file():
            errors.append(f'{name}: missing prose: {doc}')
        for file in entry.get('files', []):
            if not canonical(file):
                errors.append(f'{name}: noncanonical file path: {file}')
            owners[file].append(name)
            if not (ROOT / file).is_file():
                errors.append(f'{name}: missing file: {file}')
    count = 0
    for path in sorted((ROOT / 'crates').rglob('*.rs')):
        # Ignore comments and literal strings before finding declarations.
        text = re.sub(r'//[^\n]*|/\*.*?\*/|b?"(?:\\.|[^"\\])*"', '', path.read_text(), flags=re.S)
        matches = re.findall(r'\bspec\s+fn\s+(\w+)', text)
        count += len(matches)
        if matches and len(owners[path.relative_to(ROOT).as_posix()]) != 1:
            errors.append(f'{path.relative_to(ROOT)}: {len(matches)} spec functions need exactly one owner; got {owners[path.relative_to(ROOT).as_posix()]}')
    for tier in ('established', 'development'):
        for path in (ROOT / 'specs' / tier).rglob('*.md'):
            if path.relative_to(ROOT).as_posix() not in prose:
                errors.append(f'uncatalogued prose: {path.relative_to(ROOT)}')
    print('\n'.join(errors)) if errors else None
    print(f'{len(entries)} entries; {count} spec functions; {len(errors)} catalog errors')
    return bool(errors)

if __name__ == '__main__':
    sys.exit(main())
