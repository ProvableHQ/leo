---
id: structure
title: Structure of a Leo Program
sidebar_label: Program Structure
---

[general tags]: # "program, constant, import, record, struct, mapping"

## Layout of a Leo Program

A Leo program contains declarations of a [Program](#program), a [Constructor](#constructor), [Constants](#constant), [Imports](#import)
, [Structs](#struct), [Records](#record), [Mappings](#mappings), [Interfaces](./programs_in_practice/interfaces.md), and functions.
Declarations are locally accessible within a program file.
If you need a declaration from another Leo file, you must import it.

### Program

A program is a collection of code (its functions) and data (its types) that resides at a program ID on the Aleo blockchain. A program is declared as `program {name}.{network} { ... }`, with the body delimited by curly braces.

For the canonical list of which declarations belong inside vs. Outside the `program { ... }` block, the program-ID naming rules, and import semantics, see [Project Layout](./layout.md#programs).

```leo file=../code_snippets/layout/main_example/src/main.leo#file
```

### Constructor

A `constructor` is a special, mandatory function in the `program { ... }` block. Declare it as `constructor() { ... }`. Each program must declare exactly one constructor.

It has no parameters or return value, and it is not a regular `fn`. Do not call it directly. The network runs it on-chain during the initial deployment and each upgrade. It controls the program upgrade policy.

Two properties set a `constructor` apart from an ordinary function:

- **Immutable.** The logic set at first deployment can never be changed, modified, or deleted by a future upgrade.
- **Policy-bearing.** It carries exactly one upgrade annotation — `@noupgrade`, `@admin`, `@checksum`, or `@custom` — that selects how the program may be upgraded. The managed modes (`@noupgrade`, `@admin`, `@checksum`) require an **empty** body, since the compiler generates their logic. `@custom` requires a **non-empty** body that you write yourself. A constructor with no annotation, or with more than one, is a compile error.

```leo file=../code_snippets/upgradability/noupgrade/src/main.leo#file
```

Inside a `constructor`, you can read on-chain program metadata through the [`std::ctx`](./standard_library.md#stdctx) module — namely `std::ctx::addr()`, `std::ctx::edition()`, `std::ctx::program_owner()`, and `std::ctx::checksum()`. A `@custom` constructor typically branches on `std::ctx::edition()` to apply different rules at first deployment (`edition == 0`) versus later upgrades:

```leo file=../code_snippets/upgradability/timelock/src/main.leo#file
```

For the annotation argument grammar, the meaning of each `std::ctx::*()` accessor, and worked patterns for every upgrade mode, see the [Upgrading Programs guide](../guides/program_upgradability.md).

### Constant

A constant is declared as `const {name}: {type} = {expression};`.
Constants are immutable, and the right-hand side must be an expression evaluatable at compile time.

Constants can be declared in three scopes:

- **Global scope** (outside the `program` block in `main.leo`): accessible anywhere in the same file.
- **Local scope** (inside a function body): accessible only within that function.
- **Module scope**: applies to each non-`main.leo` source file in the package. Module files do not contain a `program` block. They can only declare `const`, `struct`, `fn`, and `interface`. Use `path::to::module::CONST_NAME` to access the constant in the same package. See [Modules](./layout.md#modules).

Constants are also supported in [libraries](./libraries.md), which are separate packages containing reusable code. A library's root file and its submodules may declare constants, accessible from any dependent package as `library::CONST_NAME` or `library::path::to::submodule::CONST_NAME`.

**Accessibility across packages:** An importing program can access global constants with `program_name.aleo::CONST_NAME`. Use `program_name.aleo::path::to::submodule::CONST_NAME` to access a constant in an imported program submodule. This access requires a dependency compiled from Leo source. Precompiled `.aleo` stubs do not contain the submodule type information that resolution requires.

```leo file=../code_snippets/structure/constants/src/main.leo#scopes
```

**Supported types:** Constants support all integer types, `bool`, `field`, `group`, `scalar`, and `address`. They also support tuples, arrays, and structs composed of these types.

**Compile-time expressions:** The right-hand side of a constant declaration must be evaluatable at compile time. Valid right-hand sides include:

- Literal values (for example, `42u32`, `true`, `1field`)
- References to previously declared constants
- Arithmetic, bitwise, and comparison expressions over constants (for example, `MAX * 2u64`, `!FLAG`)
- Tuple, array, and struct expressions whose components are themselves compile-time constants

```leo file=../code_snippets/structure/constants/src/main.leo#expressions
```

### Import

An import is declared as `import {filename}.aleo;`. The dependency resolver pulls the imported program from the network or the local `imports/` directory. See [Imports](./layout.md#imports) for the declaration syntax and the [Dependencies guide](../guides/dependencies.md) for resolution rules.

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

A [record](https://docs.aleo.org/learn/core-concepts/public-and-private-state#private-state) data type is declared as `record {name} {}`. A record name must not contain `aleo`. It must not prefix another record name **declared in the same program**. This check does not apply across imported programs. It is a snarkVM requirement.

Records contain component declarations `{visibility} {name}: {type},`. Names of record components must not contain the keyword `aleo`.

The visibility qualifier may be specified as `constant`, `public`, or `private`. If no qualifier is provided, Leo defaults to `private`.

Each record must contain an `owner` component of type `address`, as shown below. A record function input also requires the `_nonce: group` and `_version: u8` components. Do not declare these components in the Leo program. The compiler inserts them automatically.

```leo file=../code_snippets/data_types/demo/src/main.leo#token_record showLineNumbers
```
