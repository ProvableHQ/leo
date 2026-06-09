---
id: finalization
title: The Finalization Model
sidebar_label: Finalization Model
---

[general tags]: # "guides, final, entry_function, program"

## Background

Leo programs run in two distinct contexts:

- **Proof context** — private, off-chain execution that generates ZK proofs. Regular `fn` declarations run here. Inputs can be private, and the computation is not visible on-chain.
- **Finalization context** — public, on-chain execution that modifies state. `final { }` blocks run here. All inputs and operations are publicly visible.

## Managing Public State

On-chain data is stored publicly in one of three data structures: mappings, storage variables, and storage vectors. Any logic that reads from or updates the state of these structures must be contained within a `final { }` block or inside a `final fn`:

```leo file=../code_snippets/finalization/first_state/src/main.leo#file
```

Entry functions with on-chain logic return `Final` and embed the finalization code in an inline `final { }` block. A few additional rules apply:

- Entry functions can return additional data types in a tuple, including Records, along with a `Final`.
- Only one `Final` can be returned.
- If multiple types are returned, the `Final` must be the last type in the tuple.

## External Calls

Leo enables developers to call entry functions from imported programs. A call to an external entry function that returns `Final` produces a `Final` value which can be composed inside a `final { }` block using the `.run()` method:

```leo file=../code_snippets/finalization/external_call/src/main.leo#file
```

### The `Final` Type

`Final` is an opaque value: once produced, you can either run it (with `.run()`) or pass it onward to another finalize-aware function. There is no syntax to inspect, copy, reassign, or otherwise manipulate it. The compiler enforces this — every `Final` produced in a function must be consumed exactly once on every execution path.

If a finalize body needs values that were available in the caller's proof context, pass them as additional arguments to the finalize call.

## Checks-Effects-Interactions (CEI)

Inside a `final { }` block, the order of state reads, state writes, and external calls (`.run()`) matters. The compiler runs a **Checks-Effects-Interactions** analysis pass that warns when this ordering is violated:

1. **Checks** — reads of `mapping`, `storage`, or `Mapping::get` / `Mapping::contains`, and `assert`s.
2. **Effects** — writes via `set`, `remove`, or storage assignments.
3. **Interactions** — external finalize executions via `Final::run()`.

Within a single execution path, all checks and effects must precede any `.run()` call. Performing a check or effect *after* an interaction is flagged because the external program's finalize may have already modified the state your check reads or your effect writes against, opening a reentrancy-style class of bugs.

The pass emits warnings (code prefix `CEI`) in the following situations:

- A check (read or `assert`) runs after a `.run()` in the same path. Move it above the `.run()`.
- An effect (state write) runs after a `.run()` in the same path. Move it above the `.run()`.
- A call to a helper finalize function runs after a `.run()`, and the helper itself performs checks or effects. Hoist the call above the `.run()`.
- A loop body contains both an interaction and a check or effect. Split the loop into two passes — one that performs all checks/effects, then a second that performs the interactions.

The pass also performs a **cross-layer taint analysis** on values returned from external calls. A value derived from an external `Final` may be invalidated when that external finalize executes concurrently. The compiler warns when:

- A tainted value is used inside the finalize block — re-read the value on-chain instead of relying on the cached one.
- A tainted value is passed as an argument to another external finalize — pass the value through an on-chain read inside the receiving finalize.

These warnings are advisory and do not block compilation, but ignoring them is rarely correct. If you intend the ordering, restructure the code so the warning no longer triggers; suppressing it should be a last resort.

## Managing Both Public and Private State

Records are private state and can be created or consumed in the proof context (the entry `fn` body), but not inside `final { }` blocks or `final fn`. The finalization context runs purely on-chain and only has access to public on-chain state.

|                   | **Public State**     | **Private State**                 |
| ----------------- | -------------------- | --------------------------------- |
| **Where it runs** | `final { }` block    | Entry `fn` body                   |
| **Data Storage**  | `mapping`, `storage` | `record`                          |
| **Visibility**    | everyone             | visible if you have the `viewkey` |
