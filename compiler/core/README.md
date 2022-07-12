# leo-core

[![Crates.io](https://img.shields.io/crates/v/leo-ast.svg?color=neon)](https://crates.io/crates/leo-core)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

This directory includes the core library for Leo.

## Usage

Leo's core library is statically built into the compiler.
All modules in the `leo-core` rust module will be automatically included in a `core` Leo module that can be imported into a Leo program.

## Implementations

For a full explanation of core library functions, see the [Aleo developer documentation](https://developer.aleo.org/)

### Core Library (Account)

#### Compute Key

```ts
import core.account.ComputeKey;

function foo(public compute_key: ComputeKey);
function foo(compute_key: ComputeKey);
```

#### Private Key

```ts
import core.account.PrivateKey;

function foo(public private_key: PrivateKey);
function foo(private_key: PrivateKey);
```

#### Record

```ts
import core.account.Record;

function foo(public record: Record);
function foo(record: Record);
```

#### Signature

```ts
import core.account.Signature;

function foo(public signature: Signature);
function foo(signature: Signature);
```

#### View Key

```ts
import core.account.ViewKey;

function foo(public view_key: ViewKey);
function foo(view_key: ViewKey);
```

### Core Library (Algorithms)

#### Poseidon

```ts
import core.algorithms.Poseidon;

function foo(public poseidon: Poseidon);
function foo(poseidon: Poseidon);
```