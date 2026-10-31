#!/usr/bin/env python3
"""Stage 17: capture real ClientHellos (openssl s_client, curl, rustls) against a local
instance of the server, through a recording TCP relay. Stores only the TLS records that
carry each client's first ClientHello, in fuzz/seeds/hellos/<client>.bin.

Usage: STAGE17_TLS_DIR=$TMPDIR/stage17-tls scripts/stage17-capture-hellos.py
Needs target/release/scratch-server (cargo build --release) and the seed_corpus example
(built here with the fuzz crate's toolchain). Ports 18443 (server) and 18444 (relay).
"""
import os, socket, subprocess, sys, threading, time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TLS = Path(os.environ["STAGE17_TLS_DIR"])
SERVER_PORT, RELAY_PORT = 18443, 18444
OUT = ROOT / "fuzz" / "seeds" / "hellos"
captured = {}
label = {"name": None}


def hello_records(buf):
    """Return the prefix of buf holding complete records up to the end of the first
    handshake message, once it is all present; None if more bytes are needed."""
    pos, hs = 0, b""
    while pos + 5 <= len(buf):
        typ, length = buf[pos], int.from_bytes(buf[pos + 3:pos + 5], "big")
        if typ != 22:
            raise ValueError(f"first record type {typ}, not handshake")
        if pos + 5 + length > len(buf):
            return None
        hs += buf[pos + 5:pos + 5 + length]
        pos += 5 + length
        if len(hs) >= 4 and len(hs) >= 4 + int.from_bytes(hs[1:4], "big"):
            if hs[0] != 1:
                raise ValueError("first handshake message is not a ClientHello")
            return buf[:pos]
    return None


def pump(src, dst, record):
    buf = b""
    try:
        while True:
            data = src.recv(65536)
            if not data:
                break
            if record is not None and record not in captured:
                buf += data
                got = hello_records(buf)
                if got is not None:
                    captured[record] = got
            dst.sendall(data)
    except OSError:
        pass
    finally:
        for s in (src, dst):
            try:
                s.shutdown(socket.SHUT_RDWR)
            except OSError:
                pass


def relay():
    lsock = socket.socket()
    lsock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    lsock.bind(("127.0.0.1", RELAY_PORT))
    lsock.listen(8)
    while True:
        client, _ = lsock.accept()
        upstream = socket.create_connection(("127.0.0.1", SERVER_PORT))
        name = label["name"]
        threading.Thread(target=pump, args=(client, upstream, name), daemon=True).start()
        threading.Thread(target=pump, args=(upstream, client, None), daemon=True).start()


def main():
    binary = ROOT / "target" / "release" / "scratch-server"
    if not binary.exists():
        sys.exit("cargo build --release first")
    log = open(ROOT / "logs" / "stage17-capture-server.log", "w")
    env = dict(os.environ, SCRATCH_PORT=str(SERVER_PORT), SCRATCH_TLS_DIR=str(TLS))
    server = subprocess.Popen([str(binary)], env=env, stdout=log, stderr=log)
    threading.Thread(target=relay, daemon=True).start()
    time.sleep(1)
    cert = str(TLS / "cert.pem")
    url = f"https://localhost:{RELAY_PORT}/"
    resolve = f"localhost:{RELAY_PORT}:127.0.0.1"
    request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"
    seed_example = ROOT / "fuzz" / "target" / "release" / "examples" / "seed_corpus"
    clients = {
        "openssl-s_client": (["openssl", "s_client", "-connect", f"127.0.0.1:{RELAY_PORT}",
                              "-servername", "localhost", "-alpn", "http/1.1", "-CAfile", cert, "-quiet"], request),
        "curl-homebrew-openssl": (["/opt/homebrew/opt/curl/bin/curl", "-s", "-o", "/dev/null", "-w", "%{http_code}",
                                   "--cacert", cert, "--resolve", resolve, url], None),
        "curl-system": (["/usr/bin/curl", "-s", "-o", "/dev/null", "-w", "%{http_code}",
                         "--cacert", cert, "--resolve", resolve, url], None),
        "rustls-client": ([str(seed_example), "connect", f"127.0.0.1:{RELAY_PORT}"], None),
    }
    results = {}
    try:
        for name, (argv, stdin) in clients.items():
            label["name"] = name
            try:
                p = subprocess.run(argv, input=stdin, capture_output=True, timeout=10)
                results[name] = f"exit {p.returncode} out {p.stdout[:40]!r}"
            except subprocess.TimeoutExpired:
                results[name] = "timeout"
            time.sleep(0.5)
    finally:
        server.terminate()
        server.wait()
    OUT.mkdir(parents=True, exist_ok=True)
    for name, outcome in results.items():
        blob = captured.get(name)
        print(f"{name}: {outcome}; hello {len(blob) if blob else 'NOT captured'} bytes")
        if blob:
            (OUT / f"{name}.bin").write_bytes(blob)


if __name__ == "__main__":
    main()
