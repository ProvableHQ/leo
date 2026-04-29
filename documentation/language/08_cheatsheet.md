---
id: cheatsheet
title: Leo Syntax Cheatsheet
sidebar: Cheatsheet
toc_min_heading_level: 2
toc_max_heading_level: 2
---

[general tags]: # "program, import, boolean, integer, field, group, scalar, address, signature, array, tuple, struct, operators, cryptographic_operators, assert, hash, commit, random, address, block, mapping, conditionals, loops"

## 1. File Import

```leo
import foo.aleo;
```

## 2. Programs

```leo
program hello.aleo {
    // code
}
```

## 3. Primitive Data Types

```leo file=../code_snippets/cheatsheet/main/src/main.leo#primitives
```

### Type Casting

```leo file=../code_snippets/cheatsheet/main/src/main.leo#type_casting
```

The primitive types are: `address`, `bool`, `field`, `group`, `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`, `scalar`.

We can cast between all of these types except `signature`.

You can cast an `address` to a `field` but not vice versa.

### Option Types

```leo file=../code_snippets/cheatsheet/main/src/main.leo#option_primitives
```

Both `address` and `signature` types do not have option variants.

## 4. Records

Defining a `record`:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#record_definition
```

Creating a `record`:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#record_creation
```

Accessing `record` fields:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#record_access
```

## 5. Structs

Defining a `struct`:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#struct_definition
```

Creating an instance of a `struct`:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#struct_creation
```

Accessing `struct` Fields:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#struct_access
```

A struct `ExternalStruct` defined in program `external_program.aleo` can be referred to outside the program using the syntax `external_program.aleo::ExternalStruct`.

### Const Generics

```leo file=../code_snippets/cheatsheet/main/src/main.leo#const_generics_struct
```

Note that generic structs cannot currently be imported outside a program, but can be declared and used in submodules. Acceptable types for const generic parameters include integer types, `bool`, `scalar`, `group`, `field`, and `address`.

### Option Types

Creating an option type instance of a `struct`

```leo file=../code_snippets/cheatsheet/main/src/main.leo#option_struct
```

Note that because the `address` and `signature` types do not have option variants, a `struct` containing elements of these types also cannot have an option variant.

## 6. Arrays

Declaring `arrays`:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#array_decl
```

Accessing elements:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#array_access
```

Looping over arrays:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#array_loop
```

## 7. Tuples

Declaring tuples:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#tuple_decl
```

Accessing tuple elements:

```leo file=../code_snippets/cheatsheet/main/src/main.leo#tuple_access
```

## 8. Functions

There are three kinds of functions in Leo 4.0:

1. **Entry `fn`** (inside `program {}`): the program's public interface, callable from outside.
2. **Helper `fn`** (outside `program {}`): private helpers used by entry functions.
3. **`final fn`** (outside `program {}`): reusable finalization logic, inlined into `final { }` blocks at compile time.

**Direct/indirect recursive calls are not allowed.**

### Helper `fn`

A helper `fn` is used for **computations**. Declared outside `program {}`.

```leo file=../code_snippets/cheatsheet/main/src/main.leo#helper_fn
```

#### Const Generics

```leo file=../code_snippets/cheatsheet/const_generics_fn/src/main.leo#const_generics_fn
```

Acceptable types for const generic parameters include integer types, `bool`, `scalar`, `group`, `field`, and `address`.

✅ Can call: helper `fn`

❌ Cannot call: entry `fn`

### Entry `fn`

An entry `fn` is the program's **public interface**. Declared inside `program {}`. It can call helper `fn` and include `final { }` blocks for on-chain state updates.

```leo file=../code_snippets/cheatsheet/entry_calls_helper/src/main.leo#entry_calls_helper
```

✅ Can call: helper `fn`

❌ Cannot call: another entry `fn` (unless from another program)

### Entry `fn` with `final { }` (on-chain state)

An entry `fn` that also modifies **public on-chain state** returns `Final` and includes a `final { }` block.

```leo file=../code_snippets/cheatsheet/mint_with_final/src/main.leo#mint_with_final
```

✅ Can call: helper `fn`, `final fn`

❌ Cannot call: another entry `fn` (unless from another program)

### `final fn`

A `final fn` contains reusable finalization logic. It is **always inlined** into the caller's `final { }` block at compile time. Declared outside `program {}`.

```leo file=../code_snippets/cheatsheet/main/src/main.leo#final_fn_def
```

✅ Can call: other `final fn`

❌ Cannot call: helper `fn` or entry `fn`

## 9. Loops

```leo file=../code_snippets/cheatsheet/main/src/main.leo#loops
```

## 10. Conditionals

```leo file=../code_snippets/cheatsheet/main/src/main.leo#conditionals
```

## 11. Onchain Storage

### Mappings

```leo file=../code_snippets/cheatsheet/main/src/main.leo#mappings
```

### Storage Variables

```leo file=../code_snippets/cheatsheet/main/src/main.leo#storage_var
```

### Storage Vectors

```leo file=../code_snippets/cheatsheet/main/src/main.leo#storage_vec
```

## 12. Operators

### Standard

```leo file=../code_snippets/cheatsheet/main/src/main.leo#standard_operators
```

### Cryptographic

```leo file=../code_snippets/cheatsheet/main/src/main.leo#crypto_operators
```
