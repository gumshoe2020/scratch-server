#!/usr/bin/env python3
"""Bind the stage-17 DWARF companion measurement to the gated image archive.

Run the existing bertie-surface build and measurement first with NIX_LOG_PREFIX=stage19e.
Instruction similarity is a multiset comparison, not a semantic equivalence check.
"""
from collections import Counter
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import tarfile

ROOT = Path(__file__).resolve().parents[1]


def masked(word):
    # AArch64 B/BL, B.cond, CBZ/CBNZ, literal loads, TBZ/TBNZ, ADR/ADRP.
    if word & 0x7c000000 == 0x14000000:
        return word & 0xfc000000
    if (word & 0xff000010 == 0x54000000 or word & 0x7e000000 == 0x34000000
            or word & 0x3b000000 == 0x18000000):
        return word & ~0x00ffffe0
    if word & 0x7e000000 == 0x36000000:
        return word & ~0x0007ffe0
    if word & 0x1f000000 == 0x10000000:
        return word & ~0x60ffffe0
    return word


def main():
    out = ROOT / 'target/stage19e-shipped'
    out.mkdir(exist_ok=True)
    info = json.loads(subprocess.check_output(['docker', 'image', 'inspect', 'bertie-boundary:static']))[0]
    entry = info['Config']['Entrypoint'][0].lstrip('/')
    found = False
    with tarfile.open(ROOT / 'image-static.tar.gz') as archive:
        manifest = json.load(archive.extractfile('manifest.json'))[0]
        for layer in manifest['Layers']:
            with tarfile.open(fileobj=archive.extractfile(layer), mode='r|*') as contents:
                for member in contents:
                    if member.name.lstrip('./') == entry:
                        (out / 'scratch-server').write_bytes(contents.extractfile(member).read())
                        found = True
    assert found, 'image binary missing'
    records = {}
    counters = []
    for name, binary in [('shipped', out / 'scratch-server'),
                         ('audit', ROOT / 'target/stage19-musl/scratch-server')]:
        section = out / f'{name}.text'
        copy = out / f'{name}.copy'
        subprocess.run(['/opt/homebrew/opt/llvm/bin/llvm-objcopy', '--dump-section',
                        f'.text={section}', str(binary), str(copy)], check=True)
        copy.unlink()
        data = section.read_bytes()
        records[name] = dict(binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                             text_sha256=hashlib.sha256(data).hexdigest(), text_bytes=len(data))
        counters.append(Counter(masked(w[0]) for w in struct.iter_unpack('<I', data)))
    a, b = counters
    common = sum((a & b).values())
    records.update(image=info['Id'], same_text=records['shipped']['text_sha256'] == records['audit']['text_sha256'],
                   masked_instruction_multiset_common=common,
                   masked_instruction_multiset_fraction=common / max(a.total(), b.total()),
                   limitation='Debug companion has same source/target/profile except debug/strip settings; '
                              'instruction similarity is not byte identity or semantic equivalence.')
    (ROOT / 'logs/stage19e-binary-correspondence.json').write_text(json.dumps(records, indent=2) + '\n')
    path = ROOT / 'logs/stage19e-surface.json'
    report = json.loads(path.read_text())
    report.update(image=info['Id'], image_binary_sha256=records['shipped']['binary_sha256'],
                  image_source_audit=True, correspondence_receipt='logs/stage19e-binary-correspondence.json')
    path.write_text(json.dumps(report, indent=2) + '\n')
    print('Bertie lines:', report['bertie']['inbinary_lines'], 'image:', info['Id'])
    print('Same .text:', records['same_text'], 'masked instruction multiset fraction:',
          records['masked_instruction_multiset_fraction'])


if __name__ == '__main__':
    main()
