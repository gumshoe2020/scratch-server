#!/usr/bin/env python3
"""Compare the exact 18 stage-16 HTTP requests, with verified TLS on both ports."""
import ast
import json
import os
import socket
import ssl
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CERT = Path(os.environ.get('SCRATCH_TLS_DIR', ROOT.parent / 'scratch-server-tls')) / 'cert.pem'


def exchange(port, request):
    context = ssl.create_default_context(cafile=str(CERT))
    context.minimum_version = context.maximum_version = ssl.TLSVersion.TLSv1_3
    context.set_alpn_protocols(['http/1.1'])
    with socket.create_connection(('127.0.0.1', port), timeout=5) as raw:
        with context.wrap_socket(raw, server_hostname='localhost') as stream:
            stream.sendall(request)
            response = bytearray()
            while True:
                block = stream.recv(4096)
                if not block:
                    return bytes(response)
                response.extend(block)
                if len(response) > 65536:
                    raise RuntimeError('response exceeds comparison bound')


def main():
    tree = ast.parse((ROOT / 'scripts/stage16-live.py').read_text())
    compare = next(n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == 'compare')
    assignment = next(n for n in compare.body if isinstance(n, ast.Assign)
                      and isinstance(n.targets[0], ast.Name) and n.targets[0].id == 'requests')
    # The historical list contains only literals, byte multiplication and addition.
    expression = ast.Expression(assignment.value)
    for node in ast.walk(expression):
        if not isinstance(node, (ast.Expression, ast.List, ast.Constant, ast.BinOp,
                                 ast.Add, ast.Mult, ast.Load)):
            raise RuntimeError('unexpected historical request expression')
    requests = eval(compile(expression, '<stage16 requests>', 'eval'), {'__builtins__': {}})
    assert len(requests) == 18
    rows = []
    for case, request in enumerate(requests):
        old, new = exchange(8098, request), exchange(8099, request)
        identical = old == new and new.startswith(b'HTTP/1.1 ')
        rows.append({'case': case, 'identical': identical})
        print(f'case {case}: {"PASS" if identical else "FAIL"}', flush=True)
    result = {'cases': len(rows), 'identical': all(r['identical'] for r in rows), 'rows': rows}
    (ROOT / 'logs' / (os.environ.get('STAGE19_LOG_PREFIX', 'stage19') + '-comparison.json')).write_text(json.dumps(result, indent=2) + '\n')
    assert result['identical']


if __name__ == '__main__':
    main()
