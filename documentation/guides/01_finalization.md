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

You can access the inputs to an external `Final` using the following syntax:

```leo
let f = imported_program.aleo::some_function();
let value = f.0;  // or f.1, f.2, f.3 and so on depending on the input index
```

## Managing Both Public and Private State

Records are private state and can be created or consumed in the proof context (the entry `fn` body), but not inside `final { }` blocks or `final fn`. The finalization context runs purely on-chain and only has access to public on-chain state.

|                   | **Public State**     | **Private State**                 |
| ----------------- | -------------------- | --------------------------------- |
| **Where it runs** | `final { }` block    | Entry `fn` body                   |
| **Data Storage**  | `mapping`, `storage` | `record`                          |
| **Visibility**    | everyone             | visible if you have the `viewkey` |
