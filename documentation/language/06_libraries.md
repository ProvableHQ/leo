---
id: libraries
title: Leo Libraries
sidebar_label: Libraries
---

[general tags]: # "library, reusable_code, struct, const, fn, dependency, module"

A **library** is a Leo project that contains reusable code — structs, constants, and helper functions — intended to be shared across multiple programs. Unlike a regular Leo program, a library has no on-chain footprint: it declares no program ID, no mappings, no records, and no entry functions. All library code is inlined into the programs that use it at compile time.

## Creating a Library

Use `leo new` with the `--library` flag to create a library project:

```bash
leo new math_utils --library
```

This produces the same project structure as a regular Leo project, with one difference: the main source file is named `lib.leo` instead of `main.leo`.

```
math_utils/
├── program.json
├── src/
│   └── lib.leo
└── tests/
    └── test_math_utils.leo
```

## Writing a Library

A library source file (`lib.leo`) may contain `struct` definitions, `const` declarations, and `fn` definitions. It does **not** contain a `program { }` block.

```leo title="src/lib.leo"
/// The maximum value representable by a u32.
const MAX_U32: u32 = 4294967295u32;

/// A 2-D point with integer coordinates.
struct Point {
    x: i32,
    y: i32,
}

/// Returns the absolute value of a signed 32-bit integer.
fn abs(x: i32) -> i32 {
    return x >= 0i32 ? x : 0i32 - x;
}

/// Returns the Manhattan distance between two points.
fn manhattan(a: Point, b: Point) -> u32 {
    let dx: i32 = abs(a.x - b.x);
    let dy: i32 = abs(a.y - b.y);
    return dx as u32 + dy as u32;
}
```

### What a library may contain

| Item | Allowed | Notes |
| ---- | ------- | ----- |
| `const` declarations | ✅ | Global compile-time constants |
| `struct` definitions | ✅ | Shared data types |
| `fn` definitions | ✅ | Helper functions, including generic `fn::[…]` |
| `program { }` block | ❌ | Libraries have no on-chain identity |
| `mapping` / `storage` | ❌ | No on-chain state |
| `record` types | ❌ | Records belong to programs |
| Entry `fn` / `final fn` | ❌ | No callable entry points |

## Declaring the Dependency

To use a library from another Leo project, add it to that project's `program.json`. Libraries can only be referenced from the local filesystem.

### Local library

```json title="program.json"
{
  "program": "my_app.aleo",
  "version": "0.1.0",
  "description": "",
  "license": "MIT",
  "leo": "4.0.0",
  "dependencies": [
    {
      "name": "math_utils",
      "location": "local",
      "path": "../math_utils"
    }
  ]
}
```

:::info
The `leo add` command can populate these entries automatically:
```bash
leo add math_utils --local ../math_utils 
```
:::

## Using a Library

Reference library items with the `{library_name}::{item}` path syntax. No `import` statement is required; the dependency entry in `program.json` is sufficient.

```leo title="src/main.leo"
program my_app.aleo {
    fn closest(
        origin: math_utils::Point,
        a: math_utils::Point,
        b: math_utils::Point,
    ) -> math_utils::Point {
        let da: u32 = math_utils::manhattan(origin, a);
        let db: u32 = math_utils::manhattan(origin, b);
        return da <= db ? a : b;
    }

    @noupgrade
    constructor() {}
}
```

Constants from a library are referenced the same way:

```leo
const CEILING: u32 = math_utils::MAX_U32;
```

## Generic Library Functions

Library functions support const generic parameters, just like regular helper functions. The concrete type argument must be a compile-time constant.

```leo title="src/lib.leo"
/// Clamps `value` to the range [0, MAX].
fn clamp::[MAX: u32](value: u32) -> u32 {
    return value > MAX ? MAX : value;
}
```

```leo title="src/main.leo"
program my_app.aleo {
    fn normalize(x: u32) -> u32 {
        return math_utils::clamp::[100u32](x);
    }

    @noupgrade
    constructor() {}
}
```

## Submodules

A library can span multiple source files. Place additional `.leo` files alongside `lib.leo` in `src/` to create submodules. Each file becomes a submodule named after the file, and its items are accessed with an extra path segment.

```
math_utils/
├── src/
│   ├── lib.leo       ← root: math_utils::item
│   └── geometry.leo  ← submodule: math_utils::geometry::item
```

```leo title="src/geometry.leo"
fn area(width: u32, height: u32) -> u32 {
    return width * height;
}
```

```leo title="src/main.leo"
program my_app.aleo {
    fn floor_area(w: u32, h: u32) -> u32 {
        return math_utils::geometry::area(w, h);
    }

    @noupgrade
    constructor() {}
}
```

## Name Resolution and Path Precedence

When a library dependency and a local submodule share the same name, paths beginning with that name resolve to the **library** first. For example, if your project declares a library dependency called `foo` and also has a local submodule `src/foo.leo`, then `foo::bar` refers to the item `bar` from the library, not from the submodule.

:::note
Explicit disambiguation using absolute paths (similar to Rust's `crate::foo::…` for local modules) is planned for a future release.
:::

## Building a Library

Running `leo build` inside a library package parses the library sources and runs semantic validation on the library itself. Type errors, unknown identifiers, interface-cycle errors, and the like are reported at the library package, instead of surfacing only when a downstream program consumes it.

```bash
cd math_utils
leo build
```

```
       Leo 🔨 Building library 'math_utils'
       Leo ✅ Validated 'math_utils'.
```

No bytecode is produced — libraries are inlined at the point of use and have no on-chain footprint — but any frontend errors are reported with spans pointing into the library's own source files.

:::note
When a program that depends on a library is built, library sources are compiled holistically with the program — any errors in the library still surface, but as part of the consuming program's build. Running `leo build` inside the library package itself validates it in isolation, so problems are caught at the source before any consumer tries to use it.
:::

## Testing

`leo test` works on library packages directly — no wrapper program is required. Place test files in the `tests/` directory and call library functions using the `library_name::item` path syntax.

```leo title="tests/test_math_utils.leo"
program test_math_utils.aleo {
    @test
    fn test_abs() {
        assert_eq(math_utils::abs(0i32 - 5i32), 5i32);
    }

    @test
    fn test_geometry_area() {
        assert_eq(math_utils::geometry::area(3u32, 4u32), 12u32);
    }

    @noupgrade
    constructor() {}
}
```

Run from the library root:

```bash
leo test
```

See the [Testing guide](../guides/08_testing.md) for more details.

## How Libraries Work

Libraries are **inlined at compile time**. The Leo compiler resolves all library references before emitting Aleo bytecode — no library code appears as a separate program on-chain. This means:

- Calling a library function has the same cost as calling an inline helper function.
- Libraries cannot be deployed independently; they exist only as source-level abstractions.
- Circular dependencies between libraries are not allowed.

For more on how dependencies are resolved and cached, see [Dependency Management](../guides/02_dependencies.md).
