---
id: cli_update
title: ""
sidebar_label: Update
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_update, dependencies"

# `leo update`

Update dependency versions in `leo.lock` without changing `program.json`.

```bash
leo update
```

To update one dependency, specify its name:

```bash
leo update token.aleo
```

To preview changes without writing the lock file, use `--dry-run`:

```bash
leo update token.aleo --dry-run --network testnet --endpoint https://api.explorer.provable.com/v1
```

Network dependencies without an exact edition in `program.json` can move to the latest edition. Git dependencies that use a branch can move to its current commit. An exact network edition, Git revision, or locked Git tag stays fixed. Change the manifest requirement to select a different exact version.

Leo fetches and validates the selected dependencies before it writes the lock. Network entries record the program identity, network, edition, and checksum. If the bytecode changes for an edition that is already locked, the command fails. It does not replace that edition's checksum.

The first checksum comes from the configured endpoint. It detects later changes to that edition, but it does not prove that the first response was correct.

Normal builds use the locked versions. `leo build --no-cache` downloads locked network editions again and checks their checksums; it does not update them.

## Arguments and flags

### `NAME`

Optional dependency name. Omit the name to update dependencies throughout the current project or workspace, including development dependencies. Packages from the same Git source and reference update together. Other locked dependencies stay fixed unless resolution requires a change.

### `--dry-run`

Show the proposed changes without writing `leo.lock` or `program.json`.

### `--network`

Select the network for network dependencies, such as `testnet` or `mainnet`.

### `--endpoint`

Select the endpoint used to resolve and download network dependencies.

## Update the Leo installation

`leo update` updates dependency locks. Leo does not check for new releases or show release notices. The `--disable-update-check` flag was removed; remove it from existing scripts. To install the latest Leo release and plugins, follow the [installation instructions](https://github.com/ProvableHQ/leo#-build-guide).
