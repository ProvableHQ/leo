---
id: intrinsics
title: Intrinsics
sidebar_label: Intrinsics
---

[general tags]: # "intrinsic, dynamic_call, dynamic_contains, dynamic_get, dynamic_get_or_use, dynamic_dispatch, finalize"

Intrinsics are low-level operations built into the compiler. They complement Leo's high-level abstractions and are useful when those abstractions aren't expressive enough for a particular use case. They are prefixed with `_` to make them visually distinct from user-defined functions.

---

## `_dynamic_call`

Calls a function on a remote program determined at runtime, without requiring an interface definition. The target program, network, and function name are supplied as runtime values, and the full type signature is declared in the generic type parameters.

Only valid in transition contexts — it cannot be used inside `final fn` functions or `final {}` blocks.

### Syntax

```
_dynamic_call::[TYPE_PARAMS](prog, net, func, ...args)
```

- `prog` — the target program name, as a value of type `identifier` or a `field` representing an identifier
- `net` — the network, as a value of type `identifier` or a `field` representing an identifier; currently only `'aleo'` is valid
- `func` — the function name to call, as a value of type `identifier` or a `field` representing an identifier
- `...args` — the function arguments, matching the input types declared in `TYPE_PARAMS`

### Type parameters

Type parameters follow one rule: **the last entry is the return type; all preceding entries are input types** with an optional visibility modifier (`public` or `private`). Omitting type parameters entirely means void return with compiler-inferred input visibility.

#### No type parameters — void return

```leo
_dynamic_call(prog, net, func, x);
```

#### Return type only — inputs are `private` by default

```leo
let result: u64 = _dynamic_call::[u64](prog, net, func, x);
```

#### Explicit input visibility

All type params before the last are input types. Each can carry a visibility modifier:

```leo
// One public input, then the return type
let result: u64 = _dynamic_call::[public u64, u64](prog, net, func, x);
```

#### Multiple inputs

```leo
let (a, b): (u32, u32) = _dynamic_call::[public u32, public u32, (u32, u32)](prog, net, func, x, y);
```

#### Void return with input annotations

Use `()` as the return type:

```leo
_dynamic_call::[public u64, ()](prog, net, func, x);
```

#### `Final` return

When the called function returns a `Final`, include it in a tuple:

```leo
let (val, f): (u32, Final) = _dynamic_call::[(u32, Final)](target, 'aleo', 'increment', val);
return (val, final {
    f.run();
});
```

#### `dyn record` inputs and outputs

```leo
let result: dyn record = _dynamic_call::[dyn record, dyn record](prog, net, func, token);
```

### Restrictions

`_dynamic_call` is **not** allowed in `final fn` functions or `final {}` blocks.

---

## `_dynamic_contains`

Checks whether a key exists in a mapping belonging to another program, determined at runtime. Only valid inside `final fn` functions or `final {}` blocks.

### Syntax

```leo
let exists: bool = _dynamic_contains(prog, net, mapping, key);
```

- `prog` — the target program name, as a value of type `identifier` or a `field` representing an identifier
- `net` — the network, as a value of type `identifier` or a `field` representing an identifier; currently only `'aleo'` is valid
- `mapping` — the mapping name on the target program, as a value of type `identifier` or a `field` representing an identifier
- `key` — a value whose type matches the target mapping's key type

Returns `true` if `key` is present, `false` otherwise.

### Example

```leo showLineNumbers
program checker.aleo {
    mapping seen: address => bool;

    fn check(prog: field, net: field, map: field, key: address) -> Final {
        return final { finalize_check(prog, net, map, key); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_check(prog: field, net: field, map: field, key: address) {
    let exists: bool = _dynamic_contains(prog, net, map, key);
    Mapping::set(seen, key, exists);
}
```

---

## `_dynamic_get`

Reads a value from a mapping belonging to another program, determined at runtime. Only valid inside `final fn` functions or `final {}` blocks.

Fails at runtime if the key is not present — use [`_dynamic_get_or_use`](#_dynamic_get_or_use) when the key may be absent.

### Syntax

```leo
let val: T = _dynamic_get::[T](prog, net, mapping, key);
```

- `prog` — the target program name, as a value of type `identifier` or a `field` representing an identifier
- `net` — the network, as a value of type `identifier` or a `field` representing an identifier; currently only `'aleo'` is valid
- `mapping` — the mapping name on the target program, as a value of type `identifier` or a `field` representing an identifier
- `key` — a value whose type matches the target mapping's key type
- `T` — must match the target mapping's value type

### Example

```leo showLineNumbers
program reader.aleo {
    mapping result: address => u64;

    fn read(prog: field, net: field, map: field, key: address) -> Final {
        return final { finalize_read(prog, net, map, key); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_read(prog: field, net: field, map: field, key: address) {
    let val: u64 = _dynamic_get::[u64](prog, net, map, key);
    Mapping::set(result, key, val);
}
```

For example, using identifier literals:

```leo
let val: u64 = _dynamic_get::[u64]('some_program', 'aleo', 'balances', key);
```

---

## `_dynamic_get_or_use`

Reads a value from a mapping belonging to another program, determined at runtime, returning a default value if the key is absent. Only valid inside `final fn` functions or `final {}` blocks.

### Syntax

```leo
let val: T = _dynamic_get_or_use::[T](prog, net, mapping, key, default);
```

- `prog` — the target program name, as a value of type `identifier` or a `field` representing an identifier
- `net` — the network, as a value of type `identifier` or a `field` representing an identifier; currently only `'aleo'` is valid
- `mapping` — the mapping name on the target program, as a value of type `identifier` or a `field` representing an identifier
- `key` — a value whose type matches the target mapping's key type
- `default` — the fallback value returned when `key` is absent; must be the same type as `T`
- `T` — must match the target mapping's value type

### Example

```leo showLineNumbers
program reader.aleo {
    mapping result: address => u64;

    fn read(prog: field, net: field, map: field, key: address) -> Final {
        return final { finalize_read(prog, net, map, key); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_read(prog: field, net: field, map: field, key: address) {
    let val: u64 = _dynamic_get_or_use::[u64](prog, net, map, key, 0u64);
    Mapping::set(result, key, val);
}
```
