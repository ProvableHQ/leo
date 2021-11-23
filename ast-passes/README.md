# leo-ast-passes

[![Crates.io](https://img.shields.io/crates/v/leo-ast.svg?color=neon)](https://crates.io/crates/leo-ast)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

## Usage

The code here is split into several usages. Each usage represents a different pass or modification when given an AST.

### Canonicalization

This pass of the code has a few changes it must complete:

- `Self` is not allowed outside a struct.
- `Self` in structs must be replaced with an Identifier containing the Struct Name.
- Any 0 size array definitions should be rejected.
- Multi-size array definitions should be expanded such that `[0u8; (2, 3)]` becomes `[[0u8; 3] 2]`.
- Compound assignments become simple assignments such that `a += 2;` becomes `a = a + 2;`.
- Missing function output types are replaced with an empty tuple.

### Import Resolution

This pass iterates through the import statements(nestedly), resloving all imports. Thus adding the improted file's AST to the main AST.

In addition, it also handles forcibly importing the stdlib prelude files.

## Structure

Each different type of pass is located in its own directory within the src directory.
