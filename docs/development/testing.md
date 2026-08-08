# Rust testing and coverage policy

This document defines the development-time testing and coverage policy for the ArmaTAK Rust extension.

## Goals

The CI pipeline has two complementary coverage goals:

1. **100% line coverage for deterministic unit-testable code.** Pure protocol parsing, serialization, value normalization, packet construction, configuration descriptions, and other deterministic helpers belong in this gate.
2. **A non-regression floor for the complete Rust extension.** The complete report includes network adapters, mDNS, process management, Arma callbacks, socket loops, and TLS handshakes even when those components require integration or E2E infrastructure rather than unit tests.

The first goal prevents newly added deterministic logic from bypassing unit tests. The second prevents the overall extension from silently losing coverage while integration coverage is expanded.

## Local commands

Run the complete Rust unit test suite with:

```bash
cargo test --all-targets --locked
```

Install `cargo-llvm-cov` and inspect line coverage with:

```bash
cargo llvm-cov --all-targets --locked --summary-only
```

Generate an LCOV file with:

```bash
cargo llvm-cov --all-targets --locked --lcov --output-path lcov.info
```

CI publishes `lcov.info` as the `rust-lcov` artifact after a successful run.

## What belongs in the 100% unit gate

A source file should be included in the deterministic 100% gate when its behavior can be exercised without relying on:

- a live TAK Server;
- a live ATAK client;
- an operating-system socket peer;
- mDNS availability;
- an FFmpeg or external video process;
- timing-sensitive reconnect loops;
- Arma engine callback scheduling;
- real certificate enrollment or a real TLS handshake.

Examples include TAK enrollment URL/config parsing, XML serialization, MAVLink checksums and identity derivation, payload conversion, and diagnostic formatting.

When adding deterministic production code, add the tests in the same change. If a previously excluded file becomes fully unit-testable, bring it to 100% line coverage and remove it from the CI exclusion expression.

## What does not belong in a unit-only 100% gate

External adapters are still part of the full-extension coverage report, but forcing them into a unit-only percentage usually produces brittle or misleading tests. The following behavior should be verified through integration or E2E fixtures instead:

- TCP/UDP connection establishment and reconnect behavior;
- TAK Server certificate enrollment over HTTPS;
- real mTLS certificate-chain and server-name verification;
- mDNS publication/discovery;
- subprocess lifecycle and video streaming;
- Arma engine callback delivery and long-running endpoint threads.

These components must not be hidden from the complete report. Their zero or partial unit coverage remains visible and therefore continues to affect the global non-regression floor.

## Authentication test contract

Authentication protocol assumptions are derived from pinned official upstream implementations. See [authentication-flows.md](authentication-flows.md) for the exact TAK Server and ATAK source revisions.

At minimum, unit tests lock down:

- `/Marti/api/tls/config` construction;
- the v2 `signClient` enrollment path and the official `8089` mTLS default;
- Basic-auth enrollment configuration behavior at the adapter boundary;
- enrollment XML parsing, including missing and malformed tags;
- PEM normalization;
- the rule that diagnostic descriptions never contain enrollment passwords.

A future verified-enrollment implementation must add tests for the trust-bootstrap configuration and integration tests using a real or fixture TLS server. It must never silently downgrade from verified HTTPS to disabled certificate validation.

## Coverage changes

Coverage thresholds are intentionally enforced in CI rather than documented as aspirational numbers. When a change increases the complete-extension baseline, raise the global floor in the workflow in the same pull request or branch before merging. Do not lower either coverage threshold merely to make CI pass; lower thresholds only when a documented architectural change makes the previous metric invalid.
