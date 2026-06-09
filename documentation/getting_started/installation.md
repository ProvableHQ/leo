---
id: installation
title: Installation
sidebar: Installation
toc_min_heading_level: 5
toc_max_heading_level: 5
mdx:
  format: mdx
---

[general tags]: # "installation, install_leo"

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

There are a number of ways to install Leo, depending on your platform and preferences. Take your pick!

<Tabs defaultValue="cargo"
values={[
{ label: 'Cargo', value: 'cargo' },
{ label: 'cargo binstall', value: 'binstall' },
{ label: 'Pre-Built Binaries', value: 'prebuilt' },
{ label: 'Build from Source', value: 'source' },
]}>
<TabItem value="cargo">

## Install Cargo

The easiest way to install Cargo is to install the latest stable release of [Rust](https://www.rust-lang.org/tools/install).

## Install Leo

```bash
cargo install leo-lang leo-fmt leo-lsp
```

This will install the `leo`, `leo-fmt`, and `leo-lsp` executables at `~/.cargo/bin/`.
</TabItem>
<TabItem value="binstall">

## Install `cargo-binstall`

[`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) downloads pre-compiled binaries instead of building from source, cutting install time from minutes to seconds.

```bash
cargo install cargo-binstall
```

## Install Leo via binstall

```bash
cargo binstall leo-lang leo-fmt leo-lsp
```

To install a specific version:

```bash
cargo binstall leo-lang@4.1.0 leo-fmt@4.1.0 leo-lsp@4.1.0
```

:::note
`cargo binstall` metadata is only present for versions after 4.0.2. For earlier versions, use `cargo install` instead.
:::

:::tip
If no pre-built binary is available for your platform, `cargo binstall` falls back to building from source automatically.
:::

</TabItem>
<TabItem value="prebuilt">

## Download Pre-Built Binaries

Pre-built binaries are available for every release from [GitHub Releases](https://github.com/ProvableHQ/leo/releases).

Each release publishes a ZIP archive per platform:

| Platform              | Target                       |
| --------------------- | ---------------------------- |
| Linux (x86_64, glibc) | `x86_64-unknown-linux-gnu`   |
| Linux (x86_64, musl)  | `x86_64-unknown-linux-musl`  |
| macOS (Intel)         | `x86_64-apple-darwin`        |
| macOS (Apple Silicon) | `aarch64-apple-darwin`       |
| Windows (x86_64)      | `x86_64-pc-windows-msvc`     |

Archives are named `leo-lang-v{version}-{target}.zip` (e.g. `leo-lang-v4.0.1-aarch64-apple-darwin.zip`).

## Install

1. Download the ZIP for your platform from the [latest release](https://github.com/ProvableHQ/leo/releases).
1. Extract the archive.
1. On macOS/Linux, make the binaries executable and move them onto your PATH:

   ```bash
   chmod +x leo leo-fmt leo-lsp
   mv leo leo-fmt leo-lsp /usr/local/bin/
   ```

1. Verify the installation:

   ```bash
   leo --version
   ```

:::note
Plugin binaries such as `leo-fmt` and `leo-lsp` are released under their own crate tags (e.g. `leo-fmt-v4.1.0`, `leo-lsp-v4.1.0`). Download the matching plugin archives from the same [releases page](https://github.com/ProvableHQ/leo/releases).
:::

</TabItem>
<TabItem value="source">

## Install Rust

Install the latest stable release of **[Rust](https://www.rust-lang.org/tools/install)**. You can verify the installation by running:

```bash
cargo --version
```

## Install Git

Install the latest version of **[Git](https://git-scm.com/downloads)**. You can verify the installation by running:

```bash
git --version
```

## Build Leo from Source Code

```bash
# Download the source code
git clone https://github.com/ProvableHQ/leo
cd leo
# Build and install Leo
cargo install --path crates/leo
# Build and install the formatter plugin
cargo install --path crates/fmt
# Build and install the language server plugin
cargo install --path crates/lsp
```

This will install the `leo`, `leo-fmt`, and `leo-lsp` executables at `~/.cargo/bin/`.

### To use Leo, run

```bash
leo
```

</TabItem>
</Tabs>

---

For distribution maintainers and detailed artifact information, see the [Binary Distribution Reference](../guides/binary_distribution.md).
