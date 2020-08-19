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
* [4. Flying Tour](#4-flying-tour)
    * [4.1 Functions](#41-functions)
    * [4.2 Testing](#42-testing)
    * [4.3 Data Types](#43-data-types)
    * [4.4 Circuits](#44-circuits)
    * [4.5 Imports](#45-imports)
* [5. Contributing](#5-contributing)
* [6. License](#6-license)


## 1. Overview
Welcome to the Leo programming language.

Leo exists to provide a simple high-level language that compiles to a rank one constraint system (R1CS) circuit. 
With Leo, you can write circuits to support zero-knowledge tokens, private stable coins, and decentralized marketplaces.

The syntax of Leo is influenced by JavaScript, Python, Scala, and Rust with a strong emphasis on readability and ease-of-use.

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
cargo install leo
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

This will generate an executable under the `./target/release` directory. To run snarkOS, run the following command:
```bash
./target/release/leo
```

## 3. Quick Start

Use the Leo CLI to create a new project

```bash
# create a new `hello_world` Leo project
leo new hello_world
cd hello_world

# build & setup & prove & verify
leo run
```

The `leo new` command creates a new Leo project with a given name.

The `leo run` command will compile the main program, generate keys for a trusted setup, fetch inputs, generate a proof and verify it.

Congratulations! You've just run your first Leo program.

## 4. Flying Tour

The best way to get to know Leo is by writing some code. We will fly through a high level overview of a Leo file.
To gain a deeper understanding of the Leo language, then check out the [developer documentation](https://developer.aleo.org/developer/getting_started/overview)


**Square Root Example**: Let's prove that we know the square root of a number.

**`src/main.leo`**
```rust // change this to leo
function main(a: u32, b: u32) -> bool {
    return square_root(a, b)
}

function square_root(a: u32, b: u32) -> bool {
    return a * a == b
}

test function test_square_root() {
    let a: u32 = 5;
    let b: u32 = 25;
    
    let result = square_root(a, b);

    console.assert(result == true);
}
```

### 4.1 Functions
The `main` function is the entrypoint of a Leo program. 
`leo run` will provide private inputs directly to the function for proving and store the program result in an output file.

The `square_root` function is called by `main` with private inputs `a` and `b` which are both unsigned `u32` integers.

### 4.2 Testing

A naive way to test `square_root` would be to execute `leo run` several times on different inputs and check the output of the program each time.

Luckily, we can write unit tests in Leo using the `test function` syntax. 
In `test_square_root` we can sanity check our code without having to load in private inputs from a file every time. 
Want to upgrade your test function into an integration test? 
In Leo you can add a test context annotation that loads different sets of private inputs to make your test suite even more robust.

The last line of `test_square_root` uses the console function `console.assert`. 
This function along with `console.log`, `console.debug`, and `console.error` provide developers with tools that are run without
affecting the underlying constraint system. 

### 4.3 Data Types

Leo supports boolean, unsigned integer, signed integer, field, group element, and address data types.
Collections of data types can be created in the form of static arrays and tuples.

### 4.4 Circuits

**Circuits Example**

**`src/main.leo`**
```rust
circuit Point {
    x: u32,
    y: u32,

    static function new() -> Self {
        return Self { 
            x: 0, 
            y: 0, 
        }
    }

    function add() -> u32 {
        return self.x + self.y
    }
}

function main() {
    let mut p = Point::new();
    
    p.x = 4u32;
    p.y = 6u32;

    let sum = p.add();
    
    console.log("The sum is {}", sum);
}
```

Circuits in leo are similar to structures in other object-oriented languages. 
They provide a composite data type that can store primitive values and provide functions for instantiation and computation.

The `static` keyword modifies the `new` function so it can be called without instantiating the circuit.

Leo introduces `Self` and `self` keywords to access circuit member values.

### 4.5 Imports

Imports fetch other circuits and functions and bring them into the current file scope. 
Leo supports imports for dependencies that are declared locally or in an imported package.

Importing packages can be accomplished using the `leo add` command in the CLI.

## 5. Contributing
 
Please see our guidelines in the [developer documentation](https://developer.aleo.org/developer/additional_material/contributing)

Thank you for helping make Leo better!


## 6. License 
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)