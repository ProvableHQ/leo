---
id: structure
title: Structure of a Leo Program
sidebar_label: Program Structure
---

[general tags]: # "program, constant, import, record, struct, mapping"

## Layout of a Leo Program

A Leo program contains declarations of a [Program](#program), [Constants](#constant), [Imports](#import)
, [Structs](#struct), [Records](#record), [Mappings](#mappings), [Interfaces](./programs_in_practice/interfaces.md), and functions.
Declarations are locally accessible within a program file.
If you need a declaration from another Leo file, you must import it.

### Program

A program is a collection of code (its functions) and data (its types) that resides at a program ID on the Aleo blockchain. A program is declared as `program {name}.{network} { ... }`, with the body delimited by curly braces.

For the canonical list of which declarations belong inside vs. outside the `program { ... }` block, the program-ID naming rules, and import semantics, see [Project Layout](./01_layout.md#programs).

```leo file=../code_snippets/layout/main_example/src/main.leo#file
```

### Constant

A constant is declared as `const {name}: {type} = {expression};`.
Constants are immutable, and the right-hand side must be an expression evaluatable at compile time.

Constants can be declared in four scopes:

- **Global scope** (outside the `program` block in `main.leo`): accessible anywhere in the same file.
- **Program scope** (inside a `program` block, outside any function): accessible within that program.
- **Local scope** (inside a function body): accessible only within that function.
- **Module scope** (any non-`main.leo` source file in the package; module files do not contain a `program` block and may only declare `const`, `struct`, `fn`, and `interface`): accessible within the same package via `path::to::module::CONST_NAME`. See [Modules](./01_layout.md#modules) for details.

Constants are also supported in [libraries](./06_libraries.md), which are separate packages containing reusable code. A library's root file and its submodules may declare constants, accessible from any dependent package as `library::CONST_NAME` or `library::path::to::submodule::CONST_NAME`.

**Accessibility across packages:** Global and program-scope constants in a program are accessible from other programs that import it, using `program_name.aleo::CONST_NAME`. Constants declared in a submodule of an imported program are reachable through their full module path — `program_name.aleo::path::to::submodule::CONST_NAME` — provided the dependency is compiled from Leo source (pre-compiled `.aleo` stubs do not carry the submodule type information needed for resolution).

```leo file=../code_snippets/structure/constants/src/main.leo#scopes
```

**Supported types:** All integer types (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`), `bool`, `field`, `group`, `scalar`, `address`, and tuples, arrays, and structs composed of these types.

**Compile-time expressions:** The right-hand side of a constant declaration must be evaluatable at compile time. Valid right-hand sides include:

- Literal values (e.g., `42u32`, `true`, `1field`)
- References to previously declared constants
- Arithmetic, bitwise, and comparison expressions over constants (e.g., `MAX * 2u64`, `!FLAG`)
- Tuple, array, and struct expressions whose components are themselves compile-time constants

```leo file=../code_snippets/structure/constants/src/main.leo#expressions
```

### Import

An import is declared as `import {filename}.aleo;`. The dependency resolver pulls the imported program from the network or the local `imports/` directory. See [Imports](./01_layout.md#imports) for the declaration syntax and the [Dependencies guide](../guides/02_dependencies.md) for resolution rules.

```leo file=../code_snippets/layout/import_only/src/main.leo#snippet showLineNumbers
```

### Mappings

A mapping is declared as `mapping {name}: {key-type} => {value-type}`.
Mappings contain key-value pairs and are stored on chain.

```leo file=../code_snippets/structure/declarations/src/main.leo#mapping
```

### Storage

A storage variable is declared as `storage {name}: {type}`. Storage variables contain singleton values. They are declared at program scope and are stored on chain, similar to mappings.

```leo file=../code_snippets/structure/declarations/src/main.leo#storage_var
```

A storage vector is declared as `storage {name}: [{type}]`. Storage vectors contain dynamic lists of values of a given type. They are declared at program scope and are stored on chain, similar to mappings.

```leo file=../code_snippets/structure/declarations/src/main.leo#storage_vec
```

### Struct

A struct data type is declared as `struct {name} {}`.
Structs contain component declarations `{name}: {type},`.

```leo file=../code_snippets/structure/declarations/src/main.leo#struct showLineNumbers
```

### Record

A [record](https://docs.aleo.org/learn/core-concepts/public-and-private-state#private-state) data type is declared as `record {name} {}`. A record name must not contain the keyword `aleo`, and must not be a prefix of any other record name **declared in the same program** (the check does not extend across imported programs). This is a snarkVM requirement.

Records contain component declarations `{visibility} {name}: {type},`. Names of record components must not contain the keyword `aleo`.

The visibility qualifier may be specified as `constant`, `public`, or `private`. If no qualifier is provided, Leo defaults to `private`.

Record data structures must always contain a component named `owner` of type `address`, as shown below. When passing a record as input to a program function, the `_nonce: group` and `_version: u8` components are also required but do not need to be declared in the Leo program. They are inserted automatically by the compiler.

```leo file=../code_snippets/data_types/demo/src/main.leo#token_record showLineNumbers
```
