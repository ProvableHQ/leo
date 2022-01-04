# Changelog Template

The format is based on [Rust's Release Notes](https://github.com/rust-lang/rust/blob/master/RELEASES.md),
with the exception of splitting the changelog into multiple files for readability purposes.

## [Unreleased]

### Language

- Aliases
- Arrays of Unspecified Size
- Explicit `const function` definitions
- Countdown loops
- `mut self` renamed to `&self`
- stdlib now exists

### Compiler

- IR
- Recursion
- Error System and Backtraces
- WASM compatible parser

### Library Changes

- Bits/Bytes definitions now in stdlib
- Length function for arrays now in stdlib

### Stabilized APIs

- leo `fetch`

### Internal Changes

- Rust 2021 Edition
- Imports now done before ASG

### Compatibility Notes

- Shadowing disallowed on top level definitions
