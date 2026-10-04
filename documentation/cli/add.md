---
id: cli_add
title: ""
sidebar_label: Add
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_add, add_dependency, dependency, dependency_management, imports"

# `leo add`

The `leo add` command is used to add a new on-chain, local, workspace, or git dependency to the current Leo project.

To add a local dependency to your project, run the following command:

```bash
leo add --local <LOCAL> <NAME>
```

where `<NAME>` is the name of the imported program or library, and `<LOCAL>` is the path to the local project or library.

To add a program already deployed on-chain as a dependency to your project, run the following command:

```bash
leo add <NAME> --onchain
```

where `<NAME>` is the name of the imported program. To select the verification network explicitly, use `leo add <NAME> --onchain --network <NETWORK>`. If neither `--network` nor `NETWORK` is set, Leo uses `testnet`.

To add another member of the enclosing workspace as a dependency:

```bash
leo add --workspace <NAME>
```

where `<NAME>` is the directory name or program name of the workspace member.

To add a dependency from a git repository:

```bash
leo add --git <URL> <NAME>
```

where `<NAME>` is the name of the imported program or library, and `<URL>` is the repository URL. Optionally pin to a branch, tag, or revision with `--branch`, `--tag`, or `--rev`.

:::note
Libraries can only be added as local or git dependencies. Use `--local` or `--git` to add a library.
:::

## Flags

### `--local <LOCAL>`

### `-l <LOCAL>`

Specifies a local program or library dependency at `<LOCAL>`. The path can be a Leo project root, a Leo library root, or a compiled `.aleo` file.

### `--onchain`

### `-n`

Specifies that the dependency is a program deployed on-chain. Leo verifies that the program exists before it changes `program.json`.

### `--network <NETWORK>`

Specifies the network that Leo uses to verify and fetch an on-chain dependency. This option overrides the `NETWORK` environment variable. If neither is set, Leo uses `testnet`. Leo also uses the selected network for cache lookup and does not store it in `program.json`.

### `--endpoint <ENDPOINT>`

Use this option to specify the endpoint that Leo uses to verify a network dependency. This option overrides the `ENDPOINT` environment variable.

### `--network-retries <N>`

Use this option to specify how many times Leo retries a network request. This option overrides the `NETWORK_RETRIES` environment variable. The default value is `2`.

### `--workspace`

### `-w`

Specifies that the dependency is another member of the enclosing [workspace](../guides/workspaces.md). Leo validates that a `workspace.json` exists in a parent directory and that the named member is listed in it. No path is required - Leo resolves the member's location automatically.

### `--git <URL>`

### `-g <URL>`

Specifies that the dependency is fetched from the git repository at `<URL>` (a Leo program, a Leo library, or a compiled `.aleo` file). Leo clones the repository to read its manifest and auto-detect the package kind, and records the resolved commit in `leo.lock`. See [Git Dependencies](../guides/dependencies.md#git-dependencies).

### `--branch <BRANCH>` / `--tag <TAG>` / `--rev <REV>`

Pin a git dependency to a specific branch, tag, or revision. These require `--git`, and at most one may be given. When none is specified, the repository's default branch is tracked.

### `--edition <EDITION>`

### `-e <EDITION>`

Specifies the expected edition of an on-chain program. This option selects the on-chain source by itself, so do not combine it with `--onchain`. You can combine it with `--network <NETWORK>`.

:::warning
Do not use this feature unless you know what you are doing!
:::

### `--dev`

Specifies that the imported program is a development dependency and should not be used in production
