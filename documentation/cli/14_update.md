---
id: cli_update
title: ""
sidebar_label: Update
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_update, versioning"

# `leo update`

To download and install the latest Leo version run:

```bash
leo update
```

```bash title="console output:"
Checking target-arch... aarch64-apple-darwin
Checking current version... v3.1.0
Checking latest released version... v3.1.0
       Leo
Leo is already on the latest version
```

`leo update` also updates bundled plugins (such as `leo-fmt`) on a best-effort basis.

If you'd like to install a specific version of Leo, you can do so by passing the `--name` flag:

```bash
leo update --name v3.0.0
```

## Flags

### `--list`

### `-l`

Lists all available versions of Leo.

### `--name`

### `-n`

An optional release name if you wish to install a specific version of Leo. By default, the command will look for the latest release.
