#!/usr/bin/env python3
"""Prepare a self-signed P-256 certificate outside this repository.
Usage: scripts/make-lan-cert.py /absolute/existing/private/directory [hostname-or-IP]
The HTTPS server loads these files only at runtime.
"""
import argparse
import ipaddress
import os
from pathlib import Path
import re
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("directory", type=Path, help="existing absolute private directory outside the repository")
    parser.add_argument("host", nargs="?", default="localhost", help="hostname or IP (default: localhost)")
    args = parser.parse_args()
    supplied = args.directory
    destination = supplied.resolve(strict=True)
    repository = Path(__file__).resolve().parents[1]
    if not supplied.is_absolute() or not destination.is_dir():
        raise SystemExit("destination must be an existing absolute directory")
    if destination == repository or repository in destination.parents:
        raise SystemExit("certificate destination must be outside the repository")
    name = args.host
    try:
        address = ipaddress.ip_address(name)
        san = f"IP:{address}"
    except ValueError:
        if len(name) > 253 or not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9.-]*", name):
            raise SystemExit("invalid hostname or IP")
        san = f"DNS:{name}"
    key, cert = destination / "key.pem", destination / "cert.pem"
    # Exclusive creation also refuses existing files and symlinks.
    os.umask(0o077)
    created = []
    try:
        for path in (key, cert):
            path.open("xb").close()
            created.append(path)
        subprocess.run([
            "openssl", "req", "-x509", "-newkey", "ec", "-pkeyopt", "ec_paramgen_curve:prime256v1",
            "-nodes", "-days", "30", "-sha256", "-subj", f"/CN={name}",
            "-addext", f"subjectAltName={san},DNS:localhost,IP:127.0.0.1",
            "-addext", "basicConstraints=critical,CA:FALSE",
            "-addext", "keyUsage=critical,digitalSignature",
            "-addext", "extendedKeyUsage=serverAuth",
            "-keyout", str(key), "-out", str(cert),
        ], check=True)
    except BaseException:
        for path in created:
            path.unlink(missing_ok=True)
        raise
    print(f"Created {cert} and {key}; mount the directory read-only at /tls.")


if __name__ == "__main__":
    main()
