# OPAML architecture

## Current flow

1. The CLI reads `opaml.toml`.
2. Dependency entries identify exact versions in `registry/packages/<name>/<version>/`.
3. OPAML rejects unsafe path components and symbolic links, hashes each package tree, and copies files into `.opaml/packages/<name>/<version>/`.
4. OPAML writes `opaml.lock` with source paths and SHA-256 digests.
5. `opaml verify` recomputes installed-tree digests and reports missing or modified package trees.

## Planned modules

The initial implementation is intentionally kept in one Rust binary so the data model can settle before crate boundaries are frozen. The target modular design is:

- `opaml-cli`: command parsing and user-facing diagnostics.
- `package-manifest`: manifest schema and validation.
- `dependency-resolver`: version constraints and dependency graph solving.
- `registry-client`: local and remote registry adapters.
- `lockfile`: reproducible resolved package graph.
- `integrity`: hashing, signature verification, and provenance.
- `build-engine`: opt-in, sandboxed build execution.

## Roadmap

1. Stabilize manifest and lockfile formats.
2. Add end-to-end tests for install, verification, and tampering.
3. Add transitive dependency resolution and explicit version semantics.
4. Add remote registries with authenticated metadata and publisher signatures.
5. Add sandboxed, opt-in build tasks and Liquid/LiquidWeb/Liquidsoap integrations.

The roadmap is aspirational; only behavior described as current above should be considered implemented.
