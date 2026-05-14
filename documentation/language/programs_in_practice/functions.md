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

## View Functions

A `view fn` is a read-only entry point. It is declared inside a `program {}` block with the `view` modifier and exposes a query that can be evaluated by a node without producing a transaction.

```leo file=../../code_snippets/functions/view_basic/src/main.leo#file showLineNumbers
```

A `view fn` body sees the same on-chain context as a `final {}` block — it can read mappings, storage, vectors, `block.height`, and `network.id`. Beyond the `final {}` rules above, a view adds these restrictions:

- **Read-only.** All state writes are rejected — both singleton storage assignment (`counter = 5u64;`, `counter = none;`) and the mutating intrinsics `Mapping::set`, `Mapping::remove`, `Vector::set`, `Vector::push`, `Vector::pop`, `Vector::swap_remove`, `Vector::clear`.
- **Leaf in the emitted bytecode.** A view may call a helper `fn` (its body is fully inlined into the view), but it cannot `call` another `view fn`, a `final fn`, or an entry point. This keeps the emitted Aleo `view` block free of `call` instructions, which snarkVM requires. Dynamic calls (the `dyn ...` form) are also rejected.
- No `block.timestamp`, `Snark::verify`, `Snark::verify_batch`, or `program_owner` — these are available in `final {}` but not when a node evaluates a view off-consensus.
- Returns plaintext only (no records); cannot be combined with `final`.

### Calling Views from On-chain Code

`view fn`s are only callable from a finalize context — a `final {}` block, a `final fn` helper, or a hoisted finalize body. A plain entry-function body cannot call a view directly.

```leo file=../../code_snippets/functions/view_in_finalize/src/main.leo#file showLineNumbers
```

The view is not inlined into its caller; codegen emits a real `call` instruction inside the on-chain body, and snarkVM accepts it because the call target resolves to a view.

The same rule applies across programs — a `final {}` block can call a `view fn` exposed by an imported program:

```leo file=../../code_snippets/functions/view_cross_program_caller/src/main.leo#file showLineNumbers
```

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

By default the compiler inlines a helper `fn` whenever inlining is safe and beneficial — most commonly when the function is called only once, takes no arguments, or all of its arguments have empty types. Inlining reduces call overhead and shrinks the compiled program.

To opt out of this default and force a separate AVM function for a helper, annotate it with `@no_inline`:

```leo file=../../code_snippets/functions/no_inline/src/main.leo#snippet
```

Use `@no_inline` when the function is intentionally shared across multiple call sites but the compiler would otherwise duplicate it, or when you want to preserve the function boundary for readability in the compiled output.

#### When `@no_inline` is ignored

Some helpers cannot exist as standalone AVM functions and **must** be inlined regardless of the annotation. In these cases the compiler ignores `@no_inline` and emits a warning at the annotation site:

- helper functions defined in a submodule (`path::nested::fn`) — Aleo identifiers are flat, so there is no bytecode form for a nested name,
- helper functions defined in a [library](../06_libraries.md) — libraries have no on-chain footprint,
- a `final fn`,
- a helper reached from an on-chain context (a `constructor` or finalize block),
- a helper with more than 16 arguments,
- a helper whose argument or return type names an `Optional` type,
- helpers transitively reachable from another helper that itself must be inlined.

The annotation has no effect on entry `fn` declarations either — the entry-point boundary is part of the program's public interface and is never inlined away.

### The `@inline` Annotation

The compiler accepts `@inline` as a recognized annotation name, but **no compiler pass acts on it** — it is a silent no-op carried over from earlier Leo versions, where `inline` was a function-modifier keyword rather than an annotation (see [Migrating from Leo 3.5 to 4.0](../../guides/13_migration_3_5_to_4_0.md#inline-becomes-fn)). The default inlining behaviour described above is the same whether or not `@inline` is present, so prefer to leave it out of new code.

## Function Call Rules

- An entry `fn` can call: helper `fn`s, `final fn`s, and external entry `fn`s. Local entry `fn`s and `view fn`s (outside a `final {}` block) are rejected.
- A helper `fn` can only call: other helper `fn`s.
- A `final fn` can call: helper `fn`s, other `final fn`s, and `view fn`s.
- A `final {}` block can call: helper `fn`s, `final fn`s, and `view fn`s (same-program or cross-program).
- A `view fn` can only call helper `fn`s (which get inlined). Other `view fn`s, `final fn`s, and entry points are rejected.
- Recursive calls (direct or indirect) are not allowed.
