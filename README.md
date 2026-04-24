<p align="center">
    <img alt="leo" width="1412" src=".resources/leo-banner.png">
</p>

<h1 align="center">The Leo Programming Language</h1>

<p align="center">
    <a href="https://crates.io/crates/leo-lang">
        <img src="https://img.shields.io/crates/v/leo-lang?style=flat-square"/>
    </a>
    <a href="https://circleci.com/gh/ProvableHQ/leo">
        <img src="https://circleci.com/gh/ProvableHQ/leo.svg?style=svg&circle-token=00960191919c40be0774e00ce8f7fa1fcaa20c00">
    </a>
    <a href="https://discord.gg/aleo">
        <img src="https://img.shields.io/discord/700454073459015690?logo=discord"/>
    </a>
    <a href="https://twitter.com/ProvableHQ">
        <img src="https://img.shields.io/twitter/follow/ProvableHQ?style=social"/>
    </a>
</p>
<div id="top"></div>
Leo is an imperative, statically-typed programming language built for writing private applications.

## <a name='TableofContents'></a>Table of Contents

* [🍎 Overview](#-overview)
* [⚙️️ Build Guide](#-build-guide)
    * [🦀 Install Rust](#-install-rust)
    * [📦 Download using Cargo](#-download-using-cargo)
    * [🐙 Build from Source Code](#-build-from-source-code)
    * [🦁 Update from Leo](#-update-from-leo)
* [🚀 Quick Start](#-quick-start)
* [🧰 Troubleshooting](#-troubleshooting)
* [📖 Documentation](#-documentation)
* [🤝 Contributing](#-contributing)
* [🛡️ License](#-license)


## 🍎 Overview

Welcome to the Leo programming language.

Leo provides a high-level language that abstracts low-level cryptographic concepts and makes it easy to
integrate private applications into your stack. Leo compiles to circuits making zero-knowledge proofs practical.

The syntax of Leo is influenced by traditional programming languages like JavaScript, Scala, and Rust, with a strong emphasis on readability and ease-of-use.
Leo offers developers with tools to sanity check circuits including unit tests, integration tests, and console functions.

Leo is one part of a greater ecosystem for building private applications on [Aleo](https://leo-lang.org/).
The language is currently in an alpha stage and is subject to breaking changes.

## ⚙️️ Build Guide

### 🦀 Install Rust

We recommend installing Rust using [rustup](https://www.rustup.rs/). You can install `rustup` as follows:

- macOS or Linux:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Windows (64-bit):

  Download the [Windows 64-bit executable](https://win.rustup.rs/x86_64) and follow the on-screen instructions.

- Windows (32-bit):

  Download the [Windows 32-bit executable](https://win.rustup.rs/i686) and follow the on-screen instructions.

### 📦 Download using Cargo

If you don't intend to work on the Leo compiler itself, you can install the latest Leo release with:

```bash
cargo install leo-lang leo-fmt
```

Now to use leo, in your terminal, run:
```bash
leo
```

### 🐙 Build from Source Code

If you'd like to install the latest unreleased top of tree Leo, you can build from source code
as follows:

```bash
# Download the source code
git clone https://github.com/ProvableHQ/leo
cd leo

# Install 'leo' and bundled plugins
cargo install --path crates/leo
cargo install --path crates/fmt
```

### 🦁 Update from Leo

You can update Leo and its plugins to the latest released version using the
following command:

```bash
leo update
```

Note that if you were using a prerelease version of Leo, this will overwrite
that with the latest released version.

Now to check the version of leo, in your terminal, run:
```bash
leo --version
```

## 🚀 Quick Start

Use the Leo CLI to create a new project

```bash
# create a new `hello-world` Leo project
leo new helloworld
cd helloworld

# build & setup & prove & verify
leo run main 0u32 1u32
```

The `leo new` command creates a new Leo project with a given name.

The `leo run` command will compile the program into Aleo instructions and run it.

Congratulations! You've just run your first Leo program.

## 🧰 Troubleshooting
If you are having trouble installing and using Leo, please check out our [guide](docs/troubleshooting.md).

If the issue still persists, please [open an issue](https://github.com/ProvableHQ/leo/issues/new/choose).

## 📖 Documentation

* [Leo ABNF Grammar](https://github.com/ProvableHQ/grammars/blob/master/leo.abnf)
* [Homepage](https://docs.leo-lang.org/)

## 🤝 Contributing

Please see our guidelines in the [developer documentation](./CONTRIBUTING.md)

## 🛡️ License
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

<p align="right"><a href="#top">🔼 Back to top</a></p>
