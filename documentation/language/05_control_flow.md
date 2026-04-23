---
id: control
title: Control Flow
sidebar_label: Control Flow
---

[general tags]: # "loop, conditional, return"

## Conditional Statements

Conditional statements are declared as `if {condition} { ... } else if {condition} { ... } else { ... }`.
Conditional statements can be nested.

```leo
    let a: u8 = 1u8;

    if a == 1u8 {
        a += 1u8;
    } else if a == 2u8 {
        a += 2u8;
    } else {
        a += 3u8;
    }
```

Leo also supports ternary expressions. Ternary expressions are declared as `{condition} ? {then} : {else}`, and can be nested.

```leo
let a: u8 = 1u8;
a = (a == 1u8) ? a + 1u8 : ((a == 2u8) ? a + 2u8 : a + 3u8);
```

## Return Statements

Return statements are declared as `return {expression};`.

```leo
    let a: u8 = 1u8;

    if a == 1u8 {
        return a + 1u8;
    } else if a == 2u8 {
        return a + 2u8;
    } else {
        return a + 3u8;
    }
```

## For Loops

For loops are declared as `for {variable: type} in {lower bound}..{upper bound}`.
The loop bounds must be integer constants of the same type. Furthermore, if
the lower bound is superior or equal to the upper bound, the loop will result in no operations.
Nested loops are supported.

```leo
  let count: u32 = 0u32;

  for i: u32 in 0u32..5u32 {
    count += 1u32;
  }

  return count; // returns 5u32
```
