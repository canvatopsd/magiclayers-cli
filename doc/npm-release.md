# npm and public source release

Package: `@canvatopsd/magiclayers`. Command/Skill: `magiclayers`. Public repository: `https://github.com/canvatopsd/magiclayers-cli`.

Cargo.toml and package.json must declare the same fixed version. The current native matrix is macOS Apple Silicon and Linux x64/glibc 2.31+. The npm launcher selects only those bundled targets and forwards native stdout, stderr and exit status. There is no postinstall script, runtime download or Rust requirement for npm users.

## Build and inspect

On an Apple Silicon release host:

```sh
npm run build:npm
npm run check:npm
```

The build runs Rust fmt/check/Clippy, builds and checks the host executable, then builds Linux with cargo-zigbuild. The Linux build needs the Rust x86_64-unknown-linux-gnu target, cargo-zigbuild and a Linux sysroot. The build script accepts `MAGICLAYERS_RUST_TOOLCHAIN` and otherwise uses the installed 1.97.1 toolchain; project-local cross-tools, Cargo cache and sysroot live under `.tmp/cache/`. Native `cargo build` requires only standard Rust tools when building for the host.

Output stays under the owning Git repository's `.tmp/build/cli/npm/`. `check:npm` opens the final tarball, enforces its file allowlist, validates native Mach-O/ELF architecture, executable permissions, hashes and versions, compares bundled Skill/docs/contracts, and runs offline native commands through the npm launcher. Credentials, deployment settings, browser profiles and private services are excluded.

## Publish

```sh
npm run release:npm
```

This command follows build → final archive inspection → publish. It publishes the exact inspected tarball to the official npm registry with public access. `-- --dry-run` exercises the same path without publication. Do not overwrite a published version; use a new major.minor.patch version for changed bytes. After publication, compare registry integrity and install the registry package in an isolated prefix before accepting it.

## Source boundaries

This repository is a standalone CLI build, with its public API snapshot under `contracts/`. The product API owns the original contract; an explicit `deployment/sync-contract.mjs SOURCE_JSON` selects a new snapshot before a versioned CLI release. No MagicLayers service code or credentials are imported at runtime. Web/source-installer publication and product-service deployment are separate actions.
