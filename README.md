# OPAML

**OPAML** is a local-first package manager prototype for projects using **Liquid**, **LiquidWeb**, and **Liquidsoap**. It provides a shared manifest, dependency resolution, lockfile, and install interface.

> **Status:** early prototype, not production-ready. It currently installs from a local filesystem registry only; it does not download packages from a remote server.

## Requirements
- Python 3.11+

## Quick start
From the repository root:

```bash
python3 opaml --version
python3 opaml init demo
cd demo
python3 ../opaml add hello --version 1.0.0
python3 ../opaml install
python3 ../opaml list
python3 ../opaml verify
```

The example package works when the OPAML repository's local registry is available at the path configured in the CLI.

## Manifest
`opaml init` creates `opaml.toml`:

```toml
[project]
name = "demo"
version = "0.1.0"
description = "Liquid ecosystem project managed by OPAML"

[dependencies]
# "hello" = "1.0.0"
```

## Local registry
Packages live at `registry/packages/<name>/<version>/` and require a `package.toml`:

```toml
[package]
name = "hello"
version = "1.0.0"

[dependencies]
# another-package = "1.2.0"
```

Exact versions and `*` are supported; `*` selects the newest local version. This prototype does not implement semantic-version ranges or remote downloads.

## Commands
- `opaml init [path]` — create a manifest
- `opaml add <name> [--version VERSION]` — declare a dependency
- `opaml remove <name>` — remove a direct dependency
- `opaml install` — resolve local dependencies, copy package files to `.opaml/packages`, and write `opaml.lock`
- `opaml list` — show direct dependencies
- `opaml verify` — verify lockfile consistency against the current manifest and registry

## Security and roadmap
Package scripts are not executed. Registry packages should still be reviewed before use. Remote registry support, artifact hashes/signatures, semantic-version ranges, cycle diagnostics, and ecosystem-specific build adapters remain future work.

Liquidsoap is an established audio language/toolchain; integration should use its actual supported package/build mechanisms instead of assuming it shares Liquid's runtime.
