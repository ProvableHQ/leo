---
id: abi
title: ABI Generation
sidebar_label: ABI Generation
---

[general tags]: # "abi, build, sdk, tooling, types, lowering, integration"

## Overview

The Leo compiler generates an **Application Binary Interface (ABI)** alongside compiled bytecode. The ABI is a JSON file that describes the public interface of your program, enabling downstream tooling to interact with deployed programs without needing access to the original source code.

**Use cases:**

- SDK generation (Rust, TypeScript, etc.)
- Type-safe transaction construction
- Program introspection and documentation
- Tooling integration (explorers, wallets, IDEs)

## Build Outputs

When you run `leo build`, the compiler generates ABI files alongside the compiled `.aleo` bytecode:

```text
build/
├── main.aleo          # Compiled Aleo bytecode
├── abi.json           # ABI for your program
└── imports/
    ├── foo.aleo       # Imported program bytecode
    └── foo.abi.json   # ABI for imported program
```

- **`build/abi.json`** - ABI for your main program
- **`build/imports/{program}.abi.json`** - ABIs for each imported dependency

ABI generation is automatic on every build - no flags required.

## ABI Format

The ABI is a JSON object with the following top-level structure:

```json title="abi.json"
{
  "program": "token.aleo",
  "structs": [...],
  "records": [...],
  "mappings": [...],
  "storage_variables": [...],
  "functions": [...]
}
```

| Field               | Description                                                         |
| ------------------- | ------------------------------------------------------------------- |
| `program`           | Program identifier (e.g., `"token.aleo"`)                           |
| `structs`           | Struct type definitions used in the public interface                |
| `records`           | Record type definitions                                             |
| `mappings`          | On-chain key-value storage declarations                             |
| `storage_variables` | Storage variable declarations                                       |
| `functions`         | Public entry points (entry `fn` declarations, not helper functions) |

:::info
The ABI only includes types that are referenced by the public interface. Internal helper structs not used in entry functions, mappings, or storage are automatically pruned.
:::

## Type Reference

### Primitives

Primitive types are represented directly:

```json
{ "Primitive": "Address" }
{ "Primitive": "Boolean" }
{ "Primitive": "Field" }
{ "Primitive": "Group" }
{ "Primitive": "Scalar" }
{ "Primitive": "Signature" }
```

Integer types include the signedness:

```json
{ "Primitive": { "Int": "I8" } }
{ "Primitive": { "Int": "I16" } }
{ "Primitive": { "Int": "I32" } }
{ "Primitive": { "Int": "I64" } }
{ "Primitive": { "Int": "I128" } }

{ "Primitive": { "UInt": "U8" } }
{ "Primitive": { "UInt": "U16" } }
{ "Primitive": { "UInt": "U32" } }
{ "Primitive": { "UInt": "U64" } }
{ "Primitive": { "UInt": "U128" } }
```

### Arrays

Fixed-length arrays include the element type and length:

```json
{
  "Array": {
    "element": { "Primitive": "Field" },
    "length": 4
  }
}
```

Nested arrays are supported:

```json
{
  "Array": {
    "element": {
      "Array": {
        "element": { "Primitive": { "UInt": "U32" } },
        "length": 2
      }
    },
    "length": 3
  }
}
```

### Structs

Struct references include a path (supporting modules) and optionally the source program:

```json
{
  "Struct": {
    "path": ["Point"],
    "program": "geometry"
  }
}
```

For structs in modules:

```json
{
  "Struct": {
    "path": ["utils", "Vector3"],
    "program": "geometry"
  }
}
```

Struct definitions include all fields:

```json
{
  "path": ["Point"],
  "fields": [
    { "name": "x", "ty": { "Primitive": { "Int": "I32" } } },
    { "name": "y", "ty": { "Primitive": { "Int": "I32" } } }
  ]
}
```

### Records

Records are similar to structs but include a visibility mode for each field:

```json
{
  "path": ["Token"],
  "fields": [
    { "name": "owner", "ty": { "Primitive": "Address" }, "mode": "None" },
    { "name": "amount", "ty": { "Primitive": { "UInt": "U64" } }, "mode": "Public" },
    { "name": "data", "ty": { "Primitive": "Field" }, "mode": "Private" }
  ]
}
```

**Mode values:**

- `"None"` - Default visibility (private for records)
- `"Constant"` - Publicly visible constant
- `"Private"` - Encrypted, visible only to owner
- `"Public"` - Visible on-chain

### Optional

Optional types (`T?`) are represented as:

```json
{
  "Optional": { "Primitive": "Field" }
}
```

### Mappings

Mappings define on-chain key-value storage:

```json
{
  "name": "balances",
  "key": { "Primitive": "Address" },
  "value": { "Primitive": { "UInt": "U64" } }
}
```

### Storage Variables

Storage variables can be plain values or vectors:

```json
{
  "name": "counter",
  "ty": {
    "Plaintext": { "Primitive": { "UInt": "U32" } }
  }
}
```

```json
{
  "name": "history",
  "ty": {
    "Vector": {
      "Plaintext": { "Primitive": { "UInt": "U64" } }
    }
  }
}
```

### Entry Functions

Entry functions define the public entry points:

```json
{
  "name": "transfer",
  "has_final": false,
  "inputs": [
    {
      "name": "receiver",
      "ty": { "Plaintext": { "Primitive": "Address" } },
      "mode": "Public"
    },
    {
      "name": "amount",
      "ty": { "Plaintext": { "Primitive": { "UInt": "U64" } } },
      "mode": "Public"
    }
  ],
  "outputs": [
    {
      "ty": { "Plaintext": { "Primitive": { "UInt": "U64" } } },
      "mode": "Public"
    }
  ]
}
```

**Input types:**

- `Plaintext` - Primitive, array, struct, or optional
- `Record` - Record input (consumed by the entry function)

**Output types:**

- `Plaintext` - Primitive, array, struct, or optional
- `Record` - Record output (created by the entry function)
- `Final` - Entry function with a `final { }` block returns a `Final`

Entry functions with `final { }` blocks have `has_final: true` and return a `Final`:

```json
{
  "name": "mint_public",
  "has_final": true,
  "inputs": [
    { "name": "receiver", "ty": { "Plaintext": { "Primitive": "Address" } }, "mode": "Public" },
    { "name": "amount", "ty": { "Plaintext": { "Primitive": { "UInt": "U64" } } }, "mode": "Public" }
  ],
  "outputs": [{ "ty": "Final", "mode": "None" }]
}
```

## Type Lowering Specification

The ABI uses **Leo types** (the high-level representation). When interacting with the Aleo VM directly, downstream tooling must apply transformations to understand the on-chain representation.

### Leo to Aleo Type Mapping

Most Leo types map directly to Aleo types:

| Leo Type      | Aleo Type     |
| ------------- | ------------- |
| `address`     | `address`     |
| `bool`        | `boolean`     |
| `field`       | `field`       |
| `group`       | `group`       |
| `scalar`      | `scalar`      |
| `signature`   | `signature`   |
| `i8` - `i128` | `i8` - `i128` |
| `u8` - `u128` | `u8` - `u128` |
| `[T; N]`      | `[T; N]`      |
| `struct Foo`  | `Foo`         |
| `record Bar`  | `Bar.record`  |
| `Final`       | `future`      |

### Optional Lowering

Leo's optional type (`T?`) is lowered to a struct with two fields:

```text
T?  -->  struct { is_some: bool, val: T }
```

**Leo source (a helper `fn` — entry-point fns cannot take an `Optional` directly):**

```leo file=../code_snippets/abi/optional_lowering/src/main.leo#process showLineNumbers
```

**Aleo representation:**

```text
struct "u32?" {
    is_some as boolean;
    val as u32;
}

function process:
    input r0 as "u32?".private;
    // ...
```

When `is_some` is `false`, `val` contains the zero value of the underlying type.

**Nested optional example:**

```leo file=../code_snippets/abi/optional_lowering/src/main.leo#nested_optional showLineNumbers
```

Lowers to an array of structs:

```json
[
    "u64?" { is_some: true, val: 1u64 },
    "u64?" { is_some: false, val: 0u64 }
]
```

### Storage Vector Lowering

Leo's storage vectors (`storage vec: Vector<T>`) are lowered to two mappings:

```text
storage vec: Vector<T>
    -->
mapping vec__: u32 => T        // Elements indexed by position
mapping vec__len__: bool => u32  // Length stored at key `false`
```

**Leo source:**

```leo file=../code_snippets/abi/storage_vector_lowering/src/main.leo showLineNumbers
```

**Aleo representation:**

```text
mapping history__:
    key as u32.public;
    value as u64.public;

mapping history__len__:
    key as boolean.public;
    value as u32.public;
```

To read a storage vector:

1. Get length from `{name}__len__` at key `false`
2. Read elements from `{name}__` at indices `0` to `length - 1`

### Tuple Expansion

Tuples are expanded into multiple registers in Aleo bytecode:

```text
(T1, T2, T3)  -->  r0: T1, r1: T2, r2: T3
```

**Leo source:**

```leo file=../code_snippets/abi/tuple_expansion/src/main.leo showLineNumbers
```

**Aleo bytecode:**

```text
function swap:
    input r0 as u32.private;
    input r1 as u32.private;
    output r1 as u32.private;
    output r0 as u32.private;
```

:::tip
When constructing transactions, tuple inputs/outputs become separate arguments in order.
:::

## Example: Token Program

Here's a complete example showing a Leo program and its generated ABI.

**Leo source (`token.leo`):**

```leo file=../code_snippets/abi/token/src/main.leo showLineNumbers
```

**Generated ABI (`build/abi.json`):**

```json file=../code_snippets/abi/token/build/abi.json title="abi.json"
```

**Key observations:**

- Only `Token` record is included (no internal helper types)
- `mint_public` has a `final { }` block (`has_final: true`) and returns a `Final` in the ABI
- `mint_private` and `transfer_private` return `Record` outputs
- `transfer_private` takes a `Record` input (consuming the token)

## See Also

- [Leo Build Command](./../cli/03_build.md) - CLI reference for building programs
- [Data Types](../language/03_data_types.md) - Leo type system reference
- [The Finalization Model](./01_finalization.md) - Understanding the proof and finalization execution contexts
