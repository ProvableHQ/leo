---
id: limitations
title: Limitations
sidebar_label: Limitations
toc_min_heading_level: 3
toc_max_heading_level: 3
---

snarkVM imposes the following limits on Aleo programs:

- the maximum size of the program is 100 KB, by the number of characters.
- the maximum number of mappings is 31.
- the maximum number of imports is 64.
- the maximum length of a program name is 30 characters (excluding the `.aleo` suffix).
- the maximum import depth is 64.
- the maximum call depth is 31.
- the maximum number of functions is 31.
- the maximum number of structs is 310.
- the maximum number of records is 310.
- the maximum number of closures is 62.

**If your _compiled_ Leo program exceeds these limits, then consider modularizing or rearchitecting your program.** The only way these limits can be increased is through a formal protocol upgrade via the governance process defined by the Aleo Network Foundation.

Some other protocol-level limits to be aware of are:

- **the maximum transaction size is 128 KB.** If your program exceeds this, perhaps by requiring large inputs or producing large outputs, consider optimizing the data types in your Leo code.
- **the maximum number of micro-credits your transaction can consume for on-chain execution is `100_000_000`.**. If your program exceeds this, consider optimizing on-chain components of your Leo code.

As with the above restrictions, these limits can only be increased via the governance process.

## Compiling Conditional On-Chain Code

Consider the following Leo entry function.

```leo file=../../code_snippets/limitations/src/main.leo#weird_sub showLineNumbers
```

This is compiled into the following Aleo instructions:

```aleo file=../../code_snippets/limitations/build/main.aleo showLineNumbers
```

Observe that both branches of the conditional are executed in the entry function. The correct output is then selected using a ternary instruction. This compilation method is only possible because operations in transitions are purely functional. [^1].

On-chain commands are not all purely functional; for example, `get`, `get.or_use`, `contains`, `remove`, and `set`, whose semantics all depend on the state of the program. As a result, the same technique for off-chain code cannot be used. Instead, the on-chain code is compiled using `branch` and `position` commands, which allow the program to define sequences of code that are skipped. However, because destination registers in skipped instructions are not initialized, they cannot be accessed in a following instructions. In other words, depending on the branch taken, some registers are invalid and an attempt to access them will return in an execution error. The only Leo code pattern that produces such an access attempt is code that attempts to assign out to a parent scope from a conditional statement; consequently, they are disallowed.

This restriction can be mitigated by future improvements to `snarkVM`, however we table that discussion for later.

[^1]: There are some operations that are not purely functional, e.g `add` which can fail on overflow.
