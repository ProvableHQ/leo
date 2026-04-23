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

```leo
program first_public_state.aleo {
    mapping accumulator: u8 => u64;
    storage count: u8;
    storage queue: [u8];

    //=============================================================
    //               MAPPING MODIFICATION
    //=============================================================
    fn increment_accumulator() -> Final {
        return final {
            let current_count: u64 = accumulator.get_or_use(0u8, 0u64); // Get current value, defaults to 0
            let new_count: u64 = current_count + 1u64;
            accumulator.set(0u8, new_count);
        };
    }

    //=============================================================
    //            STORAGE VARIABLE MODIFICATION
    //=============================================================
    fn increment_count() -> Final {
        return final {
            let current_count: u8 = count.unwrap_or(0u8); // Get current value, defaults to 0
            count = current_count + 1u8;
        };
    }

    //=============================================================
    //            STORAGE VECTOR MODIFICATION
    //=============================================================
    fn add_to_queue(val: u8) -> Final {
        return final {
            queue.push(val); // Push to end of queue
        };
    }
}
```

Entry functions with on-chain logic return `Final` and embed the finalization code in an inline `final { }` block. A few additional rules apply:

- Entry functions can return additional data types in a tuple, including Records, along with a `Final`.
- Only one `Final` can be returned.
- If multiple types are returned, the `Final` must be the last type in the tuple.

## External Calls

Leo enables developers to call entry functions from imported programs. A call to an external entry function that returns `Final` produces a `Final` value which can be composed inside a `final { }` block using the `.run()` method:

```leo
import first_public_storage.aleo;

program second_public_storage.aleo {
    mapping hashes: u8 => scalar;

    fn two_mappings(value: u8) -> Final {
        let increment_final: Final = first_public_storage.aleo::increment();
        return final {
            increment_final.run();
            let hash: scalar = BHP256::hash_to_scalar(value);
            hashes.set(value, hash);
        };
    }
}
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
