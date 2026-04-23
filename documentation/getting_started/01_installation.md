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
cargo install leo-lang
```

This will generate the executable at `~/.cargo/bin/leo`.
</TabItem>
<TabItem value="prebuilt">

## MacOS (Apple Silicon):

1.  **[Download Leo for Apple Silicon (MacOS)](https://github.com/ProvableHQ/leo/releases/latest/download/leo.zip)**
2.  Extract the `.zip` file
3.  Open a terminal and navigate to the extracted directory.
4.  Run `chmod +x leo` to make the file executable
5.  Move `leo` to `/usr/local/bin` to use it system wide.

        mv leo /usr/local/bin

6.  Run `leo --version` to confirm installation

## Other Platforms:

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
# Build and install
cargo install --path .
```

This will generate the executable at `~/.cargo/bin/leo`.

#### To use Leo, run:

```bash
leo
```

</TabItem>
</Tabs>
