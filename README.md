<p align="center">
    <img width="1412" src="https://cdn.aleo.org/leo/banner.png">
</p>

<h1 align="center">The Leo Programming Language</h1>

<p align="center">
    <a href="https://github.com/AleoHQ/leo/actions"><img src="https://github.com/AleoHQ/leo/workflows/CI/badge.svg"></a>
    <a href="https://codecov.io/gh/AleoHQ/leo"><img src="https://codecov.io/gh/AleoHQ/leo/branch/master/graph/badge.svg?token=S6MWO60SYL"/></a>
    <a href="https://app.bors.tech/repositories/31738"><img src="https://bors.tech/images/badge_small.svg" alt="Bors enabled"></a>
    <a href="https://discord.gg/TTexWvt"><img src="https://img.shields.io/discord/700454073459015690?logo=discord"/></a>
</p>

Leo is a functional, statically-typed programming language built for writing private applications.

## <a name='TableofContents'></a>Table of Contents

* [1. Overview](#1-overview)
* [2. Build Guide](#2-build-guide)
    * [2.1 Install Rust](#21-install-rust)
    * [2.2a Build from Crates.io](#22a-build-from-cratesio)
    * [2.2b Build from Source Code](#22b-build-from-source-code)
* [3. Quick Start](#3-quick-start)
* [4. Documentation](#4-documentation)
* [5. Contributing](#5-contributing)
* [6. License](#6-license)


## 1. Overview

Welcome to the Leo programming language.

Leo provides a high-level language that abstracts low-level cryptographic concepts and makes it easy to 
integrate private applications into your stack. Leo compiles to circuits making zero-knowledge proofs practical.

The syntax of Leo is influenced by traditional programming languages like JavaScript, Scala, and Rust, with a strong emphasis on readability and ease-of-use.
Leo offers developers with tools to sanity check circuits including unit tests, integration tests, and console functions.

Leo is one part of a greater ecosystem for building private applications on [Aleo](https://aleo.org/). If your goal is to build a user experience
on the web that is both truly personal and truly private, then we recommend downloading the [Aleo Studio IDE](https://aleo.studio/)
and checking out the [Aleo Package Manager](https://aleo.pm/).

## 2. Build Guide

### 2.1 Install Rust

We recommend installing Rust using [rustup](https://www.rustup.rs/). You can install `rustup` as follows:

- macOS or Linux:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Windows (64-bit):  
  
  Download the [Windows 64-bit executable](https://win.rustup.rs/x86_64) and follow the on-screen instructions.

- Windows (32-bit):  
  
  Download the [Windows 32-bit executable](https://win.rustup.rs/i686) and follow the on-screen instructions.

### 2.2a Build from Crates.io

We recommend installing Leo this way. In your terminal, run:

```bash
cargo install leo-lang
```

Now to use Leo, in your terminal, run:
```bash
leo
```
 
### 2.2b Build from Source Code

Alternatively, you can install Leo by building from the source code as follows:

```bash
# Download the source code
git clone https://github.com/AleoHQ/leo
cd leo

# Build in release mode
$ cargo build --release
```

This will generate an executable under the `./target/release` directory. To run Leo, run the following command:
```bash
./target/release/leo
```

## 3. Quick Start

Use the Leo CLI to create a new project

```bash
# create a new `hello-world` Leo project
leo new hello-world
cd hello-world

# build & setup & prove & verify
leo run
```

The `leo new` command creates a new Leo project with a given name.

The `leo run` command will compile the main program, generate keys for a trusted setup, fetch inputs, generate a proof and verify it.

Congratulations! You've just run your first Leo program.

## 4. Documentation

* [Hello World - Next Steps](https://developer.aleo.org/developer/getting_started/hello_world)
* [Leo Language Documentation](https://developer.aleo.org/developer/language/layout)
* [Leo CLI Documentation](https://developer.aleo.org/developer/cli/new)
* [Homepage](https://developer.aleo.org/developer/getting_started/overview)

## 5. Contributing
 
Please see our guidelines in the [developer documentation](https://developer.aleo.org/developer/additional_material/contributing)

Thank you for helping make Leo better!

## 6. License 
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)
