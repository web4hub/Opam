# OPAML

**OPAML** is an experimental, safety-first package manager foundation for Liquid ecosystem projects, including Liquid, LiquidWeb, and Liquidsoap tooling.

## Current core

- Rust CLI with `init`, `add`, `remove`, `list`, `install`, and `verify`.
- `opaml.toml` project manifest and deterministic dependency ordering.
- Local package source layout: `registry/packages/<name>/<version>/`.
- Installed package cache: `.opaml/packages/<name>/<version>/`.
- `opaml.lock` with package source and SHA-256 tree digests.
- No package hooks or shell scripts are executed during installation.
- Rejects package-tree symbolic links and validates package path components.

## Quick start

Install Rust (stable), then from this repository:

```sh
cargo test
cargo run -- --help
cargo run -- init my-liquid-project
cd my-liquid-project
cargo run --manifest-path ../Cargo.toml -- add liquid-core 1.0.0
```

For local installation, place a package tree under `registry/packages/<name>/<version>/`, then run OPAML from the project directory with the OPAML executable available on your PATH:

```sh
opaml install
opaml verify
opaml list
```

The local registry is deliberately simple for the first iteration. Remote registry access, dependency graph solving, package signatures, build orchestration, and full SemVer constraints are not implemented yet.

## Manifest example

```toml
[package]
name = "my-liquid-project"
version = "0.1.0"
edition = "2021"

[dependencies]
liquid-core = "1.0.0"
liquidweb-router = "0.4.2"
liquidsoap-audio = "2.0.0"
```

Dependency values currently identify exact local registry directory versions; they are not SemVer ranges.

## Security and scope

Read [docs/security.md](docs/security.md). Hashes detect changes against a trusted lockfile; they do not establish publisher identity or package authenticity. OPAML is an early foundation, not yet a drop-in replacement for upstream OPAM.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Contributions should include tests and avoid claiming behavior that is not implemented.
