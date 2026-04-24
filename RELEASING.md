# Releasing

Binary crates are released independently using per-crate git tags.

## Tag Format

```
{crate-name}-v{version}
```

Examples: `leo-lang-v4.0.1`, `leo-fmt-v1.0.0`, `leo-lsp-v4.0.1`

Tags use the **crate name** from `Cargo.toml` (e.g. `leo-lang`, not `leo`).

## How to Release

From the GitHub Actions UI or CLI:

```bash
gh workflow run release-crate.yml -f tag=leo-fmt-v4.0.1
```

Or use the convenience script, which reads the version from `Cargo.toml`
and creates + pushes the tag:

```bash
./scripts/release.sh leo-fmt
```

## What Happens

Pushing a tag matching `*-v[0-9]*` triggers `.github/workflows/release-crate.yml`:

1. **Prepare** - Parses the tag, locates the crate by matching its package name,
   verifies the tag version matches `Cargo.toml`, and creates the tag if it
   doesn't already exist (for manual dispatch triggers).
2. **Build** - Compiles all binaries from the crate for all supported targets.
3. **Release** - Creates a GitHub Release and uploads platform ZIPs.

The workflow is fully idempotent - every job is safe to re-run.

## Re-triggering a Failed Release

If a release fails (build error, infra issue, etc.), re-trigger it from the CLI:

```bash
gh workflow run release-crate.yml -f tag=leo-fmt-v4.0.1
```

Or from the GitHub Actions UI: find the failed run and click **Re-run all jobs**.

## Supported Targets

| Target | Runner |
|--------|--------|
| `x86_64-unknown-linux-gnu` | ubuntu-latest |
| `x86_64-unknown-linux-musl` | ubuntu-latest (docker) |
| `x86_64-apple-darwin` | macos-14-large |
| `aarch64-apple-darwin` | macos-latest |
| `x86_64-pc-windows-msvc` | windows-latest |

## Artifact Naming

Each ZIP contains the crate's binaries at the archive root:

```
{crate-name}-v{version}-{target}.zip
```

Example: `leo-lang-v4.0.1-x86_64-unknown-linux-gnu.zip` contains `leo`.

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
its `Cargo.toml` and add a `[package.metadata.binstall]` section. Push a tag
matching its crate name and the workflow picks it up automatically.
