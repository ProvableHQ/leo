---
id: functions
title: Functions
sidebar_label: Functions
---

[general tags]: # "fn, final, view, entry_function, helper_function, final_fn, view_fn"

## Entry Functions

Entry functions in Leo are declared as `fn {name}() {}` inside a `program {}` block. They define the program's public interface and can be called directly when running a Leo program (via `leo run`). If they include a `final { }` block to execute code on-chain, they must return `Final`.

```leo file=../../code_snippets/functions/entry_basic/src/main.leo showLineNumbers
```

### Inputs

Inputs are declared as `{visibility} {name}: {type}`. They must be declared just after the function name declaration, in parentheses.

```leo file=../../code_snippets/functions/entry_input/src/main.leo#snippet showLineNumbers
```

A visibility modifier may not be applied to a record or `Final` parameter. Records are passed by their `.record` marker and `Final`s carry no visibility, so a `public` or `private` mode on them is meaningless and is rejected.

### Outputs

The return type of the function is declared as `-> {expression}` and must be declared just after the function inputs. A function output is calculated as `return {expression};`. A return operation ends the function. The returned value type must match the output type in the function signature.

```leo file=../../code_snippets/functions/entry_output/src/main.leo#snippet showLineNumbers
```

As with inputs, a record or `Final` output cannot carry a visibility modifier.

## On-chain State with `final { }`

A `final { }` block is used to define computation that gets executed on-chain. The most common use case is to initiate or change public on-chain state within mappings or storage.

An entry `fn` that includes on-chain logic returns `Final` and embeds the on-chain code in a `final { }` block. Final blocks are atomic. They either succeed or fail, and state is reverted on failure.

```leo file=../../code_snippets/functions/transfer_inline/src/main.leo#file showLineNumbers
```

If there is no need to create or alter the public on-chain state, a `final { }` block is not required.

## On-chain State with `final fn`

When finalization logic is shared across multiple entry functions, it can be extracted into a `final fn`, declared outside the `program {}` block. A `final fn` call must still be wrapped in a `final { }` block at the call site:

```leo file=../../code_snippets/functions/transfer_final_fn/src/main.leo#file showLineNumbers
```

The body of `decrement_balance` is inlined into each caller's `final { }` block at compile time — no shared function exists in the compiled output.

A `final fn` may also declare an output type and `return` a value, like an ordinary function. The result is bound at the call site inside the `final { }` block, which is useful for sharing a computed on-chain value across entry functions:

```leo file=../../code_snippets/functions/final_fn_return/src/main.leo#file
```

## View Functions

A `view fn` is a read-only entry point. Declare it in a `program {}` block with the `view` modifier. A node can evaluate the resultant query without a transaction.

```leo file=../../code_snippets/functions/view_basic/src/main.leo#file showLineNumbers
```

A `view fn` body sees the same on-chain context as a `final {}` block — it can read mappings, storage, vectors, `std::ctx::block_height()`, and `std::ctx::network_id()`. Beyond the `final {}` rules above, a view adds these restrictions:

- **Read-only.** All state writes are rejected — both singleton storage assignment (`counter = 5u64;`, `counter = none;`) and the mutating intrinsics `Mapping::set`, `Mapping::remove`, `Vector::set`, `Vector::push`, `Vector::pop`, `Vector::swap_remove`, `Vector::clear`.
- **Leaf in the emitted bytecode.** A view can call a helper `fn`, and Leo puts the helper body in the view. A view cannot call another `view fn`, a `final fn`, or an entry point. Thus, the Aleo `view` block has no `call` instructions, as snarkVM requires. The compiler also rejects dynamic calls in the `dyn ...` form.
- **On-chain reads and proof verification.** A view can use `std::ctx::block_timestamp()`,
  `std::ctx::program_owner()`, `Snark::verify`, and `Snark::verify_batch`. These operations do not write state.
- Returns plaintext only (no records). Cannot be combined with `final`.

### Calling Views from On-chain Code

`view fn`s are only callable from a finalize context — a `final {}` block, a `final fn` helper, or a hoisted finalize body. A plain entry-function body cannot call a view directly.

```leo file=../../code_snippets/functions/view_in_finalize/src/main.leo#file showLineNumbers
```

Leo puts a helper `fn` in its call site, but a `view fn` remains a separate callable entity. Each call from the `final {}` block runs the view body again.

The same rule applies across programs — a `final {}` block can call a `view fn` exposed by an imported program:

```leo file=../../code_snippets/functions/view_cross_program_caller/src/main.leo#file showLineNumbers
```

## The Constructor

The `constructor` is the other function-like declaration in a `program {}` block. You do not call it directly. The network runs it on-chain during deployment and each upgrade. The constructor enforces the program upgrade policy. See [Constructor](../structure.md#constructor) and the [Upgrading Programs guide](../../guides/program_upgradability.md).

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

Acceptable types for const generic parameters include integer types, `bool`, `scalar`, `group`, `field`, `address`, and `identifier`.

:::note
Const generic parameters are only valid on functions that are inlined at every call site. They are not permitted on entry point functions inside a `program {}` block, functions annotated with `@no_inline`, or function signatures declared inside an `interface`. `final fn`s are always inlined into their `final {}` callsite, so they may declare const generic parameters.
:::

### The `@no_inline` Annotation

By default, the compiler puts a helper `fn` in each call site when this operation is safe and beneficial. Common conditions are one call, no arguments, or only arguments with empty types. This operation decreases call overhead and the compiled program size.

To opt out of this default and force a separate AVM function for a helper, annotate it with `@no_inline`:

```leo file=../../code_snippets/functions/no_inline/src/main.leo#snippet
```

Use `@no_inline` when a function is intentionally shared across multiple call sites. You can also use it to keep the function boundary clear in the compiled output.

#### When `@no_inline` is ignored

Some helpers cannot exist as standalone AVM functions and **must** be inlined regardless of the annotation. In these cases the compiler ignores `@no_inline` and emits a warning at the annotation site:

- helper functions defined in a submodule (`path::nested::fn`) — Aleo identifiers are flat, so there is no bytecode form for a nested name,
- helper functions defined in a [library](../libraries.md) — libraries have no on-chain footprint,
- a `final fn`,
- a helper reached from an on-chain context (a `constructor` or finalize block),
- a helper with more than 16 arguments,
- a helper whose argument or return type names an `Optional` type,
- helpers transitively reachable from another helper that itself must be inlined.

The annotation has no effect on entry `fn` declarations either — the entry-point boundary is part of the program's public interface and is never inlined away.

### The `@inline` Annotation

The compiler accepts `@inline` as an annotation name, but **no compiler pass acts on it**. It is a silent no-op from earlier Leo versions, where `inline` was a function modifier. See [Migrating from Leo 3.5 to 4.0](../../guides/migration_3_5_to_4_0.md#inline-becomes-fn).

The default behavior is the same with or without `@inline`. Do not put `@inline` in new code.

## Function Call Rules

- An entry `fn` can call: helper `fn`s, `final fn`s, and external entry `fn`s. Local entry `fn`s and `view fn`s (outside a `final {}` block) are rejected.
- A helper `fn` can only call: other helper `fn`s.
- A `final fn` can call: helper `fn`s, other `final fn`s, and `view fn`s.
- A `final {}` block can call: helper `fn`s, `final fn`s, and `view fn`s (same-program or cross-program).
- A `view fn` can only call helper `fn`s (which get inlined). Other `view fn`s, `final fn`s, and entry points are rejected.
- Recursive calls (direct or indirect) are not allowed.

A cross-program `final fn` call is allowed only when the function and its transitive calls do not write on-chain state.
Call an entry function in the other program when the operation must write its state.
