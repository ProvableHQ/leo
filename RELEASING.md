# Releasing

Releases are crates.io first. Version bumps are merged to `master`, then
`.github/workflows/publish-crates.yml` uses release-plz to publish workspace
crate versions that are not already on crates.io. Binary crate tags then drive
`.github/workflows/release-crate.yml`, which builds GitHub release artifacts.

## Normal Release Path

Before releasing, confirm the compatible snarkOS version in
`.resources/snarkos-version`:

```text
<compatible-snarkos-version>
```

This is the single checked-in source for snarkOS release compatibility. CI and
release metadata derive the snarkOS release tag and download URLs from it.

To release:

1. Bump the crate versions that should be released.
2. Update `Cargo.lock` and merge the change to `master`.
3. Let `.github/workflows/publish-crates.yml` publish unpublished crate versions
   and create package tags.
4. Let the same workflow dispatch `.github/workflows/release-crate.yml` for
   released crate tags; non-binary crates are skipped there.

Manual crate publishing is normally unnecessary. To rerun the publish workflow:

```bash
gh workflow run publish-crates.yml --ref master
```

To check what release-plz would publish without publishing or tagging:

```bash
gh workflow run publish-crates.yml --ref master -f dry_run=true
```

## Trusted Publishing

The publish workflow uses release-plz with crates.io Trusted Publishing. It does
not use `rust-lang/crates-io-auth-action` and should not set a
`CARGO_REGISTRY_TOKEN` secret. release-plz requests a short-lived crates.io token
through GitHub OIDC when it needs to publish.

Ask the crate owners to configure each already-published crate on crates.io with
a Trusted Publisher:

- provider: GitHub Actions
- owner: `ProvableHQ`
- repository: `leo`
- workflow: `publish-crates.yml`

The publish workflow requests `contents: write`, `actions: write`,
`pull-requests: read`, and `id-token: write`. The `id-token: write` permission
allows release-plz to request a GitHub OIDC token for crates.io Trusted
Publishing.

Trusted Publishing cannot publish a brand-new crate name for the first time.
New crates need the bootstrap steps in
[Adding a New Publishable Crate](#adding-a-new-publishable-crate) before this
workflow can publish later versions.

## Why release-plz

Cargo now supports multi-package publishing with `cargo publish --workspace` and
repeated `-p` flags, but Cargo does not provide an idempotent "publish only
unpublished versions" mode or a separate publish plan command. `cargo publish
--dry-run` verifies packaging without uploading; it does not answer which
workspace packages are missing from crates.io.

release-plz fills that gap: `release-plz release` publishes all unpublished
packages and does nothing when everything is already published. It still uses
Cargo for the actual publish operation.

## Tag Format

Release tags use:

```text
{crate-name}-v{version}
```

Examples: `leo-lang-v4.0.1`, `leo-fmt-v1.0.0`, `leo-lsp-v4.0.1`.

Tags use the crate name from `Cargo.toml` (for example, `leo-lang`, not `leo`).
release-plz creates package tags for published crates. `release-crate.yml` runs
for crate tags matching `*-v[0-9]*`; crates without binary targets are validated
and then skipped without building GitHub release artifacts.

## Backfills and Reruns

If a binary crate is already published on crates.io but its GitHub release tag or
artifacts are missing, dispatch `release-crate.yml` manually from the GitHub
Actions UI or CLI:

```bash
gh workflow run release-crate.yml --ref master -f tag=leo-fmt-v4.0.1
```

`release-crate.yml` creates the tag if it is missing after validating that the
tag version matches the crate manifest on the workflow ref.

The compatibility script can also be run locally:

```bash
# Requires cargo and jq on PATH.
bash scripts/generate-release-metadata.sh leo-lsp-v4.0.2 /tmp/leo-release
```

## What the Workflows Do

`publish-crates.yml`:

1. Runs release-plz on `master`.
2. Publishes crate versions that are not already on crates.io.
3. Creates package tags while disabling release-plz GitHub release creation.
4. Dispatches `release-crate.yml` for released crate tags.

`release-crate.yml`:

1. Parses `{crate-name}-v{version}`.
2. Checks out the tag if it exists, or uses the workflow ref for missing-tag
   backfills.
3. Validates that the tag version matches the crate manifest.
4. Skips cleanly if the crate has no binary targets.
5. Creates the tag if it was missing.
6. Builds all binaries from that crate for supported targets.
7. Generates release notes and `leo-release.toml`.
8. Creates or updates the GitHub Release for that tag.

Both workflows are idempotent. release-plz skips already-published crate
versions, and GitHub release artifact generation is safe to rerun.

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

```text
{crate-name}-v{version}-{target}.zip
```

Example: `leo-lang-v4.0.1-x86_64-unknown-linux-gnu.zip` contains `leo`.

## Release Notes and Metadata

Each GitHub Release includes:

- a `Compatible Versions` table for `leo-lang`, `leo-fmt`, `leo-lsp`,
  `snarkvm`, and `snarkOS`
- GitHub-generated release notes since the previous same-crate tag, when a
  previous same-crate tag exists
- a first-release note when there is no previous same-crate tag
- a `leo-release.toml` asset for downstream packagers

`leo-release.toml` contains:

- `[release]` with the released crate, version, tag, commit, repository, and
  supported targets
- `[components.leo-lang]`, `[components.leo-fmt]`, and `[components.leo-lsp]`
  with versions, tags, crates.io URLs, release URLs, archive URL templates, and
  binary names
- `[components.snarkvm]` with the exact version resolved in `Cargo.lock`
- `[components.snarkos]` derived from `.resources/snarkos-version`

## cargo-binstall

Binary crates include `[package.metadata.binstall]` in their `Cargo.toml`,
enabling fast installation without compiling from source:

```bash
cargo binstall leo-lang
cargo binstall leo-fmt
cargo binstall leo-lsp
```

## Adding a New Publishable Crate

Before relying on the automated publishing workflow for a new crate, bootstrap
the crate on crates.io:

1. Add the crate to the workspace with the intended package name and metadata.
2. Publish the first version manually with `cargo publish -p <crate-name>`.
3. Configure crates.io Trusted Publishing for the crate:
   - provider: GitHub Actions
   - owner: `ProvableHQ`
   - repository: `leo`
   - workflow: `publish-crates.yml`
4. Confirm the crate owner set includes the maintainers who can adjust crates.io
   owner and Trusted Publishing settings.

After the first version exists on crates.io and Trusted Publishing is
configured, future version bumps merged to `master` are handled by
`publish-crates.yml`.

## Adding a New Binary Crate

For crates that should also produce GitHub release artifacts, ensure the crate
has a `[[bin]]` section in its `Cargo.toml` and add a
`[package.metadata.binstall]` section. No workflow changes are needed once the
crate has completed the publishable-crate bootstrap steps above.
