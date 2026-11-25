#!/usr/bin/env python3
"""Archive URLs and actual SHA256s for the D2 source/tool downloads."""
from pathlib import Path
import os
import hashlib
import json
import re
import tarfile
import tomllib

ROOT = Path(os.environ['BERTIE_HAX_TOOLCHAIN'])
rows = []
for p in (ROOT / 'cargo-home/registry/cache').glob('*/*.crate'):
    # Cargo filenames are unambiguously matched to lockfile package identities.
    for lock in (ROOT / 'source').glob('*/Cargo.lock'):
        packages = tomllib.loads(lock.read_text())['package']
        matches = [x for x in packages if 'checksum' in x and f"{x['name']}-{x['version']}.crate" == p.name]
        if matches:
            x = matches[0]
            digest = hashlib.sha256(p.read_bytes()).hexdigest()
            assert digest == x['checksum'], p
            rows.append(dict(url=f"https://static.crates.io/crates/{x['name']}/{p.name}", sha256=digest))
            break
    else:
        # Extraction resolves additional crates; filename split at the version digit.
        name, version = re.match(r'(.+)-(\d.*)\.crate$', p.name).groups()
        rows.append(dict(url=f'https://static.crates.io/crates/{name}/{p.name}',
                         sha256=hashlib.sha256(p.read_bytes()).hexdigest()))
# Match OPAM cache objects to the exact URL blocks in its downloaded repository.
cache = {}
for p in (ROOT / 'home/.opam/download-cache').glob('*/*/*'):
    if p.is_file():
        content = p.read_bytes()
        cache[hashlib.sha256(content).hexdigest()] = (p, content)
        cache[hashlib.sha512(content).hexdigest()] = (p, content)
        cache[hashlib.md5(content).hexdigest()] = (p, content)
        rel = p.relative_to(ROOT / 'home/.opam/download-cache')
        rows.append(dict(url='https://opam.ocaml.org/cache/'+str(rel), sha256=hashlib.sha256(content).hexdigest()))
archive = ROOT / 'home/.opam/repo/default.tar.gz'
if archive.exists():
    with tarfile.open(archive) as tf:
        for item in tf:
            if not item.name.endswith('/opam'):
                continue
            text = tf.extractfile(item).read().decode(errors='replace')
            for block in re.findall(r'(?:url|extra-source\s+"[^"]+")\s*\{(.*?)\}', text, re.S):
                urls = re.findall(r'(?:src|archive):\s*"([^"]+)"', block)
                hashes = re.findall(r'(?:sha256|sha512|md5)=([a-f0-9]+)', block)
                for digest in hashes:
                    if digest in cache:
                        p, content = cache[digest]
                        rows.extend(dict(url=url, sha256=hashlib.sha256(content).hexdigest()) for url in urls)
rows = sorted({(x['url'], x['sha256']) for x in rows})
(ROOT / 'dependency-downloads.json').write_text(json.dumps([dict(url=u, sha256=h) for u,h in rows], indent=2)+'\n')
print(f'{len(rows)} dependency download URL/checksum records')
