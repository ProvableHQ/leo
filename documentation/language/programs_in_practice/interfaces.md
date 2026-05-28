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

An interface can require the existence of a record by name. The shortest form is the **marker form**, which only requires that the implementor declare a record with that name and imposes no field constraints:

```leo file=../../code_snippets/interfaces/decl_foo_min/src/main.leo#file
```

To additionally require specific fields, list them inside braces. The list **must end with `..`**, which marks the prototype as a partial specification — implementors may declare any additional fields beyond those listed:

```leo file=../../code_snippets/interfaces/decl_foo_fields/src/main.leo#file
```

:::note
A field list without a trailing `..` is currently a parser error (``expected `..`; fully constraining record fields in interfaces is not yet supported``). Use the marker form (`record Bar;`) for "any record by this name", or the field list with `..` for "at least these fields plus optionally more". Strictly-constraining record prototypes are reserved for a future release.
:::

If `owner` is listed in a prototype, it must have type `address`. Listing `owner` is optional — every record carries an `owner: address` regardless.

### Inheritance and Composition

Interfaces can inherit from other interfaces using `:`:

```leo file=../../code_snippets/interfaces/inheritance/src/main.leo#file
```

Multiple interfaces can be composed together using `+`:

```leo file=../../code_snippets/interfaces/composition/src/main.leo#file
```

### Composition Rules

When a program implements multiple interfaces (via `+`) — or when an interface inherits from another interface (via `:`) — the compiler **flattens** the full set of inherited members into a single membership requirement and checks for conflicts member-by-member. Two interfaces that contribute the same member name must agree on its shape; otherwise compilation fails with a `conflicting interface member` error.

The matching rules per member kind:

- **Functions** with the same name must have identical parameter types and identical return types. Parameter names do not need to match.
- **Records** with the same name require the child prototype to list **every field the parent prototype lists**, with matching type and matching visibility mode (`public`, `private`, `constant`) for each. A field present in the parent prototype but absent from the child is itself a `conflicting_record_field` error — the trailing `..` lets the implementing program add fields beyond what the interface requires, not for one interface to silently omit a field another interface requires.
- **Mappings** with the same name must have identical key types **and** identical value types.
- **Storage variables** with the same name must have identical types.

If the contributions agree, the merged interface contains a single copy of the member. If they disagree, the compiler emits a `conflicting_interface_member` (or `conflicting_record_field`) error pointing at the inheriting interface or implementing program.

Cycles in interface inheritance are detected separately and rejected before flattening begins.

### Interfaces and `dyn record`

Interface declarations are a **compile-time** contract: the compiler checks that an implementing program's `record` declarations satisfy the interface's record prototypes. A `dyn record` value passed at runtime is **not** revalidated against any interface — its actual on-chain layout could carry any fields. When you read a field from a `dyn record`, the read halts if the field does not exist with the requested type. See [Reading Fields](#reading-fields) for the rules.

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

### Where `dyn record` May Appear

`dyn record` is a first-class type, but its uses are intentionally narrow. The compiler accepts it in:

- function parameters and return types (entry `fn`, helper `fn`, `final fn`),
- local `let` bindings,
- tuples returned from a function.

It is rejected in:

- `mapping` declarations and `storage` variables — public on-chain storage requires a concrete schema.
- `Optional` (`dyn record?`) — wrap concrete data instead.
- Ternary expressions (`flag ? a : b`) — the AVM has no select instruction for opaque records.
- Inside arrays, structs, or other composite types — `dyn record` only flows through top-level positions.

The most common pattern is to receive a `dyn record` as a function parameter, read whichever fields you need (with type annotations — see below), and either forward it to a dynamic call or pass it back out as the function's return value.

### Reading Fields

The `owner` field is always accessible and always typed `address`:

```leo file=../../code_snippets/interfaces/dyn_record_field_access/src/main.leo#check_owner
```

Any other field requires a **type annotation** at the point of access. The compiler accepts three sources for that annotation:

- A `let` binding with an explicit type:

  ```leo file=../../code_snippets/interfaces/dyn_record_field_access/src/main.leo#balance_via_let
  ```

- The enclosing function's declared return type (the type flows in from the return position):

  ```leo file=../../code_snippets/interfaces/dyn_record_field_access/src/main.leo#balance_via_return
  ```

- An explicit cast on the access:

  ```leo file=../../code_snippets/interfaces/dyn_record_field_access/src/main.leo#balance_via_cast
  ```

Reading a field with no annotation is a compile error:

```text
fn balance_no_annotation(r: dyn record) -> u64 {
    let amount = r.balance;   // error — type of `r.balance` cannot be inferred
    return amount;
}
```

A struct-typed field works the same way as a primitive — the binding must declare the struct type:

```leo file=../../code_snippets/interfaces/dyn_record_field_access/src/main.leo#meta_struct_field
```

If the access type at runtime does not match the type declared in the program (for instance, `let amount: u64 = r.balance` when the underlying record's `balance` field is actually a `field`), the call halts at runtime.

**Fields cannot be reassigned.** `r.balance = 100u64;` is a compile error.

### Converting Between Static Records and `dyn record`

Three conversion paths exist:

- **Static record → `dyn record`**: explicit cast with `as`. Only valid when the source value is a concrete `record`; casting a `struct`, integer, or any other non-record type is a compile error.

  ```leo file=../../code_snippets/interfaces/dyn_record_field_access/src/main.leo#cast_static_to_dyn
  ```

- **`dyn record` → static record (explicit)**: **not supported.** `r as Token` is rejected by the compiler. The runtime has no way to validate that `r`'s actual layout matches `Token`'s declared layout, so the language requires you to access fields one at a time with type annotations instead (see [Reading Fields](#reading-fields)).

- **`dyn record` → static record (implicit)**: happens at dynamic-call sites when an interface declares a concrete record parameter. See [Case D](#dynamic-records-and-dynamic-calls) below — the call performs the implicit narrowing for you, and the call halts at runtime if the actual record layout doesn't match.

For the inverse direction at call sites — the four ways static and dynamic records interact across an interface — see the four cases below.

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
