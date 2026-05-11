---
id: cli_add
title: ""
sidebar_label: Add
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_add, add_dependency, dependency, dependency_management, imports"

# `leo add`

The `leo add` command is used to add a new on-chain, local, or workspace dependency to the current Leo project.

To add a local dependency to your project, run the following command:

```bash
leo add --local <LOCAL> <NAME>
```

where `<NAME>` is the name of the imported program or library, and `<LOCAL>` is the path to the local project or library.

&nbsp;

To add a program already deployed onchain as a dependency to your project, run the following command:

```bash
leo add --network <NAME>
```

where `<NAME>` is the name of the imported program.

&nbsp;

To add another member of the enclosing workspace as a dependency:

```bash
leo add --workspace <NAME>
```

where `<NAME>` is the directory name or program name of the workspace member.

:::note
Libraries can only be added as local dependencies. Use `--local` to add a library.
:::

## Flags

### `--local <LOCAL>`

### `-l <LOCAL>`

Specifies that the dependency to be added is a local program or library located at path `<LOCAL>`. This can be the root directory for a Leo project, the root directory for a Leo library, or a path directly to an already compiled `.aleo` file.

### `--network`

### `-n`

Specifies that the dependency to be added is a remote program currently deployed onchain. The network that it will be pulled from will be the same as the one specified in by the `NETWORK` variable in `.env`

### `--workspace`

### `-w`

Specifies that the dependency is another member of the enclosing [workspace](../guides/03_workspaces.md). Leo validates that a `workspace.json` exists in a parent directory and that the named member is listed in it. No path is required - Leo resolves the member's location automatically.

### `--edition <EDITION>`

### `-e <EDITION>`

Specifies the expected edition of the program being imported. Only passing this flag will assume that the program is being imported from the network.

:::warning
Do not use this feature unless you know what you are doing!
:::

### `--dev`

Specifies that the imported program is a development dependency and should not be used in production

### `--clear`

### `-c`

Clears all previous dependencies.

:::warning
This feature is currently bugged and is non-functional.
:::
