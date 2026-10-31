#!/usr/bin/env python3
"""Compare complete executable function tokens against a git revision.

This layout-only check is deliberately stronger than erasure: contracts, invariants
and ghost blocks must also remain identical. Comments and whitespace are ignored.
Spec/proof items are excluded; executable functions in all tracked Rust files are
included, including tests and runtime adapters. It never executes baseline code.
"""
import argparse
import collections
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LEX = re.compile(r'//[^\n]*|/\*.*?\*/|b?r(\#*)".*?"\1|b?"(?:\\.|[^"\\])*"|b?\'(?:\\.|[^\'\\])\'|\w+|[^\s]', re.S)

def tokens(text):
    return [m[0] for m in LEX.finditer(text) if not m[0].startswith(('//', '/*'))]

def functions(text):
    # Function bodies follow a declaration-line opening brace or an opening
    # brace at the declaration indentation. Contract blocks are more indented.
    clean = LEX.sub(lambda m: ' ' * len(m[0]) if m[0].startswith(('//','/*','"','b"',"'","b'",'r"','br"')) else m[0], text)
    result = []
    for m in re.finditer(r'(?m)^( *)(?:(?:pub(?:\([^)]*\))?|unsafe|async|const|extern)\s+)*fn\s+(\w+)', clean):
        first = clean.index('{', m.end())
        brace = first
        if re.search(r'\b(?:requires|ensures|recommends|decreases)\b', clean[m.end():first]):
            opening = re.search(r'(?m)^' + m[1] + r'\{', clean[m.end():])
            # Inline contracts have no nested blocks in these sources. For
            # multiline contracts, the body opens at declaration indentation.
            next_item = re.search(r'(?m)^' + m[1] + r'(?:pub )?(?:(?:open |closed )?(?:spec |proof ))?fn\b', clean[m.end():])
            if opening and (not next_item or opening.start() < next_item.start()):
                brace = m.end() + opening.end() - 1
        depth, end = 1, brace + 1
        while depth:
            depth += (clean[end] == '{') - (clean[end] == '}')
            end += 1
        result.append((m[2], tuple(tokens(text[m.start():end]))))
    return result

def git(*args):
    return subprocess.check_output(['git', *args], cwd=ROOT, text=True)

def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--before', default='3c5f2f7')
    args = ap.parse_args()
    before, after = collections.Counter(), collections.Counter()
    for f in git('ls-tree','-r','--name-only',args.before).splitlines():
        if f.endswith('.rs'):
            before.update(functions(git('show', f'{args.before}:{f}')))
    for f in git('ls-files','--cached','--others','--exclude-standard').splitlines():
        if f.endswith('.rs') and (ROOT / f).is_file():
            after.update(functions((ROOT / f).read_text()))
    missing, extra = before-after, after-before
    for label, changes in [('missing/changed', missing), ('added/changed', extra)]:
        for (name, _), count in changes.items():
            print(f'{label}: {name} ({count})')
    print(f'{sum(before.values())} executable functions; complete tokens including contracts/ghosts identical: {before == after}')
    return before != after

if __name__ == '__main__':
    raise SystemExit(main())
