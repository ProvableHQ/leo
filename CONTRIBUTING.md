# Contributing

Thank you for your interest in contributing to Leo! Below you can find some guidelines that the project strives to follow.

## Pull requests

Please follow the instructions below when filing pull requests:

- Ensure that your branch is forked from the current [testnet3](https://github.com/AleoHQ/leo/tree/testnet3) branch.
- Fill out the provided markdown template for the feature or proposal. Be sure to link the pull request to any issues by using keywords. Example: "closes #130".
- Run `cargo fmt` before you commit; we use the `nightly` version of `rustfmt` to format the code, so you'll need to have the `nightly` toolchain installed on your machine; there's a [git hook](https://git-scm.com/docs/githooks) that ensures proper formatting before any commits can be made, and [`.rustfmt.toml`](https://github.com/AleoHQ/Leo/blob/testnet3/.rustfmt.toml) specifies some of the formatting conventions.
- Run `cargo clippy` to ensure that popular correctness and performance pitfalls are avoided.

## Style

These guidelines ensure consistent readable rust code within the Leo repository.

### Comments

Prefer line comments (//) to block comments (/* ... */).

When using single-line block comments there should be a single space after the opening sigil and before the closing sigil. Multi-line block comments should have a newline after the opening sigil and before the closing sigil.

Prefer to put a comment on its own line. Where a comment follows code, there should be a single space before it. Where a block comment is inline, there should be surrounding whitespace as if it were an identifier or keyword. There should be no trailing whitespace after a comment or at the end of any line in a multi-line comment. Examples:

```rust
// A comment on an item.
struct Foo { ... }

fn foo() {} // A comment after an item.

pub fn foo(/* a comment before an argument */ x: T) {...}
```

Comments should be complete sentences. Start with a capital letter, end with a period (.). An inline block comment may be treated as a note without punctuation.

### Imports

* Stated at the top of the file
* Ordered alphabetically
* Split in two sections
    * First party: crate imports + Aleo imports (example: snarkVM)
    * Third party: rust std + everything else

Example:
```rust

use crate::Circuit;
use leo_ast::IntegerType;

use serde::Serialize;
use std::{
    fmt,
    sync::{Arc, Weak},
};
```

`rust fmt` should automatically sort imports alphabetically after they are split into the appropriate sections.

## Coding conventions

Leo is a big project, so (non-)adherence to best practices related to performance can have a considerable impact; below are the rules we try to follow at all times in order to ensure high quality of the code:

### Memory handling
- If the final size is known, pre-allocate the collections (`Vec`, `HashMap` etc.) using `with_capacity` or `reserve` - this ensures that there are both fewer allocations (which involve system calls) and that the final allocated capacity is as close to the required size as possible.
- Create the collections right before they are populated/used, as opposed to e.g. creating a few big ones at the beginning of a function and only using them later on; this reduces the amount of time they occupy memory.
- If an intermediate vector is avoidable, use an `Iterator` instead; most of the time this just amounts to omitting the call to `.collect()` if a single-pass iteraton follows afterwards, or returning an `impl Iterator<Item = T>` from a function when the caller only needs to iterate over that result once.
- When possible, fill/resize collections "in bulk" instead of pushing a single element in a loop; this is usually (but not always) detected by `clippy`, suggesting to create vectors containing a repeated value with `vec![x; N]` or extending them with `.resize(N, x)`.
- When a value is to eventually be consumed in a chain of function calls, pass it by value instead of by reference; this has the following benefits:
  * It makes the fact that the value is needed by value clear to the caller, who can then potentially reclaim it from the object afterwards if it is "heavy", limiting allocations.
  * It often enables the value to be cloned fewer times (whenever it's no longer needed at the callsite).
  * When the value is consumed and is not needed afterwards, the memory it occupies is freed, improving memory utilization.
- If a slice may or may _not_ be extended (which requires a promotion to a vector) and does not need to be consumed afterwards, consider using a [`Cow<'a, [T]>`](https://doc.rust-lang.org/std/borrow/enum.Cow.html) combined with `Cow::to_mut` instead to potentially avoid an extra allocation; an example in Leo could be conditional padding of bits.
- Prefer arrays and temporary slices to vectors where possible; arrays are often a good choice if their final size is known in advance and isn't too great (as they are stack-bound), and a small temporary slice `&[x, y, z]` is preferable to a `vec![x, y, z]` if it's applicable.
- If a reference is sufficient, don't use `.clone()`/`to_vec()`, which is often the case with methods on `struct`s that provide access to their contents; if they only need to be referenced, there's no need for the extra allocation.
- Use `into_iter()` instead of `iter().cloned()` where possible, i.e. whenever the values being iterated over can be consumed altogether.
- If possible, reuse collections; an example would be a loop that needs a clean vector on each iteration: instead of creating and allocating it over and over, create it _before_ the loop and use `.clear()` on every iteration instead.
- Try to keep the sizes of `enum` variants uniform; use `Box<T>` on ones that are large.

### Misc. performance

- Avoid the `format!()` macro; if it is used only to convert a single value to a `String`, use `.to_string()` instead, which is also available to all the implementors of `Display`.
- Don't check if an element belongs to a map (using `contains` or `get`) if you want to conditionally insert it too, as the return value of `insert` already indicates whether the value was present or not; use that or the `Entry` API instead.
- If a reference is sufficient as a function parameter, use:
  * `&[T]` instead of `&Vec<T>`
  * `&str` instead of `&String`
  * `&Path` instead of `&PathBuf`
- For `struct`s that can be compared/discerned based on some specific field(s), consider hand-written implementations of `PartialEq` **and** `Hash` ([they must match](https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq)) for faster comparison and hashing.
