---
id: control_flow
title: Control Flow
sidebar_label: Control Flow
---

[general tags]: # "loop, conditional, return"

## Conditional Statements

Conditional statements are declared as `if {condition} {...} else if {condition} {...} else {...}`.
Conditional statements can be nested.

```leo file=../../code_snippets/control_flow/src/main.leo#if_else
```

Leo also supports ternary expressions. Ternary expressions are declared as `{condition} ? {then} : {else}`, and can be nested.

```leo file=../../code_snippets/control_flow/src/main.leo#ternary
```

## For Loops

For loops are declared as `for {variable: type} in {lower bound}..{upper bound}`.
The loop bounds must be integer constants of the same type. Furthermore, if
the lower bound is superior or equal to the upper bound, the loop will result in no operations.
Nested loops are supported.

```leo file=../../code_snippets/control_flow/src/main.leo#for_loop
```

## Return Statements

Return statements are declared as `return {expression};`.

```leo file=../../code_snippets/control_flow/src/main.leo#return_stmt
```
