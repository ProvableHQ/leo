---
id: cli_clean
title: ""
sidebar_label: Clean
---

[general tags]: # "cli, leo_clean, clean"

# `leo clean`

To clean the build directory, run:

```bash
leo clean
```

```bash title="console output:"
  Leo 🧹 Cleaned the build directory (in "...")
```

## Workspace Behavior

When run inside a [workspace](../guides/workspaces.md):

- **From workspace root:** Cleans build artifacts for all members.
- **From a member directory:** Cleans only that member.
- **With `--package <NAME>`:** Cleans only the specified member.

```bash
# Clean all workspace members
leo clean

# Clean only the swap member
leo clean -p swap
```
