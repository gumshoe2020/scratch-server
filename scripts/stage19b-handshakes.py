#!/usr/bin/env python3
"""Run the existing strict rustls detector against a temporary native production server."""
import argparse
import os
from pathlib import Path
import socket
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
env = dict(os.environ, CARGO_HOME=str(ROOT / '.tools/cargo'),
           RUSTUP_HOME=str(ROOT / '.tools/rustup'))
subprocess.run(['cargo', 'build', '--locked'], cwd=ROOT, env=env, check=True)
with socket.socket() as probe:
    probe.bind(('127.0.0.1', 0))
    port = probe.getsockname()[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--tls-dir', type=Path, required=True)
tls = parser.parse_args().tls_dir.resolve()
prefix = os.environ.get('STAGE19_LOG_PREFIX', 'stage19b')
with (ROOT / f'logs/{prefix}-native-server.log').open('w') as log:
    server = subprocess.Popen([str(ROOT / 'target/debug/scratch-server')], cwd=ROOT,
                              env=dict(env, SCRATCH_PORT=str(port), SCRATCH_TLS_DIR=str(tls)),
                              stdout=log, stderr=log)
    try:
        for _ in range(100):
            if server.poll() is not None:
                raise RuntimeError('native server exited before readiness')
            try:
                with socket.create_connection(('127.0.0.1', port), timeout=.1):
                    break
            except OSError:
                time.sleep(.1)
        else:
            raise RuntimeError('native server readiness timeout')
        subprocess.run(['cargo', 'test', '--locked', '--lib', 'binary_rustls_handshakes',
                        '--', '--ignored', '--nocapture', '--exact',
                        'tls::test_provider::binary_handshake_tests::binary_rustls_handshakes'],
                       cwd=ROOT, env=dict(env, M4_PORT=str(port), M4_CERT=str(tls / 'cert.pem'),
                                          M4_N='10000', M4_ASSERT_ZERO='1'), check=True)
    finally:
        server.terminate()
        server.wait(timeout=10)
