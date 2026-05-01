---
id: installation
title: Installation
sidebar: Installation
toc_min_heading_level: 5
toc_max_heading_level: 5
---

[general tags]: # "installation, install_leo"

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

There are a number of ways to install Leo, depending on your platform and preferences. Take your pick!

If you'd like to try Leo without installing it locally on your machine, check out the [Leo Playground](./02_ide.md#leo-playground).

<Tabs defaultValue="cargo"
values={[
{ label: 'Cargo', value: 'cargo' },
{ label: 'Pre-Built Binaries', value: 'prebuilt' },
{ label: 'Build from Source', value: 'source' },
]}>
<TabItem value="cargo">

## Install Cargo

The easiest way to install Cargo is to install the latest stable release of [Rust](https://www.rust-lang.org/tools/install).

## Install Leo

```bash
cargo install leo-lang leo-fmt
```

This will install the `leo` and `leo-fmt` executables at `~/.cargo/bin/`.
</TabItem>
<TabItem value="prebuilt">

## MacOS (Apple Silicon)

1. **[Download Leo for Apple Silicon (MacOS)](https://github.com/ProvableHQ/leo/releases/latest/download/leo.zip)**
2. Extract the `.zip` file
3. Open a terminal and navigate to the extracted directory.
4. Run `chmod +x leo leo-fmt` to make the files executable
5. Move both binaries to `/usr/local/bin` to use them system wide.

   ```bash
   mv leo leo-fmt /usr/local/bin
   ```

6. Run `leo --version` to confirm installation

:::note
Release archives now include plugin binaries (such as `leo-fmt`) alongside `leo`. Distribution details are subject to change - see [Leo #29355](https://github.com/ProvableHQ/leo/pull/29355).
:::

## Other Platforms

- **[Browse all Leo releases](https://github.com/ProvableHQ/leo/releases)**

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
cargo install --path .
# Build and install the formatter plugin
cargo install --path crates/fmt
```

This will install the `leo` and `leo-fmt` executables at `~/.cargo/bin/`.

### To use Leo, run

```bash
leo
```

</TabItem>
</Tabs>
