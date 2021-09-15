# leo-stdlib

[![Crates.io](https://img.shields.io/crates/v/leo-ast.svg?color=neon)](https://crates.io/crates/leo-ast)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

This directory includes the standard library for Leo.

## Usage

The src directory is the Rust code that makes the Leo folders and files statically built into the compiler. So any added Leo folders and files under the `stdlib` directory will be automatically included at compile time.

See the [Structure](#structure) section for more info.

## Structure

The structure for this repository is a bit special.

One important thing to note is the prelude directory. Any Leo files defined in this directory will have all importable objects automatically imported to every Leo program. For example, the following program is valid without any imports:

```typescript
function main() {
  let s: string = "Hello, string type alias!";
}
```

The above type alias is auto imported from `stdlib/prelude/string.leo`.

The other directories must have explicit imports. For example, the unstable Blake2s can be imported with `import std.unstable.blake2s.Blake2s`. Which imports the `Blake2s` circuit defined in `stdlib/unstable/blake2s.leo`.
