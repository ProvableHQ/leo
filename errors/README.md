# leo-errors

[![Crates.io](https://img.shields.io/crates/v/leo-ast.svg?color=neon)](https://crates.io/crates/leo-errors)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

This directory contains the code for the Errors for all the Leo crates.

## [Common](./src/common)

The common section of this crate contains a few sub files:

- [Backtraced Error](./src/common/backtraced.rs): Which contains the information needed to create a backtraceable error for Leo.
- [Formatted Error](./src/common/formatted.rs): Which contains the information needed to create a formatted error for Leo.
- [Macros](./src/common/macros.rs): Which contains the logic to make creating errors easy through a DSL. It also figures out the error codes for each error via a **top down** method. Meaning all new errors should be added to the bottom of the file. You can specify whether an error is formatted or backtraced through a decorator above a function name, where the formatted ones require a Span as an argument by default. The body takes any additional arguments you want provided to the function, the message, and the optional help message for the error. The additional arguments are just specified to implement traits to avoid as many type conversions in other Leo crates.
- [Span](./src/common/span.rs): Which contains the span object used throughout the other Leo crates (with the exception of the Input crate see more [below](#input)).
- [Tendril JSON](./src/common/tendril_json.rs): Which contains the common logic for how to searlize a StrTendril from the tendril crate.
- [Traits](./src/common/traits.rs): Which contains the common traits in errors to make defining errors easier.

## Error Types

These are the different kinds of errors that are made in this crate. Note that if you want more information about the errors please check the crates documentation or the [Error Index](./ERROR_INDEX.md). All errors here with the exception of [SnarkVM Errors](#snarkvm) have a 037 prefixed to their error codes.

### ASG

The errors for the `leo-asg` crate. Its error codes will range from 3_000-3_999 and be prefixed with the characters `ASG`.

### AST

The errors for the `leo-ast` crate. Its error codes will range from 2_000-2_999 and be prefixed with the characters `AST`.

### CLI

The errors for the `leo-lang` crate. Its error codes will range from 7_000-7_999 and be prefixed with the characters `CLI`.

### Compiler

The errors for the `leo-compiler` crate. Its error codes will range from 6_000-6_999 and be prefixed with the characters `CMP`.

### Import

The errors for the `leo-imports` crate. Its error codes will range from 4_000-4_999 and be prefixed with the characters `IMP`.

### Input

The errors for the `leo-input` crate. For right now, they have an exit code of 1, as they aren't ported.
The reason they aren't ported is due to a desire for 0 deps in case this crate moves to SnarkVM.

### Package

The errors for the `leo-package` crate. Its error codes will range from 5_000-5_999 and be prefixed with the characters `PAK`.

### Parser

The errors for the `leo-parser` crate. Its error codes will range from 0-999 and be prefixed with the characters `PAR`.

### SnarkVM

The errors from SnarkVM that bubble up into Leo in some situations. For right now, they have an exit code of 1.
When SnarkVM implements better error codes and messages, we can bubble them up.

### State

The errors for the `leo-state` crate. Its error codes will range from 1_000-1_999 and be prefixed with the characters `STA`.
