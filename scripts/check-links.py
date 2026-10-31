#!/usr/bin/env python3
"""Check local Markdown inline/reference links and GitHub-style heading anchors."""
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]

def anchors(path):
    found, counts = set(), {}
    fenced = False
    for line in path.read_text().splitlines():
        if re.match(r'^\s*(```|~~~)', line):
            fenced = not fenced
        if fenced:
            continue
        m = re.match(r'^\s{0,3}#{1,6}\s+(.+?)(?:\s+#+)?$', line)
        if m:
            title = re.sub(r'\[([^]]+)\]\([^)]*\)', r'\1', m[1])
            slug = re.sub(r'[^\w\- ]', '', title.lower()).replace(' ', '-')
            n = counts.get(slug, 0)
            counts[slug] = n + 1
            found.add(slug + (f'-{n}' if n else ''))
        found.update(re.findall(r'<a\s+(?:name|id)=["\']([^"\']+)', line))
    return found

def main():
    files = subprocess.check_output(['git', 'ls-files', '--cached', '--others', '--exclude-standard'], cwd=ROOT, text=True).splitlines()
    errors, checked = [], 0
    for name in sorted(set(files)):
        path = ROOT / name
        if path.suffix != '.md' or not path.is_file():
            continue
        text = re.sub(r'(?ms)^\s*```.*?^\s*```[^\n]*', '', path.read_text())
        targets = re.findall(r'!?\[[^\]\n]*\]\((<[^>]+>|[^\s)]+)(?:\s+[^)]*)?\)', text)
        targets += re.findall(r'^\s*\[[^]]+\]:\s*(\S+)', text, re.M)
        for target in targets:
            target = target.strip('<>')
            url = urlsplit(target)
            if url.scheme or url.netloc or target.startswith('/'):
                continue
            dest = path.parent / unquote(url.path) if url.path else path
            checked += 1
            if not dest.exists():
                errors.append(f'{name}: missing {target}')
            elif re.fullmatch(r'L\d+(?:-L\d+)?', url.fragment) and dest.is_file():
                lines = len(dest.read_text().splitlines())
                if any(int(n) > lines or int(n) < 1 for n in re.findall(r'\d+', url.fragment)):
                    errors.append(f'{name}: invalid line anchor {target}')
            elif url.fragment and dest.suffix == '.md' and unquote(url.fragment) not in anchors(dest):
                errors.append(f'{name}: missing anchor {target}')
    print('\n'.join(errors)) if errors else None
    print(f'{checked} local links checked; {len(errors)} broken')
    return bool(errors)

if __name__ == '__main__':
    sys.exit(main())
