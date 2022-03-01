# Leo WASM

<!-- [![Crates.io](https://img.shields.io/crates/v/leo-wasm.svg?color=neon)](https://crates.io/crates/leo-wasm) -->
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md) 

This directory contains WASM bindings for the Leo compiler.

## Limitations

Currently, WASM target of the compiler supports parsing and canonicalization stages.

## API

This is a list of the supported methods and their signatures.

### leo.parse

Method takes in a Leo program as string and returns JSON string with the resulting AST or throws a LeoError.

```ts
export interface LeoError {
    text: string,    // Full error text (including span)
    code: string,    // Leo error identifier (e.g. "EPAR0370005")
    exitCode: number // Exit code for an error (e.g. 370005)
}

/**
 * @param {String} program Leo program text to parse and produce AST
 * @return {String} Resulting AST as a JSON string.
 * @throws {LeoError} When program contains invalid Leo code.
 */
export function parse(program: string): string;
```
