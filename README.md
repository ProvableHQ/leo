<p align="center">
    <img width="1412" src="https://cdn.aleo.org/leo/banner.png">
</p>

<h1 align="center">The Leo Programming Language</h1>

<p align="center">
    <a href="https://circleci.com/gh/AleoHQ/leo"><img src="https://circleci.com/gh/AleoHQ/leo.svg?style=svg&circle-token=00960191919c40be0774e00ce8f7fa1fcaa20c00"></a>
    <a href="https://codecov.io/gh/AleoHQ/leo"><img src="https://codecov.io/gh/AleoHQ/leo/branch/testnet3/graph/badge.svg?token=S6MWO60SYL"/></a>
    <a href="https://discord.gg/5v2ynrw2ds"><img src="https://img.shields.io/discord/700454073459015690?logo=discord"/></a>
    <a href="https://github.com/AleoHQ/leo/blob/testnet3/CONTRIBUTORS.md"><img src="https://img.shields.io/badge/contributors-393-ee8449"/></a>
</p>
<div id="top"></div>
Leo is a functional, statically-typed programming language built for writing private applications.

## <a name='TableofContents'></a>Table of Contents

* [🍎 Overview](#-overview)
* [⚙️️ Build Guide](#-build-guide)
    * [🦀 Install Rust](#-install-rust)
    * [🐙 Build from Source Code](#-build-from-source-code)
* [🚀 Quick Start](#-quick-start)
* [🧰 Troubleshooting](#-troubleshooting)
* [📖 Documentation](#-documentation)
* [🤝 Contributing](#-contributing)
* [❤️ Contributors](#-contributors)
* [🛡️ License](#-license)


## 🍎 Overview

Welcome to the Leo programming language.

Leo provides a high-level language that abstracts low-level cryptographic concepts and makes it easy to 
integrate private applications into your stack. Leo compiles to circuits making zero-knowledge proofs practical.

The syntax of Leo is influenced by traditional programming languages like JavaScript, Scala, and Rust, with a strong emphasis on readability and ease-of-use.
Leo offers developers with tools to sanity check circuits including unit tests, integration tests, and console functions.

Leo is one part of a greater ecosystem for building private applications on [Aleo](https://aleo.org/). 
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

### 🐙 Build from Source Code

We recommend installing Leo by building from the source code as follows:

```bash
# Download the source code
git clone https://github.com/AleoHQ/leo
cd leo

# Install 'leo'
$ cargo install --path .
```

Now to use leo, in your terminal, run:
```bash
leo
```

## 🚀 Quick Start

Use the Leo CLI to create a new project

```bash
# create a new `hello-world` Leo project
leo new helloworld
cd helloworld

# build & setup & prove & verify
leo run
```

The `leo new` command creates a new Leo project with a given name.

The `leo run` command will compile the program into Aleo instructions and run it.

Congratulations! You've just run your first Leo program.

## 🧰 Troubleshooting
If you are having trouble installing and using Leo, please check out our [guide](docs/troubleshooting.md).

If the issue still persists, please [open an issue](https://github.com/AleoHQ/leo/issues/new/choose).

## 📖 Documentation

* [Hello World - Next Steps](https://developer.aleo.org/leo/hello)
* [Leo Language Documentation](https://developer.aleo.org/leo/language)
* [Leo ABNF Grammar](https://github.com/AleoHQ/grammars/blob/master/leo.abnf)
* [Homepage](https://developer.aleo.org/overview/)

## 🤝 Contributing
 
Please see our guidelines in the [developer documentation](./CONTRIBUTING.md)


## ❤️ Contributors

View all Leo contributors [here](./CONTRIBUTORS.md).

## 🛡️ License
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

<p align="right"><a href="#top">🔼 Back to top</a></p>
