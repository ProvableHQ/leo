---
id: functions
title: Functions
sidebar_label: Functions
---

[general tags]: # "fn, final, entry_function, helper_function, final_fn"

## Entry Functions

Entry functions in Leo are declared as `fn {name}() {}` inside a `program {}` block. They define the program's public interface and can be called directly when running a Leo program (via `leo run`). If they include a `final { }` block to execute code on-chain, they must return `Final`.

```leo showLineNumbers
program hello.aleo {
    fn foo(
        public a: field,
        b: field,
    ) -> field {
        return a + b;
    }
}
```

### Inputs

Inputs are declared as `{visibility} {name}: {type}`. They must be declared just after the function name declaration, in parentheses.

```leo showLineNumbers
// The entry function `foo` takes a single input `a` with type `field` and visibility `public`.
fn foo(public a: field) { }
```

### Outputs

The return type of the function is declared as `-> {expression}` and must be declared just after the function inputs.
A function output is calculated as `return {expression};`. Returning an output ends the execution of the function, and the type of the returned value must match the output type declared in the function signature.

```leo showLineNumbers
fn foo(public a: field) -> field {
    // Returns the addition of the public input a and the value `1field`.
    return a + 1field;
}
```

## On-chain State with `final { }`

A `final { }` block is used to define computation that gets executed on-chain. The most common use case is to initiate or change public on-chain state within mappings or storage.

An entry `fn` that includes on-chain logic returns `Final` and embeds the on-chain code in a `final { }` block. Final blocks are atomic; they either succeed or fail, and state is reverted on failure.

```leo showLineNumbers
program transfer.aleo {
    // The function `transfer_public_to_private` turns a specified token amount
    // from `account` into a token record for the specified receiver.
    //
    // This function preserves privacy for the receiver's record, however
    // it publicly reveals the sender and the specified token amount.
    fn transfer_public_to_private(
        receiver: address,
        public amount: u64
    ) -> (token, Final) {
        // Produce a token record for the token receiver.
        let new: token = token {
            owner: receiver,
            amount,
        };

        // Return the receiver's record, then decrement the token amount of the caller publicly.
        return (new, final {
            // Decrements `account[sender]` by `amount`.
            // If `account[sender]` does not exist, it will be created.
            // If `account[sender] - amount` underflows, `transfer_public_to_private` is reverted.
            let current_amount: u64 = Mapping::get_or_use(account, self.caller, 0u64);
            Mapping::set(account, self.caller, current_amount - amount);
        });
    }
}
```

If there is no need to create or alter the public on-chain state, a `final { }` block is not required.

## On-chain State with `final fn`

When finalization logic is shared across multiple entry functions, it can be extracted into a `final fn`, declared outside the `program {}` block. A `final fn` call must still be wrapped in a `final { }` block at the call site:

```leo showLineNumbers
final fn decrement_balance(sender: address, amount: u64) {
    let current_amount: u64 = Mapping::get_or_use(account, sender, 0u64);
    Mapping::set(account, sender, current_amount - amount);
}

program transfer.aleo {
    fn transfer_public_to_private(
        receiver: address,
        public amount: u64
    ) -> (token, Final) {
        let new: token = token {
            owner: receiver,
            amount,
        };

        return (new, final {
            decrement_balance(self.caller, amount);
        });
    }

    fn burn(public amount: u64) -> Final {
        return final {
            decrement_balance(self.caller, amount);
        };
    }
}
```

The body of `decrement_balance` is inlined into each caller's `final { }` block at compile time — no shared function exists in the compiled output.

## Helper Function

A helper function is declared as `fn {name}({arguments}) {}` **outside** the `program {}` block.
They contain expressions and statements that can compute values, but cannot produce `records`.

Helper functions cannot be called directly from outside the program. Instead, they are called by entry functions.
Inputs of helper functions cannot have `{visibility}` modifiers, since they are used only internally, not as part of a program's external interface.

```leo showLineNumbers
fn foo(
    a: field,
    b: field,
) -> field {
    return a + b;
}
```

Helper functions also support **const generics**:

```leo showLineNumbers
fn sum_first_n_ints::[N: u32]() -> u32 {
    let sum = 0u32;
    for i in 0u32..N {
        sum += i;
    }
    return sum;
}

program main.aleo {
    fn main() -> u32 {
        return sum_first_n_ints::[5u32]();
    }
}
```

Acceptable types for const generic parameters include integer types, `bool`, `scalar`, `group`, `field`, and `address`.

:::note
Const generic parameters are only valid on inlinable helper `fn` functions. They are not permitted on entry point functions inside a `program {}` block, `final fn` functions, functions annotated with `@no_inline`, or function signatures declared inside an `interface`.
:::

### The `@no_inline` Annotation

By default the compiler inlines helper functions that are called only once, which reduces call overhead. To prevent this, annotate the function with `@no_inline`:

```leo
@no_inline
fn expensive_helper(a: u32, b: u32) -> u32 {
    // ...
    return a + b;
}
```

Use `@no_inline` when the function is intentionally shared across multiple call sites but the compiler would otherwise duplicate it, or when you want to preserve the function boundary for readability in the compiled output.

## Function Call Rules

- An entry `fn` can call: helper `fn`, `final fn`, and external entry `fn`s.
- A helper `fn` can only call: other helper `fn`s.
- A `final fn` can only call: other `final fn`s.
- Recursive calls (direct or indirect) are not allowed.
