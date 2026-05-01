---
id: cli_remove
title: ""
sidebar_label: Remove
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_remove, remove_dependency, dependency, dependency_management, imports"

# `leo remove`

To remove a dependency from your project, run the following command:

```bash
leo remove <NAME>
```

where `<NAME>` is the name of the imported program.

See the **[Dependency Management](./../guides/02_dependencies.md)** guide for more details.

## Flags

### `--all`

Removes all dependencies (or dev dependencies, if used with --dev).

### `-dev`

Removes dev dependencies.
