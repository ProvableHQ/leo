---
id: diagnostics
title: Compiler Diagnostics
sidebar_label: Compiler Diagnostics
---

[general tags]: # "diagnostics, warnings, unused, dead_code, allow"

Leo emits `rustc`-style warnings during compilation. Warnings do not stop the
build — the compiled bytecode is still produced — but they should be addressed
before deploying a program.

## The `UNU` family — unused items

The `UnusedItems` pass runs after type checking and reports definitions or
bindings that the program never reads. Wording mirrors `rustc`'s `dead_code`,
`unused_variables`, and `unused_imports` lints.

### What is checked

| Item                                                                                           | Message                                       |
| ---------------------------------------------------------------------------------------------- | --------------------------------------------- |
| Free `fn` (non-entry, non-`@test`) and `final fn`                                              | function `foo` is never used                  |
| Local `let`, function parameter, loop iterator, tuple-pattern element, const-generic parameter | unused variable: `x`                          |
| `struct` (non-record), including transitive deadness                                           | struct `Foo` is never constructed             |
| `import`                                                                                       | unused import: `bar.aleo`                     |
| `const` at any scope                                                                           | constant `X` is never used                    |
| Leading-`_` binding that was read after all                                                    | used binding `_x` whose name begins with `_`  |

**Transitive deadness** applies to structs: if `struct A { f: B }` is unused
and `struct B` is referenced only by `A`, both `A` and `B` are reported.

### What is not checked

The following item kinds are intentionally **not** warned on:

- **Entry points and `view fn`s.** Transition entry points and read-only
  `view fn`s are externally callable, so their callers live in other
  compilation units; their names and parameters are never flagged.
- **Library public surface.** When a *library* is built directly, every
  top-level item — in the entry file and in submodules alike — is reachable by
  a consumer as `lib::item` or `lib::submodule::item`, so item-level functions,
  consts, and structs are never flagged. Unused *local* bindings inside those
  functions are still reported.
- **Records.** Records are part of a program's public surface; they may be
  constructed by callers outside the current package.
- **Struct and record fields.** Field-level dead-code detection requires
  data-flow tracking that the pass does not yet perform.
- **Mappings and storage variables.** These are read and written through
  intrinsics and storage operations the pass does not track.
- **Interfaces, function prototypes, and record prototypes.** These exist to
  describe an external surface; they have no body to be "used" locally.

Field, mapping, and storage diagnostics are blocked on data-flow-aware tracking
plus an `@allow_unused` attribute for the legitimate false-positive cases.

### Silencing a warning

Prefix the offending name with a single underscore (`_x`, `_HELPER`,
`_MAX_ITERS`). Where Leo permits a leading `_`, the corresponding `unused_*`
warning is suppressed:

```leo file=../code_snippets/diagnostics/silenced/src/main.leo#top_const title="Unused top-level const — silenced:"
```

```leo file=../code_snippets/diagnostics/silenced/src/main.leo#free_fn title="Orphan free fn — silenced:"
```

```leo file=../code_snippets/diagnostics/silenced/src/main.leo#let_binding title="Unused let binding — silenced:"
```

```leo file=../code_snippets/diagnostics/silenced/src/main.leo#local_const title="Unused local const — silenced:"
```

```leo file=../code_snippets/diagnostics/silenced/src/main.leo#loop_iter title="Unused loop iterator — silenced:"
```

```leo file=../code_snippets/diagnostics/silenced/src/main.leo#tuple_pattern title="Unused tuple-pattern element — silenced:"
```

```leo file=../code_snippets/diagnostics/silenced/src/main.leo#const_generic_and_param title="Unused parameter and const-generic — silenced:"
```

A leading `_` is **not** permitted in positions whose names reach the Aleo
bytecode (struct/record names, fields, mappings, storage variables, entry-point
functions, program names). See
[Identifiers](./structure.md#identifiers) for the full table of where a
leading underscore is and is not allowed.

There is no `#[allow(...)]`-style attribute for unused items yet; renaming with
a leading underscore is the only way to mark a binding "intentionally unused".

### The "used underscore" anti-pattern

If you mark a binding with a leading underscore to silence its
`unused_variable` warning and then read it, the silencing intent is broken and
the compiler reports a `used binding` warning:

```text
Warning: used binding `_y` whose name begins with `_`
  ...
  Help: Remove the leading `_` from the name, or stop reading the binding.
```

Either rename the binding (drop the leading `_`) or stop reading it.

## Imports cannot also be flagged unused

Unlike Rust, Leo does not warn on individual imported items. The `unused
import` warning fires at the granularity of a whole `import program.aleo;`
declaration. If any item from `program.aleo` is referenced anywhere in the
importing program — entry call, struct mention, constant read, submodule path
— the import is considered used.
