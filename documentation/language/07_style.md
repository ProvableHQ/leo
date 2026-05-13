---
id: style
title: Best Practices
sidebar: Best Practices
---

[general tags]: #

This guide is provided to point developers in the right direction when writing Leo code.
There are many conventions that are unique to the Leo language and the circuits it generates.

This guide is a living document.
As new Leo programming conventions arise and old ones become obsolete this guide should reflect the changes.
Feel free to add your comments and recommendations in the [Contributing](#contributing) section.

## Content

### Conditional Branches

The Leo compiler rewrites if-else statements in off-chain code into a sequence of ternary expressions.
This is because the underlying circuit construction does not support branching.
For precise control over the circuit size, it is recommended to use ternary expressions directly.

```leo file=../code_snippets/style/branches/src/main.leo#if_else title="If-Else:"
```

```leo file=../code_snippets/style/branches/src/main.leo#ternary title="Ternary:"
```

#### Why

Ternary expressions are the cheapest form of conditional.
We can resolve the _first expression_ and _second expression_ values before evaluating the _condition_.
This is very easy to convert into a circuit because we know that each expression does not depend on information in later statements.

In the original `Example`,
We cannot resolve the return statements before evaluating the condition.
As a solution, Leo creates branches in the circuit so both paths can be evaluated.

```leo file=../code_snippets/style/branches/src/main.leo#branch_a title="branch 1, condition = true"
```

```leo file=../code_snippets/style/branches/src/main.leo#branch_b title="branch 2, condition = false"
```

When the input value `condition` is fetched at proving time, we select a branch of the circuit to evaluate.
Observe that the statement `return a` is repeated in both branches.
The cost of every computation within the conditional will be doubled.
This greatly increases the constraint numbers and slows down the circuit.

### `final fn` vs. Inline `final` Blocks

For code conciseness and readability, prefer using inline `final { }` blocks rather than a separately declared `final fn`, unless the finalization logic is shared across multiple entry points:

```leo file=../code_snippets/style/final_fn/src/main.leo title="final fn (use only when shared across multiple entry points):"
```

```leo file=../code_snippets/style/inline_final/src/main.leo title="Inline final block (preferred for single-use logic):"
```

### Libraries

[Libraries](./06_libraries.md) are the right tool for sharing reusable code across programs. The following recommendations apply when authoring or consuming them.

#### Extract shared logic into a library

When the same helper functions, constants, or `struct` definitions appear in more than one program, move them into a library. This avoids duplicating constraints and makes maintenance easier.

```text
packages/
├── math_utils/   ← shared library
│   └── src/lib.leo
├── token_a.aleo/
│   └── src/main.leo
└── token_b.aleo/
    └── src/main.leo
```

#### Libraries are side-effect-free

Libraries should be **stateless**: no `program { }` block, no `mapping`, no `record`, no entry functions. All state belongs in a program. If you find yourself wanting on-chain state in a library, split the logic into a helper library and a thin program wrapper.

#### Prefer library functions over duplicating logic

Repeating a multi-step computation inline in several programs multiplies the constraint count across each circuit. Centralising that logic in a library function makes the constraint cost obvious and keeps each program smaller.

#### Use submodules for large libraries

When a library grows beyond a few hundred lines, split it across submodules named after their responsibility (`geometry.leo`, `encoding.leo`, etc.) and keep `lib.leo` as the public surface re-exporting common items.

```text
math_utils/
├── src/
│   ├── lib.leo        ← public API
│   ├── geometry.leo   ← math_utils::geometry::*
│   └── encoding.leo   ← math_utils::encoding::*
```

#### Name libraries clearly

Library package names follow the same snake_case rule as programs. Prefer a single descriptive noun when possible (`math`, `encoding`, `token_utils`).

### Modules

For maximal code cleanliness and readability, take full advantage of Leo's module system:

```text
src
├── constants.leo
├── utils.leo
├── structs.leo
└── main.leo
```

With the above structure, consider the following:

- Move all `const`s to the `constants.leo` module
- Move all helper `fn` functions to the `utils.leo` module
- Move some `struct`s to modules (but this may not make sense in the general case)

The goal is to only have the interface of the program in `main.leo`. Every function should correspond to something that can be called from an external context such as another program. Note that there is no impact on final program size since modules are flattened into a single program eventually anyways.

## Layout

### Indentation

4 spaces per indentation level.

### Blank lines

A single blank line should separate the top-level declarations in a `program` scope,
namely `fn`, `record`, and `mapping` declarations, as well as module-level `struct` and helper `fn` declarations.
Multiple imports can be optionally separated by a single blank line;
the last import at the top of the file should be followed by a blank line.

```leo title="Yes:"
import std.io.Write;
import std.math.Add;

struct A {
    // ...
}

fn foo() {
    // ...
}

program prog.aleo {
    // ...
}
```

```leo title="No:"
import std.io.Write;


import std.math.Add;
program prog.aleo {
    struct A {
        // ...
    }
    fn foo() {
        // ...
    }
}
```

### Naming Conventions

| Item                      | Convention                          |
| ------------------------- | ----------------------------------- |
| Packages                  | snake_case (but prefer single word) |
| Structs and Records       | CamelCase                           |
| Struct and Record Members | snake_case                          |
| Functions                 | snake_case                          |
| Function Parameters       | snake_case                          |
| Variables                 | snake_case                          |
| Inputs                    | snake_case                          |

### Layout

Leo file elements should be ordered:

1. Imports
2. Constants + Structs (module level)
3. Helper `fn` and `final fn` definitions
4. Program declaration
5. Mappings + Records
6. Entry point `fn` declarations

### Braces

Opening braces always go on the same line.

```leo file=../code_snippets/style/formatting/src/main.leo#braces
```

### Semicolons

Every statement including the `return` statement should end in a semicolon.

```leo file=../code_snippets/style/formatting/src/main.leo#semicolons
```

### Commas

Trailing commas should be included whenever the closing delimiter appears on a separate line.

```leo file=../code_snippets/style/formatting/src/main.leo#commas
```

## Contributing

Thank you for helping make Leo better!

Before contributing, please view the [Contributor Code of Conduct](https://github.com/ProvableHQ/leo/blob/mainnet/CONTRIBUTING.md).
By participating in this project - In the issues, pull requests, or Gitter channels -
you agree to abide by the terms.

### Report an Issue

To report an issue, please use the [GitHub issues tracker](https://github.com/ProvableHQ/leo/issues). When reporting issues, please mention the following details:

- Which version of Leo you are using.
- What was the source code (if applicable).
- Which platform are you running on.
- How to reproduce the issue.
- What was the result of the issue.
- What the expected behavior is.

Reducing the source code that caused the issue to a bare minimum is always very helpful and sometimes clarifies a misunderstanding.

### Make a Pull Request

Start by forking off of the `mainnet` branch to make your changes. Commit messages should clearly explain why and what you changed.

If you need to pull in any changes from the `mainnet` branch after making your fork (for example, to resolve potential merge conflicts),
please avoid using git merge and instead, git rebase your branch. Rebasing will help us review your changes easily.

#### Tools Required

To build Leo from source you will need the following tools:

- The latest Rust stable version and nightly version.
  - Recommend that you install multiple versions using `rustup`.
- Cargo
  - Rusty Hook install via `cargo install rusty-hook`.
- Clippy
  - Via rustup, if you didn't do the default rustup install `rustup component add clippy`.

#### Formatting

Please do the following before opening a PR.

- `cargo +nightly fmt --all` will format all your code.
- `cargo clippy --all-features --examples --all --benches`

#### Tests

If your code adds new functionality, please write tests to confirm the new features function as expected. Refer to existing tests for examples of how tests are expected to be written. Please refer to the [parser tests section](#parser-tests). To run the tests please use the following command `cargo test --all --features ci_skip --no-fail-fast`.

##### **Parser Tests**

In the root directory of the repository, there is a "tests" directory.
To add a parser test, look at the Example Leo files in the parser sub-directory.
Then when running the test command, make sure you have the environment variable `CLEAR_LEO_TEST_EXPECTATIONS` set to true. For example, on a UNIX environment, you could run the following command `CLEAR_LEO_TEST_EXPECTATIONS=true cargo test --all --features ci_skip --no-fail-fast`.

#### Grammar

[The `grammars` repository](https://github.com/ProvableHQ/grammars) contains a file [`leo.abnf`](https://github.com/ProvableHQ/grammars/blob/master/leo.abnf) that has the Leo grammar rules in the ABNF format.
If your changes affect a grammar rule, we may ask you to modify it in that `.abnf` file.

We appreciate your hard work!
