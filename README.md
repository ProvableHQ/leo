# The Leo Programming Language

![CI](https://github.com/AleoHQ/leo/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/AleoHQ/leo/branch/master/graph/badge.svg?token=S6MWO60SYL)](https://codecov.io/gh/AleoHQ/leo)

Leo is a functional, statically-typed programming language built for writing private applications.

## <a name='TableofContents'></a>Table of Contents

* [1. Overview](#1-overview)
* [2. Build Guide](#2-build-guide)
    * [2.1 Install Rust](#21-install-rust)
    * [2.2a Build from Crates.io](#22a-build-from-cratesio)
    * [2.2b Build from Source Code](#22b-build-from-source-code)
* [3. Quick Start](#3-quick-start)
    * [3.1 Zero Knowledge in One Line](#31-zero-knowledge-in-one-line)
* [4. Flying Tour](#4-flying-tour)
* [5. License](#5-license)


## 1. Overview
Welcome to the Leo programming language.

Leo exists to provide a simple high-level language that compiles to a rank one constraint system (R1CS) circuit. With Leo, you can write circuits to support zero-knowledge tokens, private stable coins, and decentralized marketplaces.

The syntax of Leo is influenced by JavaScript, Python, Scala, and Rust with a strong emphasis on readability and ease-of-use.

# 2. Build Guide

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
cargo install leo
```

Now to use Leo, in your terminal, run:
```bash
leo
```
 
### 2.2b Build from Source Code

Alternatively, you can install snarkOS by building from the source code as follows:

```bash
# Download the source code
git clone https://github.com/AleoHQ/leo
cd leo

# Build in release mode
$ cargo build --release
```

This will generate an executable under the `./target/release` directory. To run snarkOS, run the following command:
```bash
./target/release/leo
```

# 3. Quick Start

Use the Leo CLI to create a new project

```bash
leo new hello_world
cd hello_world
```

This creates a directory with the following structure:
```bash
hello_world/
├── Leo.toml # Your program manifest
├── inputs/ 
│ └── hello_world.in # Your program inputs
└── src/    
  └── main.leo # Your program file
```

Let's run the project.

## 3.1 Zero Knowledge in one line

```bash
leo run
```
This command will compile the program, generate keys for a trusted setup, fetch inputs, generate a proof and verify it.

Congratulations! You've just run your first Leo program.


# 4. Flying Tour

WIP

# 5. License 
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)