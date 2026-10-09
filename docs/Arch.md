# OPAML architecture

## Goal
A shared package-management interface for Liquid, LiquidWeb, and Liquidsoap, with ecosystem-specific integration kept behind adapters.

## Prototype implementation
- Python 3.11+ CLI.
- TOML manifest (opaml.toml) and JSON lockfile (opaml.lock).
- Local filesystem registry at registry/packages/<name>/<version>/.
- Recursive resolution of exact versions and *.
- Package installation to .opaml/packages/; package scripts are never executed.
- Lockfile consistency verification.

## Current limits
This is a prototype, not a production package manager. Version ordering is intentionally simple and is not full semantic-version resolution. The registry is local-only; no network fetches, artifact signatures, checksums, or remote registry authentication exist yet. A future archive installer must defend against path traversal and verify artifacts before extraction.

## Adapters
- **Liquid:** module/package discovery and compiler integration.
- **LiquidWeb:** web dependency and build integration.
- **Liquidsoap:** integrate with its supported tooling and audio workflow rather than assuming a shared runtime.
