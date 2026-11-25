#!/usr/bin/env python3
"""E3 live acceptance; touches only the two explicitly authorized containers."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
TLS = Path(os.environ['SCRATCH_TLS_DIR']).resolve()
ENV = dict(os.environ, STAGE19_LOG_PREFIX='stage19e', SCRATCH_TLS_DIR=str(TLS))


def docker(*args):
    return subprocess.check_output(['docker', *args], text=True).strip()


def start(name, image, port):
    return docker('run', '-d', '--name', name, '--read-only', '--user', '65532:65532',
                  '--cap-drop', 'ALL', '--security-opt', 'no-new-privileges',
                  '--restart', 'unless-stopped', '-p', f'127.0.0.1:{port}:8080',
                  '--mount', f'type=bind,src={TLS},dst=/tls,readonly',
                  '--mount', f'type=bind,src={ROOT / "config"},dst=/config,readonly',
                  '-e', 'SCRATCH_HEADER_POLICY=/config/header-policy.conf', image)


def main():
    existing = docker('ps', '-a', '--format', '{{.Names}}').splitlines()
    assert not {'bertie-boundary', 'proven-io-ref'}.intersection(existing)
    created = []
    passed = False
    try:
        start('bertie-boundary', 'bertie-boundary:static', 8099)
        created.append('bertie-boundary')
        for _ in range(60):
            state = json.loads(docker('inspect', 'bertie-boundary'))[0]['State']
            if state.get('Health', {}).get('Status') == 'healthy':
                break
            assert state['Running'], 'server exited'
            time.sleep(1)
        else:
            raise RuntimeError('container readiness deadline')
        start('proven-io-ref', 'proven-io:static', 8098)
        created.append('proven-io-ref')
        time.sleep(1)
        subprocess.run(['python3', 'scripts/stage19-compare.py'], cwd=ROOT, env=ENV, check=True)
        subprocess.run(['python3', 'scripts/stage19e-interop.py'], cwd=ROOT, env=ENV, check=True)

        # The spike source is read-only. Build its default-provider rustls client
        # in our ignored target, changing only endpoint and expected HTTP body.
        source = ROOT.parent / 'bertie-spike/client'
        out = ROOT / 'target/stage19e-default-client'
        (out / 'src').mkdir(parents=True, exist_ok=True)
        (out / 'Cargo.toml').write_text((source / 'Cargo.toml').read_text() + '\n[workspace]\n')
        shutil.copyfile(source / 'Cargo.lock', out / 'Cargo.lock')
        client = (source / 'src/main.rs').read_text()
        assert '127.0.0.1:8092' in client and 'b"ok\\n"' in client
        (out / 'src/main.rs').write_text(client.replace('127.0.0.1:8092', '127.0.0.1:8099')
                                       .replace('b"ok\\n"', 'b"\\r\\n\\r\\nok"'))
        subprocess.run(['cargo', 'run', '--locked', '--release', '--manifest-path',
                        str(out / 'Cargo.toml'), '--', str(TLS / 'cert.pem')],
                       cwd=ROOT, check=True)
        matrix = ROOT / 'logs/stage19e-interop.json'
        rows = json.loads(matrix.read_text())
        rows.append(dict(test='rustls default config', result='PASS', certificate_verification=True))
        matrix.write_text(json.dumps(rows, indent=2) + '\n')
        state = json.loads(docker('inspect', 'bertie-boundary'))[0]
        receipt = {k: state[k] for k in ['Id', 'Image', 'State', 'HostConfig', 'Mounts']}
        receipt['User'] = state['Config']['User']
        (ROOT / 'logs/stage19e-container.json').write_text(json.dumps(receipt, indent=2) + '\n')
        assert state['State']['Running'] and state['State']['Health']['Status'] == 'healthy'
        passed = True
    finally:
        if 'proven-io-ref' in created:
            docker('rm', '-f', 'proven-io-ref')
        if not passed and 'bertie-boundary' in created:
            docker('rm', '-f', 'bertie-boundary')


if __name__ == '__main__':
    main()
