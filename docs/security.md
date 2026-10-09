# OPAML security model

OPAML starts with a conservative local-registry model.

- Package names and versions are validated before being used as path components.
- Package trees containing symbolic links are rejected.
- Installation copies package files; it does not run build hooks, shell scripts, or package-provided commands.
- opaml.lock records the resolved local source and a SHA-256 digest of the package tree.
- opaml verify recalculates installed-tree digests and reports missing or modified packages.

## Integrity is not authenticity

A hash detects changes relative to the lockfile. It does not prove who published a package, and a lockfile must itself come from a trusted source. Signed registry metadata, provenance, network transport policy, and sandboxed build execution are future work.

## Current limitations

The initial implementation uses exact local directory versions as dependency values. It does not yet implement semantic-version range resolution, transitive dependency solving, remote registries, package signatures, or builds. Do not treat it as a drop-in replacement for OPAM.
