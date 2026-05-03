---
id: operators
title: Operators & Expressions
sidebar_label: Operators and Expressions
---

[general tags]: # "operators, cryptographic_operators, assert, hash, commit, random, address, block"

## Operators

Operators in Leo compute a value based off of one or more expressions. Leo defaults to checked arithmetic, which means that it will throw an error if an overflow or division by zero is detected.

For instance, addition adds `first` with `second`, storing the outcome in `destination`. For integer types, a constraint is added to check for overflow.
For cases where wrapping semantics are needed for integer types, see the wrapped variants of the operators.

```leo file=../code_snippets/operators_basics/src/main.leo#arithmetic
```

:::note
The Leo operators compile down to [`Aleo Instructions`](https://developer.aleo.org/guides/aleo/opcodes) opcodes executable by the Aleo Virtual Machine (AVM).
:::

### Operator Precedence

Operators will prioritize evaluation according to:

|                                   Operator                                    | Associativity |
| :---------------------------------------------------------------------------: | :-----------: |
|                                `!` `-`(unary)                                 |               |
|                                     `**`                                      | right to left |
|                                    `*` `/`                                    | left to right |
|                                `+` `-`(binary)                                | left to right |
|                                   `<<` `>>`                                   | left to right |
|                                      `&`                                      | left to right |
|                              <code>&#124;</code>                              | left to right |
|                                      `^`                                      | left to right |
|                               `<` `>` `<=` `>=`                               |               |
|                                   `==` `!=`                                   | left to right |
|                                     `&&`                                      | left to right |
|                           <code>&#124;&#124;</code>                           | left to right |
| `=` `+=` `-=` `*=` `/=` `%=` `**=` `<<=` `>>=` `&=` <code>&#124;=</code> `^=` |               |

### Parentheses

To prioritize a different evaluation, use parentheses `()` around the expression.

```leo file=../code_snippets/operators_basics/src/main.leo#parentheses
```

`(a + 1u8)` will be evaluated before multiplying by two `* 2u8`.
