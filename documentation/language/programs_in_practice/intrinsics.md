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

```text
_dynamic_call::[TYPE_PARAMS](prog, net, func, ...args)
```

- `prog` — the target program name, as a value of type `identifier` or a `field` representing an identifier
- `net` — the network, as a value of type `identifier` or a `field` representing an identifier; currently only `'aleo'` is valid
- `func` — the function name to call, as a value of type `identifier` or a `field` representing an identifier
- `...args` — the function arguments, matching the input types declared in `TYPE_PARAMS`

### Type parameters

Type parameters follow one rule: **the last entry is the return type; all preceding entries are input types** with an optional visibility modifier (`public` or `private`). Omitting type parameters entirely means void return with compiler-inferred input visibility.

#### No type parameters — void return

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dc_void
```

#### Return type only — inputs are `private` by default

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dc_return
```

#### Explicit input visibility

All type params before the last are input types. Each can carry a visibility modifier:

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dc_public_input
```

#### Multiple inputs

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dc_multiple_inputs
```

#### Void return with input annotations

Use `()` as the return type:

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dc_void_annotated
```

#### `Final` return

When the called function returns a `Final`, include it in a tuple:

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dc_final
```

#### `dyn record` inputs and outputs

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dc_dyn_record
```

### Restrictions

`_dynamic_call` is **not** allowed in `final fn` functions or `final {}` blocks.

---

## `_dynamic_contains`

Checks whether a key exists in a mapping belonging to another program, determined at runtime. Only valid inside `final fn` functions or `final {}` blocks.

### Syntax

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dyn_contains
```

- `prog` — the target program name, as a value of type `identifier` or a `field` representing an identifier
- `net` — the network, as a value of type `identifier` or a `field` representing an identifier; currently only `'aleo'` is valid
- `mapping` — the mapping name on the target program, as a value of type `identifier` or a `field` representing an identifier
- `key` — a value whose type matches the target mapping's key type

Returns `true` if `key` is present, `false` otherwise.

### Example

```leo file=../../code_snippets/intrinsics/dynamic_contains/src/main.leo showLineNumbers
```

---

## `_dynamic_get`

Reads a value from a mapping belonging to another program, determined at runtime. Only valid inside `final fn` functions or `final {}` blocks.

Fails at runtime if the key is not present — use [`_dynamic_get_or_use`](#_dynamic_get_or_use) when the key may be absent.

### Syntax

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dyn_get
```

- `prog` — the target program name, as a value of type `identifier` or a `field` representing an identifier
- `net` — the network, as a value of type `identifier` or a `field` representing an identifier; currently only `'aleo'` is valid
- `mapping` — the mapping name on the target program, as a value of type `identifier` or a `field` representing an identifier
- `key` — a value whose type matches the target mapping's key type
- `T` — must match the target mapping's value type

### Example

```leo file=../../code_snippets/intrinsics/dynamic_get/src/main.leo showLineNumbers
```

For example, using identifier literals:

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dyn_get_literal
```

---

## `_dynamic_get_or_use`

Reads a value from a mapping belonging to another program, determined at runtime, returning a default value if the key is absent. Only valid inside `final fn` functions or `final {}` blocks.

### Syntax

```leo file=../../code_snippets/intrinsics/dynamic_call_demos/src/main.leo#dyn_get_or_use
```

- `prog` — the target program name, as a value of type `identifier` or a `field` representing an identifier
- `net` — the network, as a value of type `identifier` or a `field` representing an identifier; currently only `'aleo'` is valid
- `mapping` — the mapping name on the target program, as a value of type `identifier` or a `field` representing an identifier
- `key` — a value whose type matches the target mapping's key type
- `default` — the fallback value returned when `key` is absent; must be the same type as `T`
- `T` — must match the target mapping's value type

### Example

```leo file=../../code_snippets/intrinsics/dynamic_get_or_use/src/main.leo showLineNumbers
```
