---
id: interfaces
title: Interfaces, Dynamic Calls & Dynamic Records
sidebar_label: Interfaces & Dynamic Dispatch
---

[general tags]: # "interface, dynamic_dispatch, dynamic_record, dyn_record, polymorphism, identifier"

Leo provides three related features for building composable, generic programs:

- **Interfaces** — declare a named contract that programs must fulfill.
- **Dynamic Calls** — call into a program determined at runtime.
- **Dynamic Records** — pass and inspect records whose structure is unknown at compile time.

## Interfaces

An `interface` declaration specifies a set of functions, records, mappings, and storage variables that a
program must provide. Interfaces are a compile-time concept and have no impact on the bytecode generated.
They are only useful as a way to enforce structural contracts — ensuring that any program claiming to implement
an interface actually provides all required functions, records, mappings, and storage variables — and to enable
dynamic calls, where the caller knows *what* it can call without knowing *which* program it is calling at
runtime. Interfaces can be declared outside the `program {}` block, in a submodule, or in a library package
(including library submodules).

```leo file=../../code_snippets/interfaces/decl_transfer/src/main.leo#file
```

### Implementing an Interface

A program implements an interface by listing it after `:` in the program declaration. The compiler checks that the program provides everything the interface requires.

```leo file=../../code_snippets/interfaces/transfer/src/main.leo#program
```

### Implementing Multiple Interfaces

A program can implement multiple interfaces at once using `+`:

```leo file=../../code_snippets/interfaces/multi/src/main.leo#program
```

### Record Requirements

An interface can require the existence of a record by name:

```leo file=../../code_snippets/interfaces/decl_foo_min/src/main.leo#file
```

It can also require that the record has specific fields. Use `..` to indicate that implementors may declare additional fields beyond those required:

```leo file=../../code_snippets/interfaces/decl_foo_fields/src/main.leo#file
```

### Inheritance and Composition

Interfaces can inherit from other interfaces using `:`:

```leo file=../../code_snippets/interfaces/inheritance/src/main.leo#file
```

Multiple interfaces can be composed together using `+`:

```leo file=../../code_snippets/interfaces/composition/src/main.leo#file
```

## Dynamic Calls

Static calls require the callee program to be known at compile time:

```leo file=../../code_snippets/interfaces/static_call_caller/src/main.leo#snippet
```

Dynamic calls allow the callee to be determined at runtime. The caller still knows *what* it can call — expressed as an interface — but not *which* program it is calling:

```leo file=../../code_snippets/interfaces/dynamic_call/src/main.leo#dynamic_call
```

The syntax is:

```text
Interface@(target)::method(args)
```

where:

- `Interface` is the interface name.
- `target` is an `identifier` value (or `field`) resolved at runtime — the name of the program to call into.
- `method` is the function to invoke.

### The `identifier` Type

The `identifier` type represents a program name resolved at runtime. An `identifier` literal uses single-quote syntax:

```leo file=../../code_snippets/interfaces/dynamic_call/src/main.leo#identifier_literal
```

By default the target is looked up on the `aleo` network. To specify a different network explicitly, pass a second `identifier` as a second argument:

```leo file=../../code_snippets/interfaces/dynamic_call/src/main.leo#identifier_with_network
```

:::note
The only valid network identifier currently is `aleo`.
:::

### Dynamic Mapping Reads

An interface that declares a `mapping` can also be used to read that mapping on a runtime-determined program. The syntax mirrors dynamic calls, but with a mapping name in place of a method name and a trailing read operation:

```text
Interface@(target[, network])::mapping.get(key)
Interface@(target[, network])::mapping.contains(key)
Interface@(target[, network])::mapping.get_or_use(key, default)
```

- `.get(key)` returns the mapped value; the transition fails at runtime if `key` is not present.
- `.contains(key)` returns a `bool`.
- `.get_or_use(key, default)` returns the mapped value, or `default` if `key` is absent.

These reads are only valid inside a `final fn` or a `final {}` block — they lower to the AVM `get.dynamic`, `contains.dynamic`, and `get.or_use.dynamic` instructions. Dynamic *writes* are not supported.

`bank.aleo` declares the `Bank` interface and implements it:

```leo file=../../code_snippets/interfaces/bank/src/main.leo#file title="bank/src/main.leo"
```

A second program imports `bank.aleo` and reads its mapping through the interface. Since the read is cross-program, the interface name is qualified with `bank.aleo::`:

```leo file=../../code_snippets/interfaces/bank_reader/src/main.leo#file title="checker/src/main.leo"
```

When the reader is inside the same program that declares the interface, drop the program qualifier — `Bank@(target)::balances.get(key)` rather than `bank.aleo::Bank@(target)::balances.get(key)`.

### Dynamic Storage Reads

Interfaces that declare [`storage`](../02_structure.md#storage) variables support dynamic reads with the same pattern. Storage reads always return an `Option<T>`:

```text
Interface@(target[, network])::singleton            // Option<T>
Interface@(target[, network])::vector.get(index)    // Option<T>
Interface@(target[, network])::vector.len()         // u32
```

Singleton storage is read by naming the variable directly (no trailing `.op(...)`). Vector storage supports `.get(index)` (out-of-bounds reads return `none`) and `.len()` (no arguments). Storage writes through the dynamic interface are not supported — use a dynamic call to an entry function that performs the write.

`logger.aleo` declares the `Logger` interface and implements it:

```leo file=../../code_snippets/interfaces/logger/src/main.leo#file title="logger/src/main.leo"
```

A second program imports `logger.aleo` and reads its storage variables through the interface. Since the read is cross-program, the interface name is qualified with `logger.aleo::`:

```leo file=../../code_snippets/interfaces/logger_reader/src/main.leo#file title="reader/src/main.leo"
```

:::note
Dynamic mapping reads are a type-checked alternative to the [`_dynamic_get`, `_dynamic_contains`, and `_dynamic_get_or_use`](./intrinsics.md) intrinsics. The interface form checks that the named mapping exists on the interface and that keys, values, and defaults have matching types; the intrinsics accept arbitrary runtime identifiers and leave that responsibility to the caller. Prefer the interface form whenever an interface is available.
:::

## Dynamic Records

A `dyn record` is a record whose field structure is not known at compile time. It retains all the ownership and privacy properties of a regular record:

```leo file=../../code_snippets/interfaces/dyn_record_helper/src/main.leo#snippet
```

`dyn record` complements dynamic calls: while dynamic calls allow a program to route logic to any compliant callee, `dyn record` allows that same program to accept, inspect, and forward records from programs it has never seen at compile time, without losing the safety guarantees of the type system.

### Dynamic Records and Dynamic Calls

Regardless of what the interface signature says, dynamic calls always take dynamic records as inputs and return dynamic records as outputs.

When making a dynamic call, all record arguments are treated as `dyn record` under the hood, and all record return values come back as `dyn record` — even when the interface uses a concrete static record type. There are four cases depending on what the interface declares and what the caller provides:

**Case A — Interface expects `dyn record`, caller has `dyn record`**

Pass the dynamic record directly with no conversion needed.

```leo file=../../code_snippets/interfaces/case_a/src/main.leo#file
```

**Case B — Interface expects `dyn record`, caller has a static record**

Convert the static record explicitly to `dyn record` using `as` before passing it.

```leo file=../../code_snippets/interfaces/case_b/src/main.leo#file
```

**Case C — Interface expects a static record, caller has a static record**

Leo implicitly converts the static record to `dyn record` at the call site. Nothing extra is required from the caller, though an implicit unsafe step occurs under the hood.

```leo
interface ARC20 {
    record Token;
    fn transfer_private(token: Token, to: address) -> Token;
}

program caller.aleo {
    record Token { owner: address, amount: u64 }

    fn main(target: identifier, token: Token, to: address) -> dyn record {
        return ARC20@(target)::transfer_private(token, to); // implicit conversion under the hood
    }
}
```

**Case D — Interface expects a static record, caller has a `dyn record`**

Leo implicitly casts the dynamic record to the expected static type at the call site. The return value is still `dyn record`.

```leo file=../../code_snippets/interfaces/case_d/src/main.leo#file
```

In all four cases, the return type of a dynamic call that involves records is always `dyn record`, regardless of what the interface declares.
