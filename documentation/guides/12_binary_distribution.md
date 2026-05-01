---
id: binary-distribution
title: Binary Distribution Reference
sidebar_label: Binary Distribution
---

[general tags]: # "guides, packaging, distribution, releases, binaries"

# Binary Distribution Reference

This page documents Leo's release artifact structure and packaging guidelines. It is intended for community packagers, downstream distributors, and advanced users who want to understand how Leo binaries are published.

For installation instructions, see [Getting Started - Installation](../getting_started/01_installation.md).

## Release Model

Leo uses a per-crate release model. Each publishable crate in the [Leo repository](https://github.com/ProvableHQ/leo) is released independently via git tags matching the pattern `{crate-name}-v{version}` (e.g. `leo-lang-v4.0.1`, `leo-fmt-v1.0.0`).

When a tag is pushed, CI builds cross-platform binaries and publishes them as a GitHub Release under that tag.

:::warning
The release model, artifact naming convention, and `cargo binstall` metadata described on this page apply to versions **after 4.0.2** only. Earlier releases were published in a more ad-hoc manner with inconsistent naming.
:::

## Release URL Pattern

GitHub Releases are published at:

```
https://github.com/ProvableHQ/leo/releases/tag/{crate-name}-v{version}
```

Each release contains one ZIP artifact per supported target, named:

```
{crate-name}-v{version}-{target}.zip
```

For example, `leo-lang-v4.0.1` produces:

- `leo-lang-v4.0.1-x86_64-unknown-linux-gnu.zip`
- `leo-lang-v4.0.1-x86_64-unknown-linux-musl.zip`
- `leo-lang-v4.0.1-x86_64-apple-darwin.zip`
- `leo-lang-v4.0.1-aarch64-apple-darwin.zip`
- `leo-lang-v4.0.1-x86_64-pc-windows-msvc.zip`

## Supported Targets

| Target Triple | OS | Architecture | Notes |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` | Linux | x86_64 | Dynamically linked against glibc |
| `x86_64-unknown-linux-musl` | Linux | x86_64 | Statically linked (Alpine, scratch containers, etc.) |
| `x86_64-apple-darwin` | macOS | Intel | |
| `aarch64-apple-darwin` | macOS | Apple Silicon | |
| `x86_64-pc-windows-msvc` | Windows | x86_64 | |

## Archive Contents

Each ZIP contains all binaries produced by the crate. Current crates and their binaries:

| Crate | Binary | Description |
|---|---|---|
| `leo-lang` | `leo` | The Leo compiler and CLI |
| `leo-fmt` | `leo-fmt` | Leo source code formatter |
| `leo-lsp` | `leo-lsp` | Language server for editor integration |

Future plugin crates will follow the same pattern.

## Plugin Versioning

Plugin crates (`leo-fmt`, `leo-lsp`) are versioned independently from `leo-lang`. Each crate has its own git tag and release cadence, allowing tooling updates to ship without requiring a new compiler release.

When packaging Leo, ensure the installed plugin versions are compatible with the installed `leo-lang` version.

:::note
A machine-readable `releases.toml` manifest is planned to track version compatibility between `leo-lang` and its plugins. Until it is available, consult the [Leo releases page](https://github.com/ProvableHQ/leo/releases) for release notes on compatible version sets.
:::

## Packaging Guidelines

Downstream packages (Homebrew taps, AUR PKGBUILDs, distribution packages, etc.) should follow these guidelines:

- **Install all binaries together.** A complete Leo installation includes `leo`, `leo-fmt`, and `leo-lsp`. Users expect the full toolchain.
- **Place all binaries on `PATH`.** Leo discovers plugins by searching `PATH` for executables matching the `leo-<name>` convention.
- **Preserve the `leo-<name>` naming convention.** Plugin dispatch depends on it - renaming `leo-fmt` to something else will break `leo fmt`.
- **Track compatible versions across crates.** Since crates are released independently, packages should pin to known-compatible version sets.

## `cargo binstall` Support

Each binary crate includes `[package.metadata.binstall]` configuration in its `Cargo.toml`, pointing to the GitHub Release artifact URL template. This means `cargo binstall` resolves the correct platform ZIP automatically:

```bash
cargo binstall leo-lang
cargo binstall leo-fmt
cargo binstall leo-lsp
```

If no pre-built binary is available for the user's platform, `cargo binstall` falls back to building from source via `cargo install`.
