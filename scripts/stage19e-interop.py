#!/usr/bin/env python3
"""Preserve the spike matrix, checking scratch-server's established timeout response.

The spike silently closed; proven-io and scratch-server reject partial HTTP with
400 at the absolute deadline. Keep the original scripts read-only and check the
stronger stage-16 response/closure/timing contract for this one row.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OLD_TRICKLE = '''with socket.create_connection((HOST,PORT),timeout=3) as raw:
 with ctx.wrap_socket(raw,server_hostname='localhost') as tls:
  tls.settimeout(13); start=time.monotonic(); tls.sendall(b'GET / HTTP/1.1\\r\\n')
  closed=False
  try:
   for _ in range(12):
    time.sleep(1); tls.sendall(b'X')
   closed=not tls.recv(1)
  except (ssl.SSLError,OSError): closed=True
  assert closed and time.monotonic()-start<13
'''
NEW_TRICKLE = '''# Observe the response as it arrives, like the stage-16 deadline test.
# Run against both the unchanged reference and the new image.
for deadline_port in [8098, PORT]:
 with socket.create_connection((HOST,deadline_port),timeout=3) as raw:
  with ctx.wrap_socket(raw,server_hostname='localhost') as tls:
   tls.settimeout(.05); start=time.monotonic(); next_byte=1; response=b''
   tls.sendall(b'GET / HTTP/1.1\\r\\n')
   while True:
    elapsed=time.monotonic()-start
    assert elapsed<12, 'HTTP deadline exceeded'
    if elapsed>=next_byte and not response:
     tls.sendall(b'X'); next_byte+=1
    try:
     chunk=tls.recv(4096)
     if not chunk: break
     response+=chunk
     assert len(response)<=4096
    except (TimeoutError, socket.timeout): pass
   elapsed=time.monotonic()-start
   assert response.startswith(b'HTTP/1.1 400 Bad Request\\r\\n')
   assert 10<=elapsed<12
   tls.unwrap().close()
'''
source = (ROOT / 'scripts/stage19-interop.py').read_text()
anchor = 'for old, new in changes.items():'
assert source.count(anchor) == 1
source = source.replace(anchor, 'changes[OLD_TRICKLE] = NEW_TRICKLE\n' + anchor)
exec(compile(source, '<stage19e response-contract adaptation>', 'exec'),
     {'__file__': __file__, '__name__': '__main__',
      'OLD_TRICKLE': OLD_TRICKLE, 'NEW_TRICKLE': NEW_TRICKLE})
