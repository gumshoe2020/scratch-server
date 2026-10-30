#!/usr/bin/env python3
"""Check an already running HTTPS container, leaving it running.
Run after ./nix-build.sh image-static and the documented hardened docker run.
"""
from datetime import datetime, timezone
import argparse
import json
import os
from pathlib import Path
import socket
import struct
import ssl
import subprocess
import tarfile
import time
from concurrent.futures import ThreadPoolExecutor

ROOT = Path(__file__).resolve().parents[1]
LOGS = ROOT / "logs"
PARSER = argparse.ArgumentParser(description=__doc__)
PARSER.add_argument('--container', default='scratch-server')
PARSER.add_argument('--port', type=int, default=8443)
PARSER.add_argument('--host', default='localhost')
PARSER.add_argument('--tls-dir', type=Path, default=ROOT.parent / 'scratch-server-tls')
PARSER.add_argument('--lan-host', default=os.environ.get('LAN_HOST'))
ARGS = PARSER.parse_args()
if not 1 <= ARGS.port <= 65535:
    PARSER.error('--port must be in 1..65535')
NAME, PORT, HOST = ARGS.container, ARGS.port, ARGS.host


def docker(*args):
    return subprocess.check_output(["docker", *args], text=True)


def connect(address):
    context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    context.check_hostname = False
    context.verify_mode = ssl.CERT_NONE
    context.minimum_version = ssl.TLSVersion.TLSv1_3
    context.maximum_version = ssl.TLSVersion.TLSv1_3
    context.set_alpn_protocols(["http/1.1"])
    stream = context.wrap_socket(socket.create_connection((address, PORT), timeout=5), server_hostname="localhost")
    assert stream.version() == "TLSv1.3"
    assert stream.cipher()[0] == "TLS_CHACHA20_POLY1305_SHA256"
    assert stream.selected_alpn_protocol() == "http/1.1"
    return stream


def main():
    LOGS.mkdir(exist_ok=True)
    container = json.loads(docker("inspect", NAME))[0]
    assert container["State"]["Running"]
    assert container["State"]["Health"]["Status"] == "healthy"
    host = container["HostConfig"]
    assert host["ReadonlyRootfs"] and host["CapDrop"] == ["ALL"]
    assert "no-new-privileges" in host["SecurityOpt"] or "no-new-privileges:true" in host["SecurityOpt"]
    assert host["PortBindings"]["8080/tcp"][0]["HostPort"] == str(PORT)
    assert container["Config"]["User"] == "65532:65532"
    image = json.loads(docker("image", "inspect", container["Image"]))[0]
    assert image["Size"] < 8_000_000
    assert "scratch-server:static" in image["RepoTags"]
    assert "SCRATCH_HEADER_POLICY=/config" in container["Config"]["Env"]
    mounts = container["Mounts"]
    policy_mount = [m for m in mounts if m["Destination"] == "/config"]
    assert len(policy_mount) == 1 and not policy_mount[0]["RW"]
    assert policy_mount[0]["Source"] == str(ROOT / "config/header-policy.conf")
    tls_mount = [m for m in mounts if m["Destination"] == "/tls"]
    assert len(tls_mount) == 1 and not tls_mount[0]["RW"]
    assert tls_mount[0]["Source"] == str(ARGS.tls_dir.resolve())
    private_dir = Path(tls_mount[0]["Source"])
    assert private_dir.stat().st_mode & 0o777 == 0o700
    assert (private_dir / "key.pem").stat().st_mode & 0o777 == 0o600
    # Avoid urllib's automatic out-of-subset Accept-Encoding header by using raw TCP.
    def get(path, address=HOST):
        with connect(address) as stream:
            stream.sendall(f"GET {path} HTTP/1.1\r\nHost: localhost\r\n\r\n".encode())
            return read_all(stream)
    for path, body in [("/", b"Hello from a FROM-scratch container."), ("/health", b"ok")]:
        response = get(path)
        assert response.startswith(b"HTTP/1.1 200 OK\r\n")
        assert response.split(b"\r\n\r\n", 1)[1].startswith(body)
    lan_host = ARGS.lan_host
    if lan_host:
        assert get("/health", lan_host).endswith(b"\r\n\r\nok")
        assert b"Hello from a FROM-scratch container." in get("/", lan_host)
    external_tls_checks(lan_host or HOST)
    idle_checks()
    entry = container["Config"]["Entrypoint"][0]
    subprocess.run(["docker", "exec", NAME, entry, "--healthcheck"], check=True)
    missing = subprocess.run(["docker", "exec", NAME, "sh"], capture_output=True, text=True)
    assert missing.returncode != 0 and "executable file not found" in (missing.stdout + missing.stderr)
    (LOGS / "no-shell.txt").write_text(missing.stdout + missing.stderr)
    archive = LOGS / "header-policy-filesystem.tar"
    with archive.open("wb") as output:
        subprocess.run(["docker", "export", NAME], check=True, stdout=output)
    with tarfile.open(archive) as filesystem:
        members = filesystem.getmembers()
        (LOGS / "filesystem.txt").write_text("\n".join(m.name for m in members) + "\n")
        # Docker injects these files even when the image is entirely empty.
        docker_files = {".dockerenv", "dev/console", "etc/hosts", "etc/hostname", "etc/resolv.conf", "config"}
        binaries = [m for m in members if m.isfile() and m.name not in docker_files]
        assert [m.name for m in binaries] == [entry.lstrip("/")], [m.name for m in binaries]
        binary = filesystem.extractfile(binaries[0]).read()
        assert binary[:6] == b"\x7fELF\x02\x01", "expected ELF64 little-endian"
        phoff = struct.unpack_from("<Q", binary, 32)[0]
        entsize, count = struct.unpack_from("<HH", binary, 54)
        for i in range(count):
            offset = phoff + i * entsize
            kind = struct.unpack_from("<I", binary, offset)[0]
            assert kind != 3, "ELF interpreter present"
            if kind == 2:  # static PIE may have dynamic relocations, but no DT_NEEDED
                dynamic_offset = struct.unpack_from("<Q", binary, offset + 8)[0]
                dynamic_size = struct.unpack_from("<Q", binary, offset + 32)[0]
                for d in range(dynamic_offset, dynamic_offset + dynamic_size, 16):
                    assert struct.unpack_from("<Q", binary, d)[0] != 1, "shared library dependency present"
    image_archive = LOGS / "header-policy-image.tar"
    subprocess.run(["docker", "image", "save", container["Image"], "-o", str(image_archive)], check=True)
    with tarfile.open(image_archive) as saved:
        manifest = json.load(saved.extractfile("manifest.json"))[0]
        regular = []
        for name in manifest["Layers"]:
            with tarfile.open(fileobj=saved.extractfile(name)) as layer:
                regular.extend(m.name.lstrip("/") for m in layer.getmembers() if m.isfile())
        assert regular == [entry.lstrip("/")], regular
    corpus = [bytes.fromhex(line) for line in (ROOT / "tests/malformed.hex").read_text().splitlines()
              if line and not line.startswith("#")]

    # The original hosts and X-Test cases are well-formed unknowns under default.
    corpus = [request for i, request in enumerate(corpus) if i not in (33, 39)]
    for name in ["content-length", "TRANSFER-ENCODING", "cOoKiE"]:
        corpus.append(f"GET / HTTP/1.1\r\n{name}: x\r\n\r\n".encode("ascii"))
    for i, request in enumerate(corpus):
        with connect(HOST) as stream:
            stream.sendall(request)
            response = read_all(stream)
            assert response.startswith(b"HTTP/1.1 400 Bad Request\r\n"), (i, response)
    # Replay captured HTTPS name order with synthetic values over TLS.
    fixture_log_start = len(docker("logs", NAME).splitlines())
    for fixture, status in [("safari-https-root.http", b"200 OK"), ("safari-https-favicon.http", b"404 Not Found"), ("firefox-https-root.http", b"200 OK"), ("firefox-https-favicon.http", b"404 Not Found"), ("brave-https-root.http", b"200 OK"), ("brave-https-favicon.http", b"404 Not Found"), ("brave-cache-disabled-https-root.http", b"200 OK"), ("brave-dnt-https-root.http", b"200 OK")]:
        wire = (ROOT / "tests" / fixture).read_bytes()
        for chunk in [len(wire), 1, 7, 1024]:
            with connect(HOST) as stream:
                for offset in range(0, len(wire), chunk):
                    stream.sendall(wire[offset:offset + chunk])
                response = read_all(stream)
                assert response.startswith(b"HTTP/1.1 " + status + b"\r\n")
                assert b"\r\nConnection: close\r\n" in response
    fixture_lines = docker("logs", NAME).splitlines()[fixture_log_start:]
    assert fixture_lines.count("status=200 note=unknown_header name=DNT") == 1
    # Both fixture replays preserve the synthetic headers and suppress curl defaults.
    for browser in ["safari", "firefox", "brave", "brave-cache-disabled"]:
        request = (ROOT / f"tests/{browser}-https-root.http").read_bytes().decode("ascii")
        args = ["curl", "--silent", "--show-error", "--fail", "--max-time", "5", "--http1.1", "--insecure",
                "--output", str(LOGS / f"{browser}-curl-body.txt"), "--write-out", "%{http_code}"]
        for line in request.split("\r\n")[1:]:
            if line:
                args.extend(["--header", line])
        args.append(f"https://{HOST}:{PORT}/")
        # Do not expose the header-bearing command in a CalledProcessError.
        replay = subprocess.run(args, capture_output=True, text=True)
        assert replay.returncode == 0 and replay.stdout == "200", f"{browser} curl replay failed"
    # Preserve the exact sanitized name case and never log values.
    for name in ["cOoKiE", "content-length", "TRANSFER-ENCODING"]:
        before = docker("logs", NAME).splitlines()
        with connect(HOST) as stream:
            stream.sendall(f"GET / HTTP/1.1\r\n{name}: synthetic\r\n\r\n".encode())
            assert read_all(stream).startswith(b"HTTP/1.1 400 Bad Request\r\n")
        assert docker("logs", NAME).splitlines()[len(before):] == [f"status=400 reason={'guarded' if name == 'cOoKiE' else 'hard_reject'} name={name}"]
    for count in [24, 25]:
        with connect(HOST) as stream:
            stream.sendall(("GET / HTTP/1.1\r\n" + "aCcEpT: synthetic\r\n" * count + "\r\n").encode())
            assert read_all(stream).startswith(b"HTTP/1.1 200" if count == 24 else b"HTTP/1.1 400")
    # A Cookie value must never enter stdout: compare the entire added log suffix.
    before = docker("logs", NAME).splitlines()
    with connect(HOST) as stream:
        stream.sendall(b"GET / HTTP/1.1\r\nCookie: x\r\n\r\n")
        assert read_all(stream).startswith(b"HTTP/1.1 400 Bad Request\r\n")
    added = docker("logs", NAME).splitlines()[len(before):]
    assert added == ["status=400 reason=guarded name=Cookie"], added
    (LOGS / "cookie-rejection.txt").write_text("\n".join(added) + "\n")
    # Duplicate diagnostics and sliding-minute aggregate bound (1000 real TLS requests).
    before = len(docker("logs", NAME).splitlines())
    with connect(HOST) as stream:
        stream.sendall(b"GET / HTTP/1.1\r\nHost: synthetic\r\nhOsT: synthetic\r\n\r\n")
        assert read_all(stream).startswith(b"HTTP/1.1 400")
    assert docker("logs", NAME).splitlines()[before:] == ["status=400 reason=duplicate name=hOsT"]
    before = len(docker("logs", NAME).splitlines())
    started = time.monotonic()
    def probe(i):
        with connect(HOST) as stream:
            stream.sendall(f"GET / HTTP/1.1\r\nX-Probe-{i % 100}: synthetic\r\n\r\n".encode())
            assert read_all(stream).startswith(b"HTTP/1.1 200")
    with ThreadPoolExecutor(max_workers=8) as pool:
        list(pool.map(probe, range(1000)))
    assert time.monotonic() - started < 60, "rate test exceeded one minute; bound must be reassessed"
    added = docker("logs", NAME).splitlines()[before:]
    notes = [line for line in added if "note=unknown_header" in line]
    assert len(notes) <= 60, "default sliding-minute budget exceeded"
    names = [line.split("name=", 1)[1].lower() for line in notes]
    assert len(names) == len(set(names))
    result = dict(date=datetime.now(timezone.utc).isoformat(), image=image["Id"], size=image["Size"],
                  binary_bytes=len(binary), malformed=len(corpus), static=True, shell=False,
                  hardened=True, routes=True, healthcheck=True, safari=True, firefox=True, brave=True, brave_cache_disabled=True, cookie_log=True, tls=True,
                  idle=True, slow=True, capacity=True, log_probe_requests=1000, log_probe_names=100, log_bound=60, log_lines=len(notes), lan_host=lan_host)
    (LOGS / "stage12-acceptance.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


def profile_checks(profile):
    """Additional checks when the selected container runs another config."""
    startup = [line for line in docker("logs", NAME).splitlines() if line.startswith("policy=")]
    assert len(startup) == 1
    if profile == "builtin":
        assert startup == ["policy=builtin unknown=drop guarded-drop= log-unknown-per-minute=60"]
    elif profile == "strict":
        assert startup == ["policy=file unknown=reject guarded-drop= log-unknown-per-minute=60"]
    elif profile == "cookie":
        assert startup == ["policy=file unknown=drop guarded-drop=Cookie log-unknown-per-minute=60"]
    else:
        raise ValueError("unknown acceptance profile")
    def request(wire):
        with connect(HOST) as stream:
            stream.sendall(wire)
            return read_all(stream)
    if profile == "cookie":
        assert request(b"GET / HTTP/1.1\r\nCookie: synthetic\r\n\r\n").startswith(b"HTTP/1.1 200")
    else:
        for fixture in ["safari", "firefox", "brave", "brave-cache-disabled"]:
            wire = (ROOT / f"tests/{fixture}-https-root.http").read_bytes()
            assert request(wire).startswith(b"HTTP/1.1 200")
        before = len(docker("logs", NAME).splitlines())
        wire = (ROOT / "tests/brave-dnt-https-root.http").read_bytes()
        for chunk in [len(wire), 1, 7, 1024]:
            with connect(HOST) as stream:
                for offset in range(0, len(wire), chunk):
                    stream.sendall(wire[offset:offset+chunk])
                assert read_all(stream).startswith(b"HTTP/1.1 400" if profile == "strict" else b"HTTP/1.1 200")
        added = docker("logs", NAME).splitlines()[before:]
        expected = "status=400 reason=unknown_header name=DNT" if profile == "strict" else "status=200 note=unknown_header name=DNT"
        assert added == [expected] * (4 if profile == "strict" else 1)
    for name in ["Host", "Connection"]:
        before = len(docker("logs", NAME).splitlines())
        wire = f"GET / HTTP/1.1\r\n{name}: synthetic\r\n{name.lower()}: synthetic\r\n\r\n".encode()
        assert request(wire).startswith(b"HTTP/1.1 400")
        assert docker("logs", NAME).splitlines()[before:] == [f"status=400 reason=duplicate name={name.lower()}"]
    print(json.dumps(dict(profile=profile, passed=True)))


def idle_checks():
    def scenario(kind):
        with connect(HOST) as stream:
            stream.settimeout(20)
            if kind == "delayed":
                time.sleep(3)
                stream.sendall(b"GET / HTTP/1.1\r\nHost: x\r\n\r\n")
                assert read_all(stream).startswith(b"HTTP/1.1 200 OK\r\n")
            elif kind == "partial":
                stream.sendall(b"GET / HTTP/1.1\r\nHost:")
                time.sleep(3)
                assert read_all(stream).startswith(b"HTTP/1.1 400 Bad Request\r\n")
            else:
                time.sleep(17)
                assert read_all(stream) == b""
    before = docker("logs", NAME).splitlines()
    with ThreadPoolExecutor(max_workers=3) as pool:
        list(pool.map(scenario, ["delayed", "partial", "idle"]))
    added = docker("logs", NAME).splitlines()[len(before):]
    assert added.count("status=400 reason=slow") == 1, added
    assert added.count("closed reason=idle") == 1, added
    # Grammar and size classes must be explicit and never include input bytes.
    for wire, line in [
        (b"GET /? HTTP/1.1\r\n\r\n", "status=400 reason=grammar"),
        (b"G" * 4097, "status=400 reason=size"),
    ]:
        before = docker("logs", NAME).splitlines()
        with connect(HOST) as stream:
            stream.sendall(wire)
            assert read_all(stream).startswith(b"HTTP/1.1 400 Bad Request\r\n")
        assert docker("logs", NAME).splitlines()[len(before):] == [line]
    # Fill workers with no-handshake sockets, then prove excess sockets are closed.
    sockets = []
    try:
        time.sleep(0.2)
        for _ in range(64):
            sockets.append(socket.create_connection((HOST, PORT), timeout=3))
        time.sleep(0.3)
        before = docker("logs", NAME).splitlines()
        with socket.create_connection((HOST, PORT), timeout=3) as excess:
            excess.settimeout(2)
            assert excess.recv(1) == b""
        added = docker("logs", NAME).splitlines()[len(before):]
        assert "closed reason=capacity" in added, added
    finally:
        for stream in sockets:
            stream.close()
    time.sleep(0.3)
    with connect(HOST) as stream:
        stream.sendall(b"GET /health HTTP/1.0\r\n\r\n")
        assert read_all(stream).startswith(b"HTTP/1.1 200 OK\r\n")


def external_tls_checks(address):
    probes = [
        ("tls13", ["-tls1_3"], True),
        ("tls12", ["-tls1_2"], False),
        ("aes", ["-tls1_3", "-ciphersuites", "TLS_AES_128_GCM_SHA256"], False),
        ("p256-kx", ["-tls1_3", "-groups", "P-256"], False),
        ("h2", ["-tls1_3", "-alpn", "h2"], False),
    ]
    for label, flags, success in probes:
        result = subprocess.run(["openssl", "s_client", "-brief", "-connect", f"{address}:{PORT}", *flags],
            input=b"GET / HTTP/1.0\r\n\r\n", capture_output=True, timeout=8)
        output = (result.stdout + result.stderr).decode("utf-8", errors="replace")
        (LOGS / f"stage12-openssl-{label}.log").write_text(output)
        if success:
            assert result.returncode == 0, output
            assert "TLSv1.3" in output and "TLS_CHACHA20_POLY1305_SHA256" in output and "X25519" in output, output
            assert "Signature type: ecdsa_secp256r1_sha256" in output, output
        else:
            assert result.returncode != 0 and "CONNECTION ESTABLISHED" not in output, output
    curl = subprocess.run(["curl", "-k", "--silent", "--show-error", "--fail", "--max-time", "5",
        f"https://{address}:{PORT}/"], capture_output=True, check=True)
    assert b"Hello from a FROM-scratch container." in curl.stdout
    with socket.create_connection((address, PORT), timeout=3) as plain:
        plain.sendall(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        try:
            response = plain.recv(8192)
        except ConnectionResetError:
            response = b""
        assert b"HTTP/1." not in response, response


def read_all(stream):
    result = bytearray()
    while True:
        try:
            chunk = stream.recv(8192)
        except ConnectionResetError:
            # Rejection can close with unread excess bytes; retain the 400 response.
            break
        if not chunk:
            break
        result.extend(chunk)
    return bytes(result)


if __name__ == "__main__":
    profile = os.environ.get("HEADER_POLICY_ACCEPTANCE_PROFILE")
    if profile:
        profile_checks(profile)
    else:
        main()
