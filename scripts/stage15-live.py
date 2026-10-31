#!/usr/bin/env python3
"""Run the existing acceptance suite on stage 15, then compare rejection stdout."""
import importlib.util
import json
from pathlib import Path
import socket
import ssl
import subprocess
import sys
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[1]
LOGS = ROOT / "logs/stage15-live"
# Reuse all existing checks unchanged; namespace their generated evidence.
spec = importlib.util.spec_from_file_location("stage15_acceptance", ROOT / "scripts/acceptance.py")
acceptance = importlib.util.module_from_spec(spec)
sys.argv = [str(ROOT / "scripts/acceptance.py"), "--container", "proven-record", "--port", "8097", *sys.argv[1:]]
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
        before = [stdout(name) for name in ["proven-dispatch", "proven-record"]]
        replies = [exchange(port, wire) for port in [8096, 8097]]
        assert replies[0] == replies[1] and replies[1].startswith(b"HTTP/1.1 "), f"response comparison {case} failed"
        after = [stdout(name) for name in ["proven-dispatch", "proven-record"]]
        suffix = [current[len(previous):] for previous, current in zip(before, after)]
        assert suffix[0] == suffix[1] and suffix[1].count(b"\n") == (0 if case < 4 else 1), f"stdout comparison {case} failed"
        lines.append(suffix[1].decode("ascii").rstrip("\n"))
    result = dict(date=datetime.now(timezone.utc).isoformat(), cases=len(requests), responses_identical=True,
                  stdout_byte_identical=True, lines=lines)
    (LOGS / "rejection-equivalence.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({key: value for key, value in result.items() if key != "lines"}, indent=2))


if __name__ == "__main__":
    acceptance.main()
    compare()
