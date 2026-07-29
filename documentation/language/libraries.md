---
id: libraries
title: Leo Libraries
sidebar_label: Libraries
---

[general tags]: # "library, reusable_code, struct, const, fn, dependency, module"

A **library** is a Leo project that contains reusable code — structs, constants, and helper functions — intended to be shared across multiple programs. Unlike a regular Leo program, a library has no on-chain footprint: it declares no program ID, no mappings, no records, and no entry functions. All library code is inlined into the programs that use it at compile time.

:::note
Every Leo program implicitly depends on one built-in library: `std`. See the [Standard Library reference](./standard_library.md) for the full catalog of hash, commit, signature, randomness, serialization, group, and execution-context functions available without any declaration.
:::

## Creating a Library

Use `leo new` with the `--library` flag to create a library project:

```bash
leo new math_utils --library
```

This produces the same project structure as a regular Leo project, with one difference: the main source file is named `lib.leo` instead of `main.leo`.

```text
math_utils/
├── program.json
├── src/
│   └── lib.leo
└── tests/
    └── test_math_utils.leo
```

## Writing a Library

A library source file (`lib.leo`) may contain `struct` definitions, `const` declarations, and `fn` definitions. It does **not** contain a `program { }` block.

```leo file=../code_snippets/libraries/math_utils/src/lib.leo#basics title="src/lib.leo"
```

By default, a library item is private to its source file. Add `export` to an item that another source file or package must use. Other declarations in the same file can use the private item.

### What a library may contain

| Item                    | Allowed | Notes                                         |
| ----------------------- | ------- | --------------------------------------------- |
| `const` declarations    | ✅      | Global compile-time constants                 |
| `struct` definitions    | ✅      | Shared data types                             |
| `fn` definitions        | ✅      | Helper functions, including generic `fn::[…]` |
| `program { }` block     | ❌      | Libraries have no on-chain identity           |
| `mapping` / `storage`   | ❌      | No on-chain state                             |
| `record` types          | ❌      | Records belong to programs                    |
| Entry `fn` / `final fn` | ❌      | No callable entry points                      |

## Declaring the Dependency

To use a library from another Leo project, add it to that project's `program.json`. Libraries can only be referenced from the local filesystem.

### Local library

```json file=../code_snippets/libraries/my_app_closest/program.json title="program.json"
```

:::info
The `leo add` command can populate these entries automatically:

```bash
leo add math_utils --local ../math_utils 
```

:::

## Using a Library

Use the `{library_name}::{item}` path syntax to refer to exported library items. You do not need an `import` statement. The dependency entry in `program.json` is sufficient.

```leo file=../code_snippets/libraries/my_app_closest/src/main.leo#program title="src/main.leo"
```

Constants from a library are referenced the same way:

```leo file=../code_snippets/libraries/ceiling_demo/src/main.leo#snippet
```

## Generic Library Functions

Library functions support const generic parameters, just like regular helper functions. The concrete type argument must be a compile-time constant.

```leo file=../code_snippets/libraries/math_utils/src/lib.leo#clamp title="src/lib.leo"
```

```leo file=../code_snippets/libraries/my_app_normalize/src/main.leo title="src/main.leo"
```

Const-generic library functions operate like const-generic functions in a program. The compiler monomorphizes each `library::fn::[const_args](runtime_args)` call for its const arguments. Then, it puts the function code in the caller. Library code always uses this process, including code across package boundaries. A consuming program can reference and instantiate const-generic library structs with a fully qualified path. For example, use `math_utils::Vec::[10]`.

## Submodules

A library can have multiple source files. Put additional `.leo` files with `lib.leo` in `src/` to create submodules. Each file creates a submodule that has the file name. Export an item before you access it through the additional path segment.

```text
math_utils/
├── src/
│   ├── lib.leo       ← root: math_utils::item
│   └── geometry.leo  ← submodule: math_utils::geometry::item
```

```leo file=../code_snippets/libraries/math_utils/src/geometry.leo title="src/geometry.leo"
```

```leo file=../code_snippets/libraries/my_app_floor_area/src/main.leo title="src/main.leo"
```

## Name Resolution and Path Precedence

When a library dependency and local submodule have the same name, paths with that name resolve to the **library** first. For example, assume that a library dependency is named `foo` and a local submodule is `src/foo.leo`. In this case, `foo::bar` refers to `bar` in the library, not the submodule.

:::note
Explicit disambiguation using absolute paths (similar to Rust's `crate::foo::…` for local modules) is planned for a future release.
:::

## Building a Library

Running `leo build` inside a library package parses the library sources and runs semantic validation on the library itself. Type errors, unknown identifiers, interface-cycle errors, and the like are reported at the library package, instead of surfacing only when a downstream program consumes it.

```bash
cd math_utils
leo build
```

```text
       Leo 🔨 Building library 'math_utils'
       Leo ✅ Validated 'math_utils'.
```

The build does not produce bytecode because libraries have no on-chain footprint. However, Leo reports frontend errors with spans that point to the library source files.

:::note
Leo compiles library sources with each program that depends on the library. Thus, the consuming program build reports errors in the library. Run `leo build` in the library package to validate it separately. This operation finds problems before a consumer uses the library.
:::

## Testing

`leo test` works on library packages directly — no wrapper program is required. Place test files in the `tests/` directory and call library functions using the `library_name::item` path syntax.

```leo file=../code_snippets/libraries/math_utils/tests/test_math_utils.leo#tests title="tests/test_math_utils.leo"
```

Run from the library root:

```bash
leo test
```

See the [Testing guide](../guides/testing.md) for more details.

## How Libraries Work

Libraries are **inlined at compile time**. The Leo compiler resolves all library references before emitting Aleo bytecode — no library code appears as a separate program on-chain. This means:

- Calling a library function has the same cost as calling an inline helper function.
- Libraries cannot be deployed independently. They exist only as source-level abstractions.
- Circular dependencies between libraries are not allowed.

For more on how dependencies are resolved and cached, see [Dependency Management](../guides/dependencies.md).
