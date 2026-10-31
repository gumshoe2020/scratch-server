# ATTACK-SURFACE.md: what's left, and the plan

Goal (Andrew, 2026-10-27): a web application whose only attack surface is the app itself and its
pinned dependencies, with every interface to untrusted input limited to formally verified code
where that exists. "No attack surface" is not a finish line; "every remaining piece named,
measured and shrinking" is. Measured with [formal-coverage](https://github.com/gumshoe2020/formal-coverage).

**Done: nothing in the image but the app.** One static binary (~1.5 MB). No shell, interpreter,
compiler, package manager or shared libraries. Read-only filesystem, no Linux capabilities,
non-root.

After stage 16, measured with the per-stage method in [EVOLUTION.md](EVOLUTION.md): **1,208 of
1,861 lines of running server code proven (64.9%)**. The stage-13 baseline below uses a different
counting rule and is not comparable.

First-party security-relevant code: **644 of 1,363 SLoC proven (47.25%)**, measured by
[formal-coverage v1.4](https://github.com/gumshoe2020/formal-coverage) at `2fa5eb7`;
300 tested, 419 unverified. Counting rule: package source SLoC, with the security
boundary defined by formal-coverage's scratch-server config. Shipped third-party
runtime dependencies: **336,654 SLoC (not proven)**, for aarch64-unknown-linux-musl.
This is the stage-13 baseline measurement, not a new measurement of this release pass.

Remaining surface, in the order an attacker's bytes meet it:

| # | Piece | State | Decision (Andrew, 2026-10-27) |
|---|---|---|---|
| 1 | TLS protocol: rustls handshake + record layer (~42K lines in the crates; at most 5,095 in our binary, measured in stage 17) | Unproven; sees every byte before our parser. Its crypto primitives are HACL*-verified upstream (libcrux); the protocol logic is not. No production verified TLS for Rust exists today. | **Noted; revisit.** Biggest and hardest gap. **Measured and fuzzed in stage 17:** 2,741 in-binary lines are attacker-facing; 30 minutes per target found nothing ([RUSTLS-SURFACE.md](RUSTLS-SURFACE.md)). |
| 2 | Our connection code: socket reading, buffering, deadlines, response writing, libcrux↔rustls glue | Unproven, first-party, small. | **Attacked in stage 8:** core proven; I/O shell and most provider glue still unverified. |
| 3 | HTTP request parser | **Proven** (Verus, all inputs). | Good. |
| 4 | Application code (routes, later logins, database, JSON, templates) | Trivial today; each future feature is a new untrusted-input surface. | Fine as long as security-relevant code stays measured. |
| 5 | Linux kernel underneath (shared with other containers) | Unproven. Options: appliance VM (own kernel, VM wall); seL4 someday. | **Later.** |
