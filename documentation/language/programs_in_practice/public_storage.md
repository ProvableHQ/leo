---
id: public_state
title: Public State
sidebar_label: Public State
---

[general tags]: # "program, mapping, storage"

## Mappings

There are several functions available to query and modify mappings. The examples below will reference the following mapping:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#mapping_decl
```

### Querying

To simply check if a value has been set for a particular `address` in `balance`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#mapping_contains
```

To query a value for a particular `address` in `balance`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#mapping_get
```

Note that if value at `addr` does not exist above, then the program will fail to execute. To query a value with a fallback for this case:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#mapping_get_or_use
```

A program can also query values from another program's mappings:

```leo file=../../code_snippets/public_storage/credits_reader/src/main.leo#cross_program_reads
```

Although values can be queried, a program cannot directly modify another program's mappings.

### Modifying

To set a value for a particular `address` in `balance`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#mapping_set
```

To remove the value set at particular `address` in `balance`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#mapping_remove
```

### Usage

```leo file=../../code_snippets/public_storage/demo/src/main.leo#mapping_usage showLineNumbers
```

:::info
Mapping operations are only allowed inside a `final { }` block or inside a `final fn`.
:::

## Storage Variables

Storage variables behave similar to option types. There are several functions available to query and modify singleton storage variables. The examples below will reference the following:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_var_decl
```

### Querying

To query the value currently stored at `counter`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_var_unwrap
```

Note that if `counter` has not been initialized, then the program will fail to execute. To query the value with a fallback for this case:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_var_unwrap_or
```

### Modifying

To set a value for `counter`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_var_set
```

To unset the value at `counter`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_var_unset
```

### Usage

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_var_usage showLineNumbers
```

:::info
Storage variable operations are only allowed inside a `final { }` block or inside a `final fn`.
:::

### External Access

Storage variables defined in another program can be accessed using the fully qualified form `program_name.aleo::storage_name`. External storage variables are **read-only** and cannot be modified.

For example, suppose another program defines the following storage variable:

```leo file=../../code_snippets/public_storage/external_var_decl/src/main.leo#file
```

You may query this value from your program using:

```leo file=../../code_snippets/public_storage/external_consumer/src/main.leo#ext_var_unwrap
```

As with local storage variables, calling `unwrap()` will cause execution to fail if the storage variable has not been initialized. To safely query the value with a fallback:

```leo file=../../code_snippets/public_storage/external_consumer/src/main.leo#ext_var_unwrap_or
```

External storage variables cannot be assigned to or unset. The following operations are invalid:

```leo
external_program.aleo::counter = 5u64;   // invalid
external_program.aleo::counter = none;   // invalid
```

## Storage Vectors

Storage vectors behave like dynamic arrays of values of a given type. Several functions are available to query and modify storage vectors. The examples below reference the following declaration:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_decl
```

### Querying

To query the element currently stored in `id_numbers` at index `idx`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_get
```

This returns `u64?`. If `idx` is out of bounds, the result is `none`.

To get the current length of `id_numbers`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_len
```

This always returns a `u32` and cannot fail.

### Modifying

To set an element at index `idx` in `id_numbers`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_set
```

To push an element onto the end of `id_numbers`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_push
```

To pop and return the last element of `id_numbers`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_pop
```

To remove the element at index `idx`, return it, and replace it with the final element of `id_numbers`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_swap_remove
```

To clear every element in `id_numbers`:

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_clear
```

:::note

- `clear()` does not actually remove any values from the vector. It simply sets the length to `0`.
- Similarly, `swap_remove()` and `pop()` do not physically remove values. They reduce the length by `1`, ensuring the final element is no longer accessible.

  :::

### Usage

```leo file=../../code_snippets/public_storage/demo/src/main.leo#storage_vec_usage showLineNumbers
```

:::info
Storage vector operations are only allowed inside a `final { }` block or inside a `final fn`.
:::

### External Access

Storage vectors defined in another program can be accessed using the fully qualified form `program_name.aleo::storage_name`. External storage vectors are **read-only** and cannot be modified.

For example, suppose another program defines the following storage vector:

```leo file=../../code_snippets/public_storage/external_vec_decl/src/main.leo#file
```

You may query elements or the length of this vector from your program:

```leo file=../../code_snippets/public_storage/external_consumer/src/main.leo#ext_vec_reads
```

External storage vectors cannot be modified. The following operations are invalid:

```leo
external_program.aleo::id_numbers.push(5u64);        // invalid
external_program.aleo::id_numbers.set(0u32, 5u64);   // invalid
external_program.aleo::id_numbers.pop();             // invalid
external_program.aleo::id_numbers.swap_remove(0u32); // invalid
external_program.aleo::id_numbers.clear();           // invalid
```
