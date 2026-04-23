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
   name, verifies the tag version matches `Cargo.toml`, and creates the tag if
   it doesn't already exist (for manual dispatch triggers).
2. **Build** - Compiles the binary for all supported targets.
3. **Release** - Creates a GitHub Release and uploads platform ZIPs.

The workflow is fully idempotent - every job is safe to re-run.

## Re-triggering a Failed Release

If a release fails (build error, infra issue, etc.), re-trigger it from the CLI:

```bash
gh workflow run release-crate.yml -f tag=leo-fmt-v4.0.1
```

Or from the GitHub Actions UI: find the failed run and click **Re-run all jobs**.

The same dispatch mechanism also works as an alternative to `release.sh` - it
creates the tag automatically if it doesn't exist yet.

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
