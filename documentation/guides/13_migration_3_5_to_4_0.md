---
id: migration-3-5-to-4-0
title: Migrating from Leo 3.5 to 4.0
sidebar_label: Migration Guide (3.5 → 4.0)
---

[general tags]: # "guides, migration, upgrade, leo4, breaking_changes"

Leo 4.0 redesigns the language surface to make Leo's **execution model** transparent. Every Leo program runs in two distinct contexts: a **proof context** (private, off-chain, generating ZK proofs) and a **finalization context** (public, on-chain, modifying state). The old keywords - `transition`, `function`, `async`, `Future` - obscured this distinction; 4.0 replaces them with a minimal set (`fn`, `final`, `Final`) that makes each function's execution context immediately clear. The `program {}` block now explicitly defines a program's public interface.

This guide covers every breaking change and shows how to update your code.

## Quick Reference

| 3.5 Syntax                         | 4.0 Syntax                                                   |
| ---------------------------------- | ------------------------------------------------------------ |
| `transition foo()`                 | `fn foo()` (inside `program {}`)                             |
| `async transition foo() -> Future` | `fn foo() -> Final`                                          |
| `function foo()`                   | `fn foo()` (outside `program {}`)                            |
| `async function foo()`             | `final { ... }` block (see [below](#asyncfinalize-to-final)) |
| `inline foo()`                     | `fn foo()` (outside `program {}`)                            |
| `Future` (type)                    | `Final`                                                      |
| `async { ... }`                    | `final { ... }`                                              |
| `f.await()`                        | `f.run()`                                                    |
| `@test script foo()`               | `@test fn foo()` (inside `program {}`)                       |
| `async constructor()`              | `constructor()`                                              |
| `foo.aleo/bar`                     | `foo.aleo::bar`                                              |

## Function Declaration Keywords

The entire function keyword vocabulary has been unified under `fn`. Where a function lives and what modifiers it carries determine its role.

### Transitions become `fn`

In 3.5, entry points were declared with `transition`:

```leo
// 3.5
program test.aleo {
    transition mint(public amount: u64) -> Token {
        return Token { owner: self.caller, amount: amount };
    }
}
```

In 4.0, use `fn` inside a `program {}` block:

```leo file=../code_snippets/migration/transitions_to_fn/src/main.leo#file
```

### `function` becomes `fn` (outside `program {}`)

In 3.5, helper functions lived inside the `program {}` block with the `function` keyword:

```leo
// 3.5
program test.aleo {
    function helper(a: u32, b: u32) -> u32 {
        return a + b;
    }

    transition mint(public amount: u64) -> u64 {
        return helper(amount, 1u32);
    }
}
```

In 4.0, helper functions move **outside** the `program {}` block, since that block now defines the program's public interface (entry points, records, mappings). Helper functions are internal and not part of the interface:

```leo file=../code_snippets/migration/function_to_fn/src/main.leo#file
```

### `inline` becomes `fn`

In 3.5, helper functions used the `inline` keyword (both inside and outside `program {}` blocks). In 4.0, `inline` is removed - all helpers become `fn` and those inside program blocks move outside:

```leo
// 3.5
program test.aleo {
    inline helper() -> u32 {
        return 42u32;
    }

    transition mint() -> u32 {
        return helper();
    }
}
```

```leo file=../code_snippets/migration/inline_to_fn/src/main.leo#file
```

## Program Block as Interface Boundary

In 3.5, all declarations - transitions, functions, structs, mappings - lived inside the `program {}` block. In 4.0, the `program {}` block defines the program's **public interface**: the entry points, records, mappings, and storage that are visible on-chain. Everything else moves outside:

| Inside `program {}`           | Outside `program {}`    |
| ----------------------------- | ----------------------- |
| Entry point `fn` declarations | Helper `fn` definitions |
| `record` definitions          | `final fn` definitions  |
| `mapping` declarations        | `struct` definitions    |
|                               | `interface` definitions |

This separation makes it easy to see what a program exposes on-chain at a glance. Helper functions and types that support the implementation but aren't part of the on-chain interface live at module level.

## Async/Finalize to Final

The async/finalize pattern - the mechanism for updating public on-chain state - has been reworked around the `final` keyword.

### Core concept

Leo programs execute in two distinct contexts:

- **Proof context** - private, off-chain execution that generates ZK proofs. Regular `fn` declarations run here. Inputs can be private, and the computation is not visible on-chain.
- **Finalization context** - public, on-chain execution that modifies state (mappings, storage). `final fn` definitions and `final { }` blocks run here. All inputs and operations are publicly visible.

In 3.5, the "async" terminology (`async transition`, `async function`, `Future`) suggested asynchronous execution, but what it really meant was "runs on-chain during finalization." The 4.0 keyword `final` directly communicates this: a `final` block or `final fn` runs in the finalization context.

In practice: 3.5 split on-chain logic across an `async transition` and a separate `async function`. In 4.0, on-chain logic lives inside `final { }` blocks within entry points.

When the compiler processes a `final { }` block, it lifts it into a standalone finalization function - the on-chain equivalent of 3.5's `async function`. `final fn` definitions, by contrast, are always inlined into the caller's finalization block before this lifting occurs, making them a compile-time code reuse mechanism rather than standalone on-chain functions.

### Inline finalize

**3.5** - separate `async transition` and `async function`:

```leo
// 3.5
program token.aleo {
    mapping balances: address => u64;

    async transition mint(public receiver: address, public amount: u64) -> Future {
        return finalize_mint(receiver, amount);
    }

    async function finalize_mint(public receiver: address, public amount: u64) {
        let current: u64 = balances.get_or_use(receiver, 0u64);
        balances.set(receiver, current + amount);
    }
}
```

**4.0** - `final { }` block inline:

```leo file=../code_snippets/migration/inline_finalize/src/main.leo#file
```

### Async blocks

If you were using the `async { }` shorthand in 3.5:

```leo
// 3.5
program token.aleo {
    mapping balances: address => u64;

    async transition mint(public receiver: address, public amount: u64) -> Future {
        let f: Future = async {
            let current: u64 = balances.get_or_use(receiver, 0u64);
            balances.set(receiver, current + amount);
        };
        return f;
    }
}
```

Replace `async` with `final` and `Future` with `Final`:

```leo file=../code_snippets/migration/async_block/src/main.leo#file
```

### Reusable finalization logic with `final fn`

4.0 introduces `final fn` as a new mechanism for **deduplicating** finalization logic across multiple entry points. Unlike 3.5's `async function` - which compiled to a standalone on-chain finalization - `final fn` bodies are **always inlined** into the caller's `final { }` block at compile time. They are a code reuse tool, not a direct replacement for `async function`.

The direct replacement for `async function` is the `final { }` block shown in the sections above. Use `final fn` when multiple entry points share common finalization logic:

```leo file=../code_snippets/migration/final_fn/src/main.leo#file
```

Here, `update_balance` is inlined into each caller's finalization block before the compiler lifts those blocks into standalone on-chain functions. The result is two independent on-chain finalizations that each contain the inlined logic - no shared `update_balance` function exists in the compiled output.

`final fn` definitions live **outside** the `program {}` block, since they are not part of the program's public on-chain interface.

### `.await()` becomes `.run()`

When composing futures from external program calls:

```leo
// 3.5
program example.aleo {
    async transition compose(value: u8) -> Future {
        let f: Future = other_program.aleo/action();
        return finalize_compose(value, f);
    }

    async function finalize_compose(value: u8, f: Future) {
        f.await();
        // ... on-chain logic
    }
}
```

```leo file=../code_snippets/migration/await_to_run/src/main.leo#file
```

### Summary of keyword changes

| 3.5                                | 4.0                                    |
| ---------------------------------- | -------------------------------------- |
| `async transition foo() -> Future` | `fn foo() -> Final`                    |
| `async function foo()`             | `final { ... }` block                  |
| `let f: Future = async { ... }`    | `let f: Final = final { ... }`         |
| `f.await()`                        | `f.run()`                              |
| `return finalize_foo(args)`        | `return final { finalize_foo(args); }` |

## Module-Level Struct Declarations

In both 3.5 and 4.0, structs can be declared inside or outside `program {}` blocks. The 4.0 convention is to place structs that aren't part of the on-chain interface (i.e. not records) at module level outside `program {}`. Records remain inside the program block since they are part of the public interface. Structs inside `program {}` still compile.

**3.5:**

```leo
program test.aleo {
    struct Point {
        x: i32,
        y: i32,
    }

    transition foo(p: Point) -> Point {
        return Point { x: p.y, y: p.x };
    }
}
```

**4.0 (recommended):**

```leo file=../code_snippets/migration/module_struct/src/main.leo#file
```

## Constructor

The `async` keyword is removed from constructor declarations. In 3.5, constructors were declared with `async constructor`; in 4.0 the keyword is simply `constructor`:

```leo
// 3.5
program hello.aleo {
    @noupgrade
    async constructor() {}
}
```

```leo file=../code_snippets/migration/constructor/src/main.leo#file
```

## External Call Syntax: `/` becomes `::`

In 3.5, calling a function or referencing a type in another program used a `/` separator:

```leo
// 3.5
let result: u32 = other_program.aleo/some_fn(1u32);
let s: other_program.aleo/MyStruct = other_program.aleo/MyStruct { x: 1u32 };
```

In 4.0, this separator is `::`, consistent with the path syntax used elsewhere in Leo:

```leo file=../code_snippets/migration/cross_program_caller/src/main.leo#snippet
```

This applies to all cross-program references: function calls, type annotations, external mapping access, external storage access, and external storage vector access.

**To migrate:** replace `program_name.aleo/` with `program_name.aleo::` wherever it appears in your Leo source files.

## Removed Features

### `script` functions, interpreter, and debugger

The `script` keyword, the interpreter (`leo test` for script functions), and the interactive debugger (`leo debug`) have all been removed. The interpreter worked by traversing the AST directly - a custom evaluation model that didn't reflect how code actually executes on-chain via the VM. Tests could pass in the interpreter but behave differently when compiled and run on the real VM.

In 3.5, tests used `@test script` inside a program block:

```leo
// 3.5
import some_program.aleo;

program test_some_program.aleo {
    @test
    script test_it() {
        let result: u32 = some_program.aleo/main(1u32, 2u32);
        assert_eq(result, 3u32);
    }
}
```

In 4.0, `script` is removed. Use `@test fn` inside a program block in a test file (under `tests/`):

```leo file=../code_snippets/migration/test_target/tests/test_test_program.leo#file
```

For end-to-end and integration testing, use the [SDK](https://github.com/ProvableHQ/sdk) directly or `snarkVM` as a library.

## New Features

The following are not breaking changes, but are worth knowing about when migrating.

### Interfaces

4.0 introduces `interface` definitions that specify contracts a program must fulfill:

```leo file=../code_snippets/migration/interface_basic/src/main.leo#file
```

Interfaces can declare function signatures, record definitions, mappings, and storage variables. Programs implement an interface by listing it after `:` in the program declaration.

For full documentation including record requirements, interface composition, dynamic calls, and dynamic records, see [Interfaces & Dynamic Dispatch](../language/programs_in_practice/interfaces.md).

Interfaces support inheritance:

```leo file=../code_snippets/migration/interface_inheritance/src/main.leo#file
```

### Inclusive ranges

4.0 adds `..=` for inclusive range bounds in `for` loops:

```leo file=../code_snippets/migration/inclusive_ranges/src/main.leo#snippet
```
