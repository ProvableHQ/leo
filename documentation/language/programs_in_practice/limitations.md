---
id: limitations
title: Limitations
sidebar_label: Limitations
toc_min_heading_level: 3
toc_max_heading_level: 3
---

snarkVM imposes the following limits on Aleo programs:

- the maximum size of the compiled program is **2048 KB** by character count. This limit is raised by consensus version: 100 KB (V1), 512 KB (V14), and 2048 KB (V16).
- the maximum number of mappings is **31**.
  Each `storage` singleton uses one mapping slot, and each `storage` vector uses two.
  Storage declarations share this limit with explicit `mapping` declarations.
- the maximum number of imports is **64**.
- the maximum length of a program name is **30 characters** (excluding the `.aleo` suffix). Identifiers (including struct, record, mapping, and function names) are limited to **31 ASCII alphanumeric or underscore characters** by the Aleo identifier rules.
- the maximum import depth is **64**.
- the maximum call depth is **31**.
- the maximum number of entry functions is **31** per program. A `constructor` counts separately. Helper `fn`s are inlined at their call sites and do not consume slots in this budget. (`final { }` blocks compile into a `finalize` section attached to their parent entry function — they do not produce additional functions.)
- the maximum number of structs is **310** (`10 × MAX_FUNCTIONS`).
- the maximum number of records is **310**.
- the maximum number of closures is **62** (`2 × MAX_FUNCTIONS`).
- the maximum number of inputs and outputs **per entry point** is **16** each.

**If your _compiled_ Leo program exceeds these limits, then consider modularizing or rearchitecting your program.** The only way these limits can be increased is through a formal protocol upgrade via the governance process defined by the Aleo Network Foundation.

Some other protocol-level limits to be aware of are:

- **the maximum transaction size is 2304 KB**, also raised by consensus version: 128 KB (V1), 768 KB (V14), and 2304 KB (V16). If your program exceeds this — for example by requiring large inputs or producing large outputs — consider optimizing the data types in your Leo code.
- **the maximum number of micro-credits your transaction can consume for on-chain execution is `100_000_000`.** If your program exceeds this, consider optimizing on-chain components of your Leo code.

Only the governance process can increase these limits.
The authoritative values, such as `MAX_PROGRAM_SIZE`, `MAX_MAPPINGS`, and `MAX_FUNCTIONS`, are in `snarkvm-console-network`.

## Compiling Conditional On-Chain Code

Consider the following Leo entry function.

```leo file=../../code_snippets/limitations/src/main.leo#weird_sub showLineNumbers
```

This is compiled into the following Aleo instructions:

```aleo file=../../code_snippets/limitations/build/limitations_demo/limitations_demo.aleo showLineNumbers
```

Observe that both branches of the conditional are executed in the entry function. The correct output is then selected using a ternary instruction. This compilation method is only possible because operations in transitions are purely functional. [^1].

Some on-chain commands are not purely functional.
For example, `get`, `get.or_use`, `contains`, `remove`, and `set` depend on the program state.
Thus, Leo cannot use the off-chain technique.
It compiles on-chain code with `branch` and `position` commands, which can skip instruction sequences.
Skipped instructions do not initialize their destination registers.
A later instruction cannot access these registers, and an access attempt causes an execution error.

An assignment from a conditional statement to its parent scope can cause this access.
Therefore, Leo does not permit these assignments.

This restriction can be mitigated by future improvements to `snarkVM`, however we table that discussion for later.

[^1]: There are some operations that are not purely functional, e.g `add` which can fail on overflow.
