---
id: cli_plugins
title: ""
sidebar_label: Plugins
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_plugins, plugins, extensions"

# `leo plugins`

List all CLI plugins currently installed and visible to Leo.

```bash
leo plugins
```

```bash title="console output:"
Installed plugins:

  fmt    /usr/local/bin/leo-fmt
```

If no plugins are found on your PATH:

```bash title="console output:"
No leo plugins detected on PATH.
```

## Plugin System

Leo supports external plugin binaries. Plugins use the `leo-<name>` naming convention.
Leo automatically finds them on your system `PATH`.

When a command does not match a built-in, Leo searches `PATH` for a `leo-<name>` binary and runs it.
Leo sends the arguments without changes. For example:

```bash
leo fmt --check
```

is equivalent to:

```bash
leo-fmt --check
```

If the plugin binary is not found, Leo prints an error:

```text
'leo-fmt' not found. Install the plugin and ensure it is available on your PATH.
```

### Available Plugins

| Plugin    | Crate                                          | Description                            |
| --------- | ---------------------------------------------- | -------------------------------------- |
| `leo-fmt` | [`leo-fmt`](https://github.com/ProvableHQ/leo) | Format Leo source files                |
| `leo-lsp` | [`leo-lsp`](https://github.com/ProvableHQ/leo) | Language server for editor integration |

### Installing Plugins

Plugins can be installed via `cargo install` or `cargo binstall`:

```bash
cargo install leo-fmt leo-lsp
# or, to download pre-built binaries:
cargo binstall leo-fmt leo-lsp
```

Pre-built binaries are also available from [Leo releases](https://github.com/ProvableHQ/leo/releases). Each plugin crate is released independently under its own git tag (for example `leo-fmt-v4.1.0`), so a plugin's version may differ from `leo-lang`.

For details on release artifacts, target platforms, and packaging guidelines, see the [Binary Distribution Reference](../guides/binary_distribution.md).

Running `leo update` will also attempt to update bundled plugins like `leo-fmt` on a best-effort basis.

### Writing Custom Plugins

Leo identifies each `leo-<name>` executable on `PATH` as a plugin.
You can use any language to write a custom plugin. The requirements are:

1. The binary is named `leo-<name>` (for example `leo-mytools`)
2. The binary is located in a directory on your `PATH`
3. The binary is executable
