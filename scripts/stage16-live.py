#!/usr/bin/env python3
"""Run the existing acceptance suite on stage 16, then compare rejection stdout."""
import importlib.util
import json
from pathlib import Path
import socket
import ssl
import subprocess
import sys
import time
import select
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[1]
LOGS = ROOT / "logs/stage16-live"
# Reuse all existing checks unchanged; namespace their generated evidence.
spec = importlib.util.spec_from_file_location("stage16_acceptance", ROOT / "scripts/acceptance.py")
acceptance = importlib.util.module_from_spec(spec)
sys.argv = [str(ROOT / "scripts/acceptance.py"), "--container", "proven-io", "--port", "8098", "--image-tag", "proven-io:static", "--skip-capacity", *sys.argv[1:]]
spec.loader.exec_module(acceptance)
acceptance.LOGS = LOGS


def stdout(name):
    return subprocess.check_output(["docker", "logs", name], stderr=subprocess.PIPE)


def exchange(port, wire):
    context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    context.check_hostname = False
    context.verify_mode = ssl.CERT_NONE
    context.minimum_version = context.maximum_version = ssl.TLSVersion.TLSv1_3
    context.set_alpn_protocols(["http/1.1"])
    with context.wrap_socket(socket.create_connection(("localhost", port), timeout=5), server_hostname="localhost") as stream:
        stream.sendall(wire)
        return acceptance.read_all(stream)


def compare():
    requests = [
        b"GET / HTTP/1.1\r\nHost: x\r\n\r\n",
        b"GET /health HTTP/1.1\r\nHost: x\r\n\r\n",
        b"GET /missing HTTP/1.1\r\nHost: x\r\n\r\n",
        b"HEAD / HTTP/1.1\r\nHost: x\r\n\r\n",
        b"GET /? HTTP/1.1\r\n\r\n", b"G" * 4097,
        b"GET / HTTP/1.1\r\nCookie: x\r\n\r\n",
        b"GET / HTTP/1.1\r\ncOnTeNt-LeNgTh: x\r\n\r\n",
        b"GET / HTTP/1.1\r\nTransfer-Encoding: x\r\n\r\n",
        b"GET / HTTP/1.1\r\nHost: x\r\nhOsT: x\r\n\r\n",
        b"GET / HTTP/1.1\r\nConnection: x\r\nCONNECTION: x\r\n\r\n",
        b"GET / HTTP/1.1\r\n" + b"Accept: x\r\n" * 25 + b"\r\n",
        b"GET / HTTP/1.1\r\nX\rName: x\r\n\r\n",
        b"GET / HTTP/1.1\r\nX\nName: x\r\n\r\n",
        b"GET / HTTP/1.1\r\n: x\r\n\r\n",
        b"GET / HTTP/1.1\r\nCookie: x\r\nContent-Length: x\r\n\r\n",
        b"GET / HTTP/1.1\r\nContent-Length: x\r\nCookie: x\r\n\r\n",
        b"GET / HTTP/1.1\r\nHost: x\r\nhost: x\r\nCookie: x\r\n\r\n",
    ]
    lines = []
    for case, wire in enumerate(requests):
        before = [stdout(name) for name in ["proven-record", "proven-io"]]
        replies = [exchange(port, wire) for port in [8097, 8098]]
        assert replies[0] == replies[1] and replies[1].startswith(b"HTTP/1.1 "), f"response comparison {case} failed"
        after = [stdout(name) for name in ["proven-record", "proven-io"]]
        suffix = [current[len(previous):] for previous, current in zip(before, after)]
        assert suffix[0] == suffix[1] and suffix[1].count(b"\n") == (0 if case < 4 else 1), f"stdout comparison {case} failed"
        lines.append(suffix[1].decode("ascii").rstrip("\n"))
    result = dict(date=datetime.now(timezone.utc).isoformat(), cases=len(requests), responses_identical=True,
                  stdout_byte_identical=True, lines=lines)
    (LOGS / "rejection-equivalence.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key != "lines"}, indent=2))


def trickle(port, name):
    context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    context.check_hostname = False
    context.verify_mode = ssl.CERT_NONE
    context.minimum_version = context.maximum_version = ssl.TLSVersion.TLSv1_3
    context.set_alpn_protocols(["http/1.1"])
    before = stdout(name)
    wire = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"
    with context.wrap_socket(socket.create_connection(("localhost", port), timeout=5), server_hostname="localhost") as stream:
        start = time.monotonic()
        next_byte = 0
        response = b""
        while time.monotonic() - start < 56:
            elapsed = time.monotonic() - start
            if next_byte < len(wire) and elapsed >= next_byte * 1.5:
                stream.sendall(wire[next_byte:next_byte+1])
                next_byte += 1
            ready, _, _ = select.select([stream], [], [], 0.02)
            if ready:
                response = acceptance.read_all(stream)
                break
        elapsed = time.monotonic() - start
    suffix = stdout(name)[len(before):]
    if port == 8098:
        assert response.startswith(b"HTTP/1.1 400 Bad Request\r\n")
        assert 10 <= elapsed < 12, f"new trickle duration {elapsed}"
        assert suffix == b"status=400 reason=slow\n", "new trickle log mismatch"
    else:
        assert response.startswith(b"HTTP/1.1 200 OK\r\n")
        assert elapsed >= (len(wire)-1) * 1.5
        assert suffix == b"", "old trickle log mismatch"
    return dict(port=port, seconds=elapsed, sent_bytes=next_byte,
                status=400 if port == 8098 else 200, slow_log=port == 8098)


def trickle_compare():
    # Only two slow live connections, one to each stage.
    with ThreadPoolExecutor(max_workers=2) as pool:
        results = list(pool.map(lambda pair: trickle(*pair), [(8098, "proven-io"), (8097, "proven-record")]))
    (LOGS / "trickle-equivalence.json").write_text(json.dumps(results, indent=2) + "\n")
    print(json.dumps(results, indent=2))


if __name__ == "__main__":
    acceptance.main()
    compare()
    trickle_compare()
