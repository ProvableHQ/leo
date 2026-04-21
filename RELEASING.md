# Releasing

Binary crates are released independently using per-crate git tags.

## Tag Format

```
{binary-name}-v{version}
```

Examples: `leo-v4.0.1`, `leo-fmt-v1.0.0`, `leo-lsp-v4.0.1`

The tag uses the **binary name** (the name users type to run it), not the Cargo
crate name. The CI workflow maps binary name to crate automatically.

## How to Release

Use the convenience script:

```bash
./scripts/release.sh leo-fmt
```

This reads the version from the crate's `Cargo.toml`, creates a git tag, and
pushes it. The CI workflow handles the rest.

Or tag manually (the script is preferred as it verifies versions and checks for
existing tags):

```bash
git tag leo-fmt-v1.0.0
git push <repo-url> leo-fmt-v1.0.0
```

## What Happens

Pushing a tag matching `*-v[0-9]*` triggers `.github/workflows/release-crate.yml`:

1. **Prepare** - Parses the tag, locates the crate by matching its `[[bin]]`
   name, and verifies the tag version matches `Cargo.toml`.
2. **Build** - Compiles the binary for all supported targets.
3. **Release** - Creates a GitHub Release and uploads platform ZIPs.

## Supported Targets

| Target | Runner |
|--------|--------|
| `x86_64-unknown-linux-gnu` | ubuntu-latest |
| `x86_64-unknown-linux-musl` | ubuntu-latest (docker) |
| `x86_64-apple-darwin` | macos-14-large |
| `aarch64-apple-darwin` | macos-latest |
| `x86_64-pc-windows-msvc` | windows-latest |

## Artifact Naming

Each ZIP contains a single binary at the archive root:

```
{binary-name}-v{version}-{target}.zip
```

Example: `leo-fmt-v1.0.0-x86_64-unknown-linux-gnu.zip` contains `leo-fmt`.

## cargo-binstall

Binary crates include `[package.metadata.binstall]` in their `Cargo.toml`,
enabling fast installation without compiling from source:

```bash
cargo binstall leo-lang
cargo binstall leo-fmt
cargo binstall leo-lsp
```

## Adding a New Binary Crate

No workflow changes needed. Just ensure the new crate has a `[[bin]]` section in
its `Cargo.toml`. Push a tag matching its binary name and the workflow picks it
up automatically.
