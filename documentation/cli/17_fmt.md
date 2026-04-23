---
id: cli_fmt
title: ""
sidebar_label: Fmt
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "cli, leo_fmt, fmt, format, formatting"

# `leo fmt`

Format Leo source files, automatically formatting all `.leo` files in your project.

```bash
leo fmt
```

By default, `leo fmt` formats all `.leo` files in the `src/` directory. There is no output on success.

You can also format specific files or directories:

```bash
leo fmt src/main.leo
```

## Check Mode

To check if files are formatted without modifying them, use the `--check` flag. This prints a colored diff of any unformatted files and exits with code 1 if changes are needed:

```bash
leo fmt --check
```

![leo fmt --check](../img/leo_fmt_check.png)

### Flags:

#### `--check`

#### `-c`

Check if files are formatted without modifying them. Prints a diff and exits with code 1 if any files need formatting.

#### `[PATH]...`

Files or directories to format. Defaults to the project `src/` directory if not specified.
