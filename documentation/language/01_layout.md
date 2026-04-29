---
id: layout
title: Layout of a Leo Project
sidebar_label: Project Layout
---

[general tags]: # "project, project_layout, manifest, module"

## Manifest

**program.json** is the Leo manifest file that configures our package.

```json file=../code_snippets/hello/program.json title="program.json"
```

The program ID in `program` is the official name that other developers will be able to look up after you have published your program.

```json
    "program": "hello.aleo",
```

Dependencies will be added to the field of the same name, as they are added. The dependencies are also pegged in the **leo.lock** file.

The `src/` directory is where all of your Leo code will live. The main entry point of your project is a file in this directory appropriately named `main.leo`. Calls to many of the Leo CLI commands will require you to have this file within your project in order to succeed properly.

## Programs

A program is a collection of code (its functions) and data (its types) that resides at a
[program ID](#program-id) on the Aleo blockchain. A program is declared as `program {name}.{network} { ... }`.
The body of the program is delimited by curly braces `{}`.

```leo file=../code_snippets/layout/main_example/src/main.leo#file title=main.leo
```

The following must be declared inside the scope of a program in a Leo file:

- [Mappings](./02_structure.md#mappings) and [Storage](./02_structure.md#storage)
- [Records](./02_structure.md#record)
- [Entry point `fn` declarations](./programs_in_practice/functions.md#entry-functions)

The following must be declared outside the scope of a program in a Leo file:

- [Imports](#imports)
- [Constants](./02_structure.md#constant)
- [Structs](./02_structure.md#struct)
- Helper `fn` definitions
- [`final fn` definitions](./programs_in_practice/functions.md#on-chain-state-with-final-fn)
- [`interface` definitions](./programs_in_practice/interfaces.md)

Declarations are locally accessible within a program file. If you need a declaration from another Leo file, you must import it.

### Imports

You can import dependencies that are downloaded to the `imports` directory.
An import is declared as `import {filename}.aleo;`
The dependency resolver will pull the imported program from the network or the local filesystem.

```leo file=../code_snippets/layout/import_only/src/main.leo#snippet showLineNumbers
```

### Program ID

A program ID is declared as `{name}.{network}`.

The first character of a `name` must be a lowercase letter.
`name` can only contain lowercase letters, numbers, and underscores, and must not contain a double underscore (`__`) or the keyword `aleo` in it.

Currently, `aleo` is the only supported `network` domain.

```leo showLineNumbers
program hello.aleo; // valid

program Foo.aleo;   // invalid
program baR.aleo;   // invalid
program 0foo.aleo;  // invalid
program 0_foo.aleo; // invalid
program _foo.aleo;  // invalid
program foo__bar.aleo;  // invalid
program aleo.aleo;  // invalid
```

## Modules

In addition to your main file, Leo also supports a module system as of v3.2.0.

Leaf modules (i.e. modules without submodules) must be defined in a single file (ex. `foo.leo`). Modules with submodules must be defined by an optional top-level `.leo` file and a subdirectory containing the submodules:

Take the following project as an example:

```text
src
├── common.leo
├── main.leo
├── outer.leo
└── outer
    └── inner.leo
```

Given the structure above, the following modules are defined:

| Filename          | Type      | Module Name    | Access Location & Pattern                                                   |
| ----------------- | --------- | -------------- | --------------------------------------------------------------------------- |
| `common.leo`      | Module    | `common`       | `main.leo` : `common::<item>`                                               |
| `outer.leo`       | Module    | `outer`        | `main.leo` : `outer::<item>`                                                |
| `outer/inner.leo` | Submodule | `outer::inner` | `main.leo` : `outer::inner::<item>` <br></br> `outer.leo` : `inner::<item>` |

:::info
Only relative paths are implemented so far. That means that items in `outer.leo` cannot be accessed from items in `inner.leo`, for example. This is limiting for now but will no longer be an issue when we add absolute paths.
:::

A module file may only contain `struct`, `const`, and `fn` definitions:

```leo file=../code_snippets/layout/module_demo/src/mymod.leo#snippet
```

### Accessing Submodules of Imported Programs

When an imported program organizes its source across submodules, you can reach any `struct`, `const`, or helper `fn` from those submodules using an extended locator path:

```text
program.aleo::submodule::item
```

For example, suppose `provider.aleo` has a submodule `colors` that defines a `Color` struct, a `MAX_CH` constant, and a `blend` helper:

```leo file=../code_snippets/layout/provider/src/colors.leo title="provider/src/colors.leo"
```

```leo file=../code_snippets/layout/provider/src/main.leo title="provider/src/main.leo"
```

A program that imports `provider.aleo` can reach the submodule struct, constant, and helper through the extended path, and can also call `provider.aleo`'s top-level entry functions:

```leo file=../code_snippets/layout/consumer/src/main.leo title="consumer/src/main.leo"
```

Helper `fn`s reached through `program.aleo::submodule::name(...)` are inlined directly into the caller's bytecode; they are not separate on-chain calls and do not appear in the provider's ABI. Only top-level entry functions declared inside `program provider.aleo { ... }` remain part of its on-chain interface.

Submodule paths can be arbitrarily deep — `program.aleo::a::b::item` is valid if `program.aleo` has a nested submodule `a/b.leo`. The same extended path syntax applies to library submodules (see [Leo Libraries](./06_libraries.md#submodules)).

`interface` definitions may also be referenced through the same path syntax — both library submodules (`program my_app.aleo: my_lib::interfaces::Adder { ... }`) and imported program submodules (`program my_app.aleo: other_prog.aleo::interfaces::Adder { ... }`) work in a program header.

<!--

## The Tests

TODO

## The Build and Outputs

Only generated when the project is compiled.  Removed when `leo clean` is called.

TODO

-->
