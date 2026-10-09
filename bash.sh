# Create a project
opaml init my-project

# Install packages
opaml add liquid-core
opaml add liquidweb-router
opaml add liquidsoap-audio

# Resolve and install dependencies
opaml install

# Update compatible dependencies
opaml update

# Build or validate the project
opaml build
opaml test

opaml/
├── README.md
├── Cargo.toml
├── crates/
│   ├── opaml-cli/
│   ├── package-manifest/
│   ├── dependency-resolver/
│   ├── registry-client/
│   ├── lockfile/
│   ├── integrity/
│   └── build-engine/
├── registry/
│   └── index/
├── tests/
└── docs/
    ├── architecture.md
    ├── package-format.md
    └── security.md
