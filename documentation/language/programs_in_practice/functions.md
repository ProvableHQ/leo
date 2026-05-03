---
id: functions
title: Functions
sidebar_label: Functions
---

[general tags]: # "fn, final, entry_function, helper_function, final_fn"

## Entry Functions

Entry functions in Leo are declared as `fn {name}() {}` inside a `program {}` block. They define the program's public interface and can be called directly when running a Leo program (via `leo run`). If they include a `final { }` block to execute code on-chain, they must return `Final`.

```leo file=../../code_snippets/functions/entry_basic/src/main.leo showLineNumbers
```

### Inputs

Inputs are declared as `{visibility} {name}: {type}`. They must be declared just after the function name declaration, in parentheses.

```leo file=../../code_snippets/functions/entry_input/src/main.leo#snippet showLineNumbers
```

### Outputs

The return type of the function is declared as `-> {expression}` and must be declared just after the function inputs.
A function output is calculated as `return {expression};`. Returning an output ends the execution of the function, and the type of the returned value must match the output type declared in the function signature.

```leo file=../../code_snippets/functions/entry_output/src/main.leo#snippet showLineNumbers
```

## On-chain State with `final { }`

A `final { }` block is used to define computation that gets executed on-chain. The most common use case is to initiate or change public on-chain state within mappings or storage.

An entry `fn` that includes on-chain logic returns `Final` and embeds the on-chain code in a `final { }` block. Final blocks are atomic; they either succeed or fail, and state is reverted on failure.

```leo file=../../code_snippets/functions/transfer_inline/src/main.leo#file showLineNumbers
```

If there is no need to create or alter the public on-chain state, a `final { }` block is not required.

## On-chain State with `final fn`

When finalization logic is shared across multiple entry functions, it can be extracted into a `final fn`, declared outside the `program {}` block. A `final fn` call must still be wrapped in a `final { }` block at the call site:

```leo file=../../code_snippets/functions/transfer_final_fn/src/main.leo#file showLineNumbers
```

The body of `decrement_balance` is inlined into each caller's `final { }` block at compile time — no shared function exists in the compiled output.

## Helper Function

A helper function is declared as `fn {name}({arguments}) {}` **outside** the `program {}` block.
They contain expressions and statements that can compute values, but cannot produce `records`.

Helper functions cannot be called directly from outside the program. Instead, they are called by entry functions.
Inputs of helper functions cannot have `{visibility}` modifiers, since they are used only internally, not as part of a program's external interface.

```leo file=../../code_snippets/functions/helper_basic/src/main.leo#snippet showLineNumbers
```

Helper functions also support **const generics**:

```leo file=../../code_snippets/functions/const_generic/src/main.leo showLineNumbers
```

Acceptable types for const generic parameters include integer types, `bool`, `scalar`, `group`, `field`, and `address`.

:::note
Const generic parameters are only valid on inlinable helper `fn` functions. They are not permitted on entry point functions inside a `program {}` block, `final fn` functions, functions annotated with `@no_inline`, or function signatures declared inside an `interface`.
:::

### The `@no_inline` Annotation

By default the compiler inlines helper functions that are called only once, which reduces call overhead. To prevent this, annotate the function with `@no_inline`:

```leo file=../../code_snippets/functions/no_inline/src/main.leo#snippet
```

Use `@no_inline` when the function is intentionally shared across multiple call sites but the compiler would otherwise duplicate it, or when you want to preserve the function boundary for readability in the compiled output.

## Function Call Rules

- An entry `fn` can call: helper `fn`, `final fn`, and external entry `fn`s.
- A helper `fn` can only call: other helper `fn`s.
- A `final fn` can only call: other `final fn`s.
- Recursive calls (direct or indirect) are not allowed.
