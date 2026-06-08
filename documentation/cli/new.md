---
id: cli_new
title: ""
sidebar_label: New
---

[general tags]: # "cli, leo_new, new, new_project"

# `leo new`

To create a new project, run:

```bash
leo new <NAME>
```

To create a new library, run:

```bash
leo new --library <NAME>
```

Valid project and library names are snake_case: lowercase letters and numbers separated by underscores.
This command will create a new directory with the given name.

See [Project Layout](./../language/layout.md) for more details.

When `leo new <NAME>` is run from inside a [workspace](../guides/workspaces.md), the new package is appended to the enclosing `workspace.json` automatically. See [Adding Members](../guides/workspaces.md#adding-members) for the exact behavior.

## Flags

### `--library`

Creates a new Leo library instead of a program. A library provides reusable logic that can be imported by other Leo programs or libraries using `leo add --local`, but cannot be deployed or executed on its own. The generated project includes a `tests/` directory with a starter test file.

### `--workspace`

Creates a workspace skeleton instead of a package. The generated directory contains only a `workspace.json` with an empty `members` array; populate it by listing member paths or globs, or by running `leo new` from within the workspace (see [Workspaces](../guides/workspaces.md)). Conflicts with `--library`.
