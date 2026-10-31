# Historical stage 3 M4 blocked: published coverage does not establish a permitted ARM TLS suite

Stage 5 uses the separately authorized libcrux direction; see [TLS-PROVIDER.md](../../specs/development/TLS-PROVIDER.md).
This file preserves the stage 3 AWS-LC blocker. At that stage, no TLS implementation had been added. HTTP remains operational.
This is a source-coverage blocker, not an observed musl build failure. A static
aws-lc-rs build has not been attempted because it would not resolve this blocker.
No ring fallback, weaker cryptographic policy, or claim of verified TLS is made.

Primary documents actually read:

- [AWS-LC README, Formal Verification](https://github.com/aws/aws-lc/blob/main/README.md#formal-verification)
- [AWS-LC verification README, verified code and caveats](https://github.com/awslabs/aws-lc-verification/blob/master/README.md#verified-code)
- [s2n-bignum proof scope](https://github.com/awslabs/s2n-bignum/blob/main/README.md#testing-and-formal-verification)
- [rustls 0.23.35 AWS-LC provider suite list](https://github.com/rustls/rustls/blob/v/0.23.35/rustls/src/crypto/aws_lc_rs/mod.rs#L123)

| Candidate component | Published coverage | Status for this ARM deployment |
| --- | --- | --- |
| AES-256-GCM | AWS-LC verification lists SandyBridge–Skylake x86, specific EVP APIs, whole-block data, no additional authenticated data | **unconfirmed**; ARM and TLS record usage are not established by that proof |
| SHA-384 | AWS lists x86 and ARM neoverse-n1/v1 implementations, with caveats | implementation/CPU/build correspondence on this Docker host **unconfirmed** |
| HMAC-SHA-384 / TLS HKDF | HMAC proof listed for x86; HKDF row commented out of the verification README | ARM/TLS usage **unconfirmed** |
| X25519 | AWS-LC lists s2n-bignum selector functions for ARM and x86; HOL Light proofs cover those routines | low-level routines documented; complete provider glue and build correspondence **unconfirmed** |
| AES-128-GCM / ChaCha20-Poly1305 | no corresponding entry in the read verification table | **unconfirmed** |

The rustls provider's candidate suites use AES-GCM or ChaCha20-Poly1305. Inference:
the read primary sources do not establish any complete suite meeting M4's
restriction for aarch64-linux. Even the documented x86 AES-256-GCM result excludes
AAD and non-whole-block data, both relevant to ordinary TLS records. It would be
incorrect to call a TLS suite fully covered based only on matching algorithm names.
The newer s2n-bignum direction does not, in the read sources, close that record
cipher/API gap. Absence from a table is not proof that verification is impossible.

Resume M4 when exact implementation/target/API coverage is established, or when
Andrew explicitly revises the requirement to permit documented partially covered
crypto. Under the current spec, do not ship an unconfirmed suite as an accepted M4.
The TLS protocol, X.509 validation, randomness, record handling and provider glue
would remain trusted even then. Libcrux was not substituted; its suitability and
coverage for this precise deployment are unconfirmed.

`scripts/make-lan-cert.py /absolute/existing/private/directory hostname-or-IP`
prepares a 30-day self-signed P-384 LAN certificate and key outside this repository.
It refuses repository paths (including resolved symlinks) and existing files.
The script was syntax/path-refusal checked; generation was not run because TLS is
blocked. No keys/certificates were generated or committed, and the current server
does not read certificate files. Runtime TLS configuration and handshake tests
remain outstanding with M4.
