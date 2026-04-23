---
id: public_state
title: Public State
sidebar_label: Public State
---

[general tags]: # "program, mapping, storage"

## Mappings

There are several functions available to query and modify mappings. The examples below will reference the following mapping:

```leo
mapping balance: address => u64;
```

### Querying

To simply check if a value has been set for a particular `address` in `balance`:

```leo
balance.contains(addr)
Mapping::contains(balance, addr); // Alternate syntax
```

To query a value for a particular `address` in `balance`:

```leo
balance.get(addr)
Mapping::get(balance, addr); // Alternate syntax
```

Note that if value at `addr` does not exist above, then the program will fail to execute. To query a value with a fallback for this case:

```leo
balance.get_or_use(addr,fallback_value)
Mapping::get_or_use(balance, addr, fallback_value); // Alternate syntax
```

A program can also query values from another program's mappings:

```leo
let balance1 = credits.aleo::account.get(addr);
let balance2 = credits.aleo::account.get_or_use(addr, 0u64);
```

Although values can be queried, a program cannot directly modify another program's mappings.

### Modifying

To set a value for a particular `address` in `balance`:

```leo
balance.set(addr,value)
Mapping::set(balance, addr, value); // Alternate syntax
```

To remove the value set at particular `address` in `balance`:

```leo
balance.remove(addr)
Mapping::remove(balance, addr); // Alternate syntax
```

### Usage

```leo showLineNumbers
program map.aleo {
    mapping balance: address => u64;

    fn dubble() -> Final {
        let addr: address = self.caller;
        return final {
            let current_value: u64 = balance.get_or_use(addr, 0u64);
            balance.set(addr, current_value + 1u64);

            let next_current_value: u64 = balance.get(addr);
            balance.set(addr, next_current_value + 1u64);
        };
    }
}
```

:::info
Mapping operations are only allowed inside a `final { }` block or inside a `final fn`.
:::

## Storage Variables

Storage variables behave similar to option types. There are several functions available to query and modify singleton storage variables. The examples below will reference the following:

```leo
storage counter: u64;
```

### Querying

To query the value currently stored at `counter`:

```leo
counter.unwrap();
```

Note that if `counter` has not been initialized, then the program will fail to execute. To query the value with a fallback for this case:

```leo
counter.unwrap_or(fallback_value);
```

### Modifying

To set a value for `counter`:

```leo
counter = 5u64;
```

To unset the value at `counter`:

```leo
counter = none;
```

### Usage

```leo showLineNumbers
program storage_variable.aleo {
    storage counter: u64;

    fn increment() -> Final {
        return final {
            let current_value: u64 = counter.unwrap_or(0u64);
            counter = current_value + 1u64;
        };
    }
}
```

:::info
Storage variable operations are only allowed inside a `final { }` block or inside a `final fn`.
:::

### External Access

Storage variables defined in another program can be accessed using the fully qualified form `program_name.aleo::storage_name`. External storage variables are **read-only** and cannot be modified.

For example, suppose another program defines the following storage variable:

```leo
program external_program.aleo {
    storage counter: u64;

    ...
}
```

You may query this value from your program using:

```leo
let value: u64 = external_program.aleo::counter.unwrap();
```

As with local storage variables, calling `unwrap()` will cause execution to fail if the storage variable has not been initialized. To safely query the value with a fallback:

```leo
let value: u64 = external_program.aleo::counter.unwrap_or(0u64);
```

External storage variables cannot be assigned to or unset. The following operations are invalid:

```leo
external_program.aleo::counter = 5u64;   // invalid
external_program.aleo::counter = none;   // invalid
```

## Storage Vectors

Storage vectors behave like dynamic arrays of values of a given type. Several functions are available to query and modify storage vectors. The examples below reference the following declaration:

```leo
storage id_numbers: [u64];
```

### Querying

To query the element currently stored in `id_numbers` at index `idx`:

```leo
id_numbers.get(idx);
```

This returns `u64?`. If `idx` is out of bounds, the result is `none`.

To get the current length of `id_numbers`:

```leo
id_numbers.len();
```

This always returns a `u32` and cannot fail.

### Modifying

To set an element at index `idx` in `id_numbers`:

```leo
id_numbers.set(idx, value);
```

To push an element onto the end of `id_numbers`:

```leo
id_numbers.push(value);
```

To pop and return the last element of `id_numbers`:

```leo
id_numbers.pop();
```

To remove the element at index `idx`, return it, and replace it with the final element of `id_numbers`:

```leo
id_numbers.swap_remove(idx);
```

To clear every element in `id_numbers`:

```leo
id_numbers.clear();
```

:::note

- `clear()` does not actually remove any values from the vector. It simply sets the length to `0`.
- Similarly, `swap_remove()` and `pop()` do not physically remove values. They reduce the length by `1`, ensuring the final element is no longer accessible.
  :::

### Usage

```leo showLineNumbers
program storage_vector.aleo {
    storage id_numbers: [u64];

    fn add_id(new_id: u64) -> Final {
        return final {
            id_numbers.push(new_id);
        };
    }

    fn remove_id(idx: u32) -> Final {
        return final {
            id_numbers.swap_remove(idx);
        };
    }
}
```

:::info
Storage vector operations are only allowed inside a `final { }` block or inside a `final fn`.
:::

### External Access

Storage vectors defined in another program can be accessed using the fully qualified form `program_name.aleo::storage_name`. External storage vectors are **read-only** and cannot be modified.

For example, suppose another program defines the following storage vector:

```leo
program external_program.aleo {
    storage id_numbers: [u64];
}
```

You may query elements or the length of this vector from your program:

```leo
let first: u64? = external_program.aleo::id_numbers.get(0u32);
let length: u32 = external_program.aleo::id_numbers.len();
```

External storage vectors cannot be modified. The following operations are invalid:

```leo
external_program.aleo::id_numbers.push(5u64);        // invalid
external_program.aleo::id_numbers.set(0u32, 5u64);   // invalid
external_program.aleo::id_numbers.pop();             // invalid
external_program.aleo::id_numbers.swap_remove(0u32); // invalid
external_program.aleo::id_numbers.clear();           // invalid
```
