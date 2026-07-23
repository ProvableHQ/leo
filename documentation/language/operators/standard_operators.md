---
id: standard_operators
title: Standard Operators
sidebar_label: Standard Operators
toc_min_heading_level: 2
toc_max_heading_level: 3
---

[general tags]: # "operators, standard_operators, assert, hash, commit, random, address, block"

## Table of Contents

| Name                                                       | Description                                               |
| ---------------------------------------------------------- | --------------------------------------------------------- |
| [abs](#abs)                                                | Absolute value                                            |
| [abs_wrapped](#abs_wrapped)                                | Wrapping absolute value                                   |
| [add](#add)                                                | Addition                                                  |
| [add_wrapped](#add_wrapped)                                | Wrapping addition                                         |
| [and](#and)                                                | Conjunction                                               |
| [assert](#assert)                                          | Assert boolean true                                       |
| [assert_eq](#assert_eq)                                    | Assert equality                                           |
| [assert_neq](#assert_neq)                                  | Assert non-equality                                       |
| [Deserialize:from_bits::[TYPE]](#deserializefrom_bitstype) | Deserialize bits to a data type                           |
| [div](#div)                                                | Division                                                  |
| [div_wrapped](#div_wrapped)                                | Wrapping division operation                               |
| [double](#double)                                          | Double                                                    |
| [group::GEN](#groupgen)                                    | group generator                                           |
| [Aleo::generator](#aleogenerator)                          | Aleo group generator constant                             |
| [Aleo::generator_powers](#aleogenerator_powers)            | Precomputed Aleo generator powers `[group; 251]`          |
| [gt](#gt)                                                  | Greater than comparison                                   |
| [gte](#gte)                                                | Greater than or equal to comparison                       |
| [inv](#inv)                                                | Multiplicative inverse                                    |
| [eq](#eq)                                                  | Equality comparison                                       |
| [neq](#neq)                                                | Non-equality comparison                                   |
| [lt](#lt)                                                  | Less than comparison                                      |
| [lte](#lte)                                                | Less than or equal to comparison                          |
| [mod](#mod)                                                | Modulo                                                    |
| [mul](#mul)                                                | Multiplication                                            |
| [mul_wrapped](#mul_wrapped)                                | Wrapping multiplication                                   |
| [nand](#nand)                                              | Negated conjunction                                       |
| [neg](#neg)                                                | Additive inverse                                          |
| [nor](#nor)                                                | Negated disjunction                                       |
| [not](#not)                                                | Logical negation                                          |
| [or](#or)                                                  | (Inclusive) disjunction                                   |
| [pow](#pow)                                                | Exponentiation                                            |
| [pow_wrapped](#pow_wrapped)                                | Wrapping exponentiation                                   |
| [rem](#rem)                                                | Remainder                                                 |
| [rem_wrapped](#rem_wrapped)                                | Wrapping remainder                                        |
| [Serialize::to_bits](#serializeto_bits)                    | Serialize data to bits                                    |
| [shl](#shl)                                                | Shift left                                                |
| [shl_wrapped](#shl_wrapped)                                | Wrapping shift left                                       |
| [shr](#shr)                                                | Shift right                                               |
| [shr_wrapped](#shr_wrapped)                                | Wrapping shift right                                      |
| [square_root](#square_root)                                | Square root                                               |
| [square](#square)                                          | Square                                                    |
| [sub](#sub)                                                | Subtraction                                               |
| [sub_wrapped](#sub_wrapped)                                | Wrapping subtraction                                      |
| [ternary](#ternary)                                        | Ternary select                                            |
| [to_x_coordinate](#to_x_coordinate)                        | Extract x-coordinate of a group element                   |
| [to_y_coordinate](#to_y_coordinate)                        | Extract y-coordinate of a group element                   |
| [xor](#xor)                                                | Exclusive conjunction                                     |

## Arithmetic Operators

### `abs`

```leo file=../../code_snippets/operators/standard/src/main.leo#abs
```

Computes the absolute value of the input, checking for overflow, storing the result in the destination.

Note that execution will halt if the operation overflows. For cases where wrapping semantics are needed, see the [abs_wrapped](#abs_wrapped) instruction. This overflow happens when the input is the minimum value of a signed integer type. For example, `abs -128i8` would result in overflow, since `128` cannot be represented as an `i8`.

#### Supported Types

| Input  | Destination |
| ------ | ----------- |
| `i8`   | `i8`        |
| `i16`  | `i16`       |
| `i32`  | `i32`       |
| `i64`  | `i64`       |
| `i128` | `i128`      |

[Back to Top](#table-of-contents)

---

### `abs_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#abs_wrapped
```

Compute the absolute value of the input, wrapping around at the boundary of the type, and storing the result in the destination.

#### Supported Types

| Input  | Destination |
| ------ | ----------- |
| `i8`   | `i8`        |
| `i16`  | `i16`       |
| `i32`  | `i32`       |
| `i64`  | `i64`       |
| `i128` | `i128`      |

[Back to Top](#table-of-contents)

---

### `add`

```leo file=../../code_snippets/operators/standard/src/main.leo#add
```

Adds `first` with `second`, storing the result in `destination`.

Note that execution will halt if the operation overflows. For cases where wrapping semantics are needed for integer types, see the [add_wrapped](#add_wrapped) instruction.

#### Supported Types

| First    | Second   | Destination |
| -------- | -------- | ----------- |
| `field`  | `field`  | `field`     |
| `group`  | `group`  | `group`     |
| `i8`     | `i8`     | `i8`        |
| `i16`    | `i16`    | `i16`       |
| `i32`    | `i32`    | `i32`       |
| `i64`    | `i64`    | `i64`       |
| `i128`   | `i128`   | `i128`      |
| `u8`     | `u8`     | `u8`        |
| `u16`    | `u16`    | `u16`       |
| `u32`    | `u32`    | `u32`       |
| `u64`    | `u64`    | `u64`       |
| `u128`   | `u128`   | `u128`      |
| `scalar` | `scalar` | `scalar`    |

[Back to Top](#table-of-contents)

---

### `add_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#add_wrapped
```

Adds `first` with `second`, wrapping around at the boundary of the type, and storing the result in `destination`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `div`

```leo file=../../code_snippets/operators/standard/src/main.leo#div
```

Performs division of the first operand by the second, storing the result in the destination. The operation halts if division by zero is attempted.

For integer types, this operation performs truncated division. Truncated division rounds towards zero, regardless of the sign of the operands. This means it cuts off any digits after the decimal, leaving the whole number whose absolute value is less than or equal to the result.

For example:

- `7 / 3` yields `2`, not `2.3333`.
- `-7 / 3` yields `-2`, not `-2.3333`.

The operation halts if there is an underflow. Underflow occurs when dividing the minimum value of a signed integer type by -1. For example, `-128i8 / -1i8` would result in underflow, since 128 cannot be represented as an `i8`.

For field types, division `a / b` is well-defined for any field values `a` and `b` except when `b = 0field`.

For cases where wrapping semantics are needed for integer types, see the [div_wrapped](#div_wrapped) instruction.

#### Supported Types

| First   | Second  | Destination |
| ------- | ------- | ----------- |
| `field` | `field` | `field`     |
| `i8`    | `i8`    | `i8`        |
| `i16`   | `i16`   | `i16`       |
| `i32`   | `i32`   | `i32`       |
| `i64`   | `i64`   | `i64`       |
| `i128`  | `i128`  | `i128`      |
| `u8`    | `u8`    | `u8`        |
| `u16`   | `u16`   | `u16`       |
| `u32`   | `u32`   | `u32`       |
| `u64`   | `u64`   | `u64`       |
| `u128`  | `u128`  | `u128`      |

[Back to Top](#table-of-contents)

---

### `div_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#div_wrapped
```

Divides `first` by `second`, wrapping around at the boundary of the type, and storing the result in `destination`. Halts if `second` is zero.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `mod`

```leo file=../../code_snippets/operators/standard/src/main.leo#mod_op
```

Takes the modulo of `first` with respect to `second`, storing the result in `destination`. Halts if `second` is zero.

The semantics of this operation are consistent with the mathematical definition of modulo operation.

`mod` ensures the remainder has the same sign as the `second` operand. This differs from [`rem`](#rem), which follows truncated division and takes the sign of the `first` operand.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `mul`

```leo file=../../code_snippets/operators/standard/src/main.leo#mul
```

Multiplies `first` with `second`, storing the result in `destination`.

Note that execution will halt if the operation overflows/underflows. For cases where wrapping semantics are needed for integer types, see the [mul_wrapped](#mul_wrapped) instruction.

#### Supported Types

| First    | Second   | Destination |
| -------- | -------- | ----------- |
| `field`  | `field`  | `field`     |
| `group`  | `scalar` | `group`     |
| `scalar` | `group`  | `group`     |
| `i8`     | `i8`     | `i8`        |
| `i16`    | `i16`    | `i16`       |
| `i32`    | `i32`    | `i32`       |
| `i64`    | `i64`    | `i64`       |
| `i128`   | `i128`   | `i128`      |
| `u8`     | `u8`     | `u8`        |
| `u16`    | `u16`    | `u16`       |
| `u32`    | `u32`    | `u32`       |
| `u64`    | `u64`    | `u64`       |
| `u128`   | `u128`   | `u128`      |

[Back to Top](#table-of-contents)

---

### `mul_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#mul_wrapped
```

Multiplies `first` with `second`, wrapping around at the boundary of the type, and storing the result in `destination`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `neg`

```leo file=../../code_snippets/operators/standard/src/main.leo#neg
```

Negates the first operand, storing the result in the destination.

For signed integer types, the operation halts if the minimum value is negated. For example, `-128i8.neg()` halts since `128` cannot be represented as an `i8`.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `field` | `field`     |
| `group` | `group`     |
| `i8`    | `i8`        |
| `i16`   | `i16`       |
| `i32`   | `i32`       |
| `i64`   | `i64`       |
| `i128`  | `i128`      |

[Back to Top](#table-of-contents)

---

### `pow`

```leo file=../../code_snippets/operators/standard/src/main.leo#pow
```

Raises `first` to the power of `second`, storing the result in `destination`.

Note that execution will halt if the operation overflows/underflows. For cases where wrapping semantics are needed for integer types, see the [pow_wrapped](#pow_wrapped) instruction.

#### Supported Types

`Magnitude` can be a `u8`, `u16`, or `u32`.

| First   | Second      | Destination |
| ------- | ----------- | ----------- |
| `field` | `field`     | `field`     |
| `i8`    | `Magnitude` | `i8`        |
| `i16`   | `Magnitude` | `i16`       |
| `i32`   | `Magnitude` | `i32`       |
| `i64`   | `Magnitude` | `i64`       |
| `i128`  | `Magnitude` | `i128`      |
| `u8`    | `Magnitude` | `u8`        |
| `u16`   | `Magnitude` | `u16`       |
| `u32`   | `Magnitude` | `u32`       |
| `u64`   | `Magnitude` | `u64`       |
| `u128`  | `Magnitude` | `u128`      |

[Back to Top](#table-of-contents)

---

### `pow_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#pow_wrapped
```

Raises `first` to the power of `second`, wrapping around at the boundary of the type, storing the result in `destination`.

#### Supported Types

`Magnitude` can be a `u8`, `u16`, or `u32`.

| First  | Second      | Destination |
| ------ | ----------- | ----------- |
| `i8`   | `Magnitude` | `i8`        |
| `i16`  | `Magnitude` | `i16`       |
| `i32`  | `Magnitude` | `i32`       |
| `i64`  | `Magnitude` | `i64`       |
| `i128` | `Magnitude` | `i128`      |
| `u8`   | `Magnitude` | `u8`        |
| `u16`  | `Magnitude` | `u16`       |
| `u32`  | `Magnitude` | `u32`       |
| `u64`  | `Magnitude` | `u64`       |
| `u128` | `Magnitude` | `u128`      |

[Back to Top](#table-of-contents)

---

### `rem`

```leo file=../../code_snippets/operators/standard/src/main.leo#rem
```

Computes the remainder of the division of the `first` operand by the `second`, storing the result in `destination` following truncated division rules:

a and b refers to first and second respectively

`a % b = a - (a / b) * b`

Here, `a` and `b` refer to the `first` and `second` operands, respectively

Note that execution will halt if the operation underflows or divides by zero. This underflow happens when the associated division operation, [div](#div), underflows.

For cases where wrapping semantics are needed for integer types, see the [rem_wrapped](#rem_wrapped) instruction.

`rem` follows truncated division, meaning the remainder has the same sign as `a`. This differs from [mod](#mod), where the remainder matches the sign of `b`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `rem_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#rem_wrapped
```

Computes the remainder of the division of the `first` operand by the `second` following truncated division rules, storing the result in `destination`. Halts on division by zero.
Unlike [`rem`](#rem), `rem_wrapped` is always defined and does not halt, even when [`div`](#div) would wrap around.

Notably, `rem_wrapped` does not introduce wrapping itself but ensures the operation remains defined where `rem` would be undefined.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `sub`

```leo file=../../code_snippets/operators/standard/src/main.leo#sub
```

Computes `first - second`, storing the result in `destination`. The operation halts if the result is negative in an unsigned type or if it exceeds the minimum representable value in a signed type.

#### Supported Types

| First   | Second  | Destination |
| ------- | ------- | ----------- |
| `field` | `field` | `field`     |
| `group` | `group` | `group`     |
| `i8`    | `i8`    | `i8`        |
| `i16`   | `i16`   | `i16`       |
| `i32`   | `i32`   | `i32`       |
| `i64`   | `i64`   | `i64`       |
| `i128`  | `i128`  | `i128`      |
| `u8`    | `u8`    | `u8`        |
| `u16`   | `u16`   | `u16`       |
| `u32`   | `u32`   | `u32`       |
| `u64`   | `u64`   | `u64`       |
| `u128`  | `u128`  | `u128`      |

[Back to Top](#table-of-contents)

---

### `sub_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#sub_wrapped
```

Computes `first - second`, wrapping around at the boundary of the type, and storing the result in `destination`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

## Boolean/Bitwise Operators

### `and`

```leo file=../../code_snippets/operators/standard/src/main.leo#and_op
```

Performs an AND operation on integer (bitwise) or boolean `first` and `second`,
storing the result in `destination`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `bool` | `bool` | `bool`      |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `nand`

```leo file=../../code_snippets/operators/standard/src/main.leo#nand_op
```

Calculates the negated conjunction of `first` and `second`, storing the result in `destination`.
The result is false if and only if both first and second are true.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `bool` | `bool` | `bool`      |

[Back to Top](#table-of-contents)

---

### `nor`

```leo file=../../code_snippets/operators/standard/src/main.leo#nor_op
```

Calculates the negated (inclusive) disjunction of `first` and `second`, storing the result in `destination`. The result is `true` if and only if both `first` and `second` are `false`.

#### Supported Type

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `bool` | `bool` | `bool`      |

[Back to Top](#table-of-contents)

---

### `not`

```leo file=../../code_snippets/operators/standard/src/main.leo#not_op
```

Perform a NOT operation on an integer (bitwise) or boolean input, storing the result in `destination`.

#### Supported Types

| Input  | Destination |
| ------ | ----------- |
| `bool` | `bool`      |
| `i8`   | `i8`        |
| `i16`  | `i16`       |
| `i32`  | `i32`       |
| `i64`  | `i64`       |
| `i128` | `i128`      |
| `u8`   | `u8`        |
| `u16`  | `u16`       |
| `u32`  | `u32`       |
| `u64`  | `u64`       |
| `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `or`

```leo file=../../code_snippets/operators/standard/src/main.leo#or_op
```

Performs an inclusive OR operation on integer (bitwise) or boolean `first` and `second`, storing the result in `destination`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `bool` | `bool` | `bool`      |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

### `shl`

```leo file=../../code_snippets/operators/standard/src/main.leo#shl
```

Shifts `first` left by `second` bits, storing the result in `destination`. The operation halts if the shift distance exceeds the bit size of `first`, or if the shifted result does not fit within the type of `first`.

#### Supported Types

`Magnitude` can be a `u8`, `u16`, or `u32`.

| First  | Second      | Destination |
| ------ | ----------- | ----------- |
| `i8`   | `Magnitude` | `i8`        |
| `i16`  | `Magnitude` | `i16`       |
| `i32`  | `Magnitude` | `i32`       |
| `i64`  | `Magnitude` | `i64`       |
| `i128` | `Magnitude` | `i128`      |
| `u8`   | `Magnitude` | `u8`        |
| `u16`  | `Magnitude` | `u16`       |
| `u32`  | `Magnitude` | `u32`       |
| `u64`  | `Magnitude` | `u64`       |
| `u128` | `Magnitude` | `u128`      |

[Back to Top](#table-of-contents)

---

### `shl_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#shl_wrapped
```

Shifts `first` left by `second` bits, wrapping around at the boundary of the type, storing the result in `destination`. The shift distance is masked to the bit width of `first`, ensuring that shifting by n is equivalent to shifting by `n % bit_size`.

If bits are shifted beyond the type's range, they are discarded, which may cause sign changes for signed integers.

#### Supported Types

`Magnitude` can be a `u8`, `u16`, or `u32`.

| First  | Second      | Destination |
| ------ | ----------- | ----------- |
| `i8`   | `Magnitude` | `i8`        |
| `i16`  | `Magnitude` | `i16`       |
| `i32`  | `Magnitude` | `i32`       |
| `i64`  | `Magnitude` | `i64`       |
| `i128` | `Magnitude` | `i128`      |
| `u8`   | `Magnitude` | `u8`        |
| `u16`  | `Magnitude` | `u16`       |
| `u32`  | `Magnitude` | `u32`       |
| `u64`  | `Magnitude` | `u64`       |
| `u128` | `Magnitude` | `u128`      |

[Back to Top](#table-of-contents)

---

### `shr`

```leo file=../../code_snippets/operators/standard/src/main.leo#shr
```

Shifts `first` right by `second` bits, storing the result in `destination`. The operation halts if the shift distance exceeds the bit size of `first`.

#### Supported Types

`Magnitude` can be a `u8`, `u16`, or `u32`.

| First  | Second      | Destination |
| ------ | ----------- | ----------- |
| `i8`   | `Magnitude` | `i8`        |
| `i16`  | `Magnitude` | `i16`       |
| `i32`  | `Magnitude` | `i32`       |
| `i64`  | `Magnitude` | `i64`       |
| `i128` | `Magnitude` | `i128`      |
| `u8`   | `Magnitude` | `u8`        |
| `u16`  | `Magnitude` | `u16`       |
| `u32`  | `Magnitude` | `u32`       |
| `u64`  | `Magnitude` | `u64`       |
| `u128` | `Magnitude` | `u128`      |

[Back to Top](#table-of-contents)

---

### `shr_wrapped`

```leo file=../../code_snippets/operators/standard/src/main.leo#shr_wrapped
```

Shifts `first` right by `second` bits, wrapping around at the boundary of the type, storing the result in `destination`. The shift distance is masked to the bit width of `first`, ensuring that shifting by `n` is equivalent to shifting by `n % bit_size`.

#### Supported Types

`Magnitude` can be a `u8`, `u16`, or `u32`.

| First  | Second      | Destination |
| ------ | ----------- | ----------- |
| `i8`   | `Magnitude` | `i8`        |
| `i16`  | `Magnitude` | `i16`       |
| `i32`  | `Magnitude` | `i32`       |
| `i64`  | `Magnitude` | `i64`       |
| `i128` | `Magnitude` | `i128`      |
| `u8`   | `Magnitude` | `u8`        |
| `u16`  | `Magnitude` | `u16`       |
| `u32`  | `Magnitude` | `u32`       |
| `u64`  | `Magnitude` | `u64`       |
| `u128` | `Magnitude` | `u128`      |

[Back to Top](#table-of-contents)

---

### `xor`

```leo file=../../code_snippets/operators/standard/src/main.leo#xor_op
```

Performs a XOR operation on integer (bitwise) or boolean `first` and `second`, storing the result in `destination`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `bool` | `bool` | `bool`      |
| `i8`   | `i8`   | `i8`        |
| `i16`  | `i16`  | `i16`       |
| `i32`  | `i32`  | `i32`       |
| `i64`  | `i64`  | `i64`       |
| `i128` | `i128` | `i128`      |
| `u8`   | `u8`   | `u8`        |
| `u16`  | `u16`  | `u16`       |
| `u32`  | `u32`  | `u32`       |
| `u64`  | `u64`  | `u64`       |
| `u128` | `u128` | `u128`      |

[Back to Top](#table-of-contents)

---

## Comparators

### `gt`

```leo file=../../code_snippets/operators/standard/src/main.leo#gt
```

Checks if `first` is greater than `second`, storing the result in `destination`.

#### Supported Types

| First    | Second   | Destination |
| -------- | -------- | ----------- |
| `field`  | `field`  | `bool`      |
| `i8`     | `i8`     | `bool`      |
| `i16`    | `i16`    | `bool`      |
| `i32`    | `i32`    | `bool`      |
| `i64`    | `i64`    | `bool`      |
| `i128`   | `i128`   | `bool`      |
| `u8`     | `u8`     | `bool`      |
| `u16`    | `u16`    | `bool`      |
| `u32`    | `u32`    | `bool`      |
| `u64`    | `u64`    | `bool`      |
| `u128`   | `u128`   | `bool`      |
| `scalar` | `scalar` | `bool`      |

[Back to Top](#table-of-contents)

---

### `gte`

```leo file=../../code_snippets/operators/standard/src/main.leo#gte
```

Checks if `first` is greater than or equal to `second`, storing the result in `destination`.

#### Supported Types

| First    | Second   | Destination |
| -------- | -------- | ----------- |
| `field`  | `field`  | `bool`      |
| `i8`     | `i8`     | `bool`      |
| `i16`    | `i16`    | `bool`      |
| `i32`    | `i32`    | `bool`      |
| `i64`    | `i64`    | `bool`      |
| `i128`   | `i128`   | `bool`      |
| `u8`     | `u8`     | `bool`      |
| `u16`    | `u16`    | `bool`      |
| `u32`    | `u32`    | `bool`      |
| `u64`    | `u64`    | `bool`      |
| `u128`   | `u128`   | `bool`      |
| `scalar` | `scalar` | `bool`      |

[Back to Top](#table-of-contents)

---

### `eq`

```leo file=../../code_snippets/operators/standard/src/main.leo#eq
```

Compares `first` and `second` for equality, storing the result in `destination`.

#### Supported Types

| First       | Second      | Destination |
| ----------- | ----------- | ----------- |
| `address`   | `address`   | `bool`      |
| `bool`      | `bool`      | `bool`      |
| `field`     | `field`     | `bool`      |
| `group`     | `group`     | `bool`      |
| `i8`        | `i8`        | `bool`      |
| `i16`       | `i16`       | `bool`      |
| `i32`       | `i32`       | `bool`      |
| `i64`       | `i64`       | `bool`      |
| `i128`      | `i128`      | `bool`      |
| `u8`        | `u8`        | `bool`      |
| `u16`       | `u16`       | `bool`      |
| `u32`       | `u32`       | `bool`      |
| `u64`       | `u64`       | `bool`      |
| `u128`      | `u128`      | `bool`      |
| `scalar`    | `scalar`    | `bool`      |
| `Signature` | `Signature` | `bool`      |
| `struct`    | `struct`    | `bool`      |
| `Record`    | `Record`    | `bool`      |

[Back to Top](#table-of-contents)

---

### `neq`

```leo file=../../code_snippets/operators/standard/src/main.leo#neq
```

Compares `first` and `second` for non-equality, storing the result in `destination`.

#### Supported Types

| First       | Second      | Destination |
| ----------- | ----------- | ----------- |
| `address`   | `address`   | `bool`      |
| `bool`      | `bool`      | `bool`      |
| `field`     | `field`     | `bool`      |
| `group`     | `group`     | `bool`      |
| `i8`        | `i8`        | `bool`      |
| `i16`       | `i16`       | `bool`      |
| `i32`       | `i32`       | `bool`      |
| `i64`       | `i64`       | `bool`      |
| `i128`      | `i128`      | `bool`      |
| `u8`        | `u8`        | `bool`      |
| `u16`       | `u16`       | `bool`      |
| `u32`       | `u32`       | `bool`      |
| `u64`       | `u64`       | `bool`      |
| `u128`      | `u128`      | `bool`      |
| `scalar`    | `scalar`    | `bool`      |
| `Signature` | `Signature` | `bool`      |
| `struct`    | `struct`    | `bool`      |
| `Record`    | `Record`    | `bool`      |

[Back to Top](#table-of-contents)

---

### `lt`

```leo file=../../code_snippets/operators/standard/src/main.leo#lt
```

Checks if `first` is less than `second`, storing the result in `destination`.

#### Supported Types

| First    | Second   | Destination |
| -------- | -------- | ----------- |
| `field`  | `field`  | `bool`      |
| `i8`     | `i8`     | `bool`      |
| `i16`    | `i16`    | `bool`      |
| `i32`    | `i32`    | `bool`      |
| `i64`    | `i64`    | `bool`      |
| `i128`   | `i128`   | `bool`      |
| `u8`     | `u8`     | `bool`      |
| `u16`    | `u16`    | `bool`      |
| `u32`    | `u32`    | `bool`      |
| `u64`    | `u64`    | `bool`      |
| `u128`   | `u128`   | `bool`      |
| `scalar` | `scalar` | `bool`      |

[Back to Top](#table-of-contents)

---

### `lte`

```leo file=../../code_snippets/operators/standard/src/main.leo#lte
```

Checks if `first` is less than or equal to `second`, storing the result in `destination`.

#### Supported Types

| First    | Second   | Destination |
| -------- | -------- | ----------- |
| `field`  | `field`  | `bool`      |
| `i8`     | `i8`     | `bool`      |
| `i16`    | `i16`    | `bool`      |
| `i32`    | `i32`    | `bool`      |
| `i64`    | `i64`    | `bool`      |
| `i128`   | `i128`   | `bool`      |
| `u8`     | `u8`     | `bool`      |
| `u16`    | `u16`    | `bool`      |
| `u32`    | `u32`    | `bool`      |
| `u64`    | `u64`    | `bool`      |
| `u128`   | `u128`   | `bool`      |
| `scalar` | `scalar` | `bool`      |

[Back to Top](#table-of-contents)

---

## Context-dependent Expressions

Execution-context accessors — the immediate caller, the transaction signer, the program's own address, deployment metadata, and the current block / network — live in the [`std::ctx`](../standard_library.md#stdctx) module of the standard library. On-chain metadata for **other** (imported) programs (their checksum, edition, owner, and per-function checksum) lives in [`std::prog`](../standard_library.md#stdprog).

## Group/Field Specific Operators

### `group::GEN`

```leo file=../../code_snippets/operators/standard/src/main.leo#group_gen
```

Returns the generator of the algebraic group that the `group` type consists of.

The compilation of Leo is based on an elliptic curve, whose points form a group,
and on a specified point on that curve, which generates a subgroup, whose elements form the type `group`.

This is a constant, not a function. Thus, it takes no inputs, and just returns an output.

It is an associated constant, whose name is `GEN` and whose associated type is `group`.

#### Supported Types

| Destination |
| ----------- |
| `group`     |

[Back to Top](#table-of-contents)

---

### `Aleo::generator`

```leo file=../../code_snippets/operators/standard/src/main.leo#aleo_generator
```

Returns the generator point of the Aleo group. This is equivalent to [`group::GEN`](#groupgen) but expressed through the `Aleo` namespace for consistency with the other `Aleo::*` operators. Like `group::GEN`, it is a constant that takes no inputs.

#### Supported Types

| Destination |
| ----------- |
| `group`     |

[Back to Top](#table-of-contents)

---

### `Aleo::generator_powers`

```leo file=../../code_snippets/operators/standard/src/main.leo#aleo_generator_powers
```

Returns a precomputed array of the first 251 consecutive powers of the Aleo group generator: `[G^0, G^1, G^2, ..., G^250]`. Useful for efficient scalar multiplication without recomputing the powers at runtime.

The array always has exactly 251 elements. Indexing beyond `250u32` is rejected at compile time by Leo's array bounds check — there is no runtime halt specific to this operator.

#### Supported Types

| Destination    |
| -------------- |
| `[group; 251]` |

[Back to Top](#table-of-contents)

---

### `double`

```leo file=../../code_snippets/operators/standard/src/main.leo#double
```

Adds the input to itself, storing the result in `destination`.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `field` | `field`     |
| `group` | `group`     |

[Back to Top](#table-of-contents)

---

### `inv`

```leo file=../../code_snippets/operators/standard/src/main.leo#inv
```

Computes the multiplicative inverse of the input, storing the result in `destination`.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `field` | `field`     |

[Back to Top](#table-of-contents)

---

### `square`

```leo file=../../code_snippets/operators/standard/src/main.leo#square
```

Squares the input, storing the result in `destination`.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `field` | `field`     |

[Back to Top](#table-of-contents)

---

### `square_root`

```leo file=../../code_snippets/operators/standard/src/main.leo#square_root
```

Computes the square root of the input, storing the result in `destination`. If the input is a quadratic residue, the function returns the `smaller` of the two possible roots based on modular ordering. If the input is not a quadratic residue, execution halts.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `field` | `field`     |

[Back to Top](#table-of-contents)

---

### `to_x_coordinate`

```leo file=../../code_snippets/operators/standard/src/main.leo#to_x_coordinate
```

Extracts the x-coordinate of the group element as a field element.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `group` | `field`     |

[Back to Top](#table-of-contents)

---

### `to_y_coordinate`

```leo file=../../code_snippets/operators/standard/src/main.leo#to_y_coordinate
```

Extracts the y-coordinate of the group element as a field element.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `group` | `field`     |

[Back to Top](#table-of-contents)

---

## Serialization / Deserialization

### `Serialize::to_bits`

```leo file=../../code_snippets/operators/standard/src/main.leo#to_bits
```

By appending `_raw` to the end of the function, the function will omit the metadata of a type and directly serialize the input bits.

#### Supported Types

| First     | Destination   | Destination (Raw) |
| --------- | ------------- | ----------------- |
| `address` | `[bool; 279]` | `[bool; 253]`     |
| `bool`    | `[bool; 27]`  | `[bool; 1]`       |
| `field`   | `[bool; 279]` | `[bool; 253]`     |
| `group`   | `[bool; 279]` | `[bool; 253]`     |
| `i8`      | `[bool; 34]`  | `[bool; 8]`       |
| `i16`     | `[bool; 42]`  | `[bool; 16]`      |
| `i32`     | `[bool; 58]`  | `[bool; 32]`      |
| `i64`     | `[bool; 90]`  | `[bool; 64]`      |
| `i128`    | `[bool; 154]` | `[bool; 128]`     |
| `u8`      | `[bool; 34]`  | `[bool; 8]`       |
| `u16`     | `[bool; 42]`  | `[bool; 16]`      |
| `u32`     | `[bool; 58]`  | `[bool; 32]`      |
| `u64`     | `[bool; 90]`  | `[bool; 64]`      |
| `u128`    | `[bool; 154]` | `[bool; 128]`     |
| `scalar`  | `[bool; 277]` | `[bool; 251]`     |

[Back to Top](#table-of-contents)

---

### `Deserialize::from_bits::[TYPE]`

```leo file=../../code_snippets/operators/standard/src/main.leo#from_bits
```

By appending `_raw` to the end of the function, the function will omit the metadata of a type and directly serialize the input bits.

#### Supported Types

| TYPE      | Input         | Input (Raw)   | Destination |
| --------- | ------------- | ------------- | ----------- |
| `address` | `[bool; 279]` | `[bool; 253]` | `address`   |
| `bool`    | `[bool; 27]`  | `[bool; 1]`   | `bool`      |
| `field`   | `[bool; 279]` | `[bool; 253]` | `field`     |
| `group`   | `[bool; 279]` | `[bool; 253]` | `group`     |
| `i8`      | `[bool; 34]`  | `[bool; 8]`   | `i8`        |
| `i16`     | `[bool; 42]`  | `[bool; 16]`  | `i16`       |
| `i32`     | `[bool; 58]`  | `[bool; 32]`  | `i32`       |
| `i64`     | `[bool; 90]`  | `[bool; 64]`  | `i64`       |
| `i128`    | `[bool; 154]` | `[bool; 128]` | `i128`      |
| `u8`      | `[bool; 34]`  | `[bool; 8]`   | `u8`        |
| `u16`     | `[bool; 42]`  | `[bool; 16]`  | `u16`       |
| `u32`     | `[bool; 58]`  | `[bool; 32]`  | `u32`       |
| `u64`     | `[bool; 90]`  | `[bool; 64]`  | `u64`       |
| `u128`    | `[bool; 154]` | `[bool; 128]` | `u128`      |
| `scalar`  | `[bool; 277]` | `[bool; 251]` | `scalar`    |

[Back to Top](#table-of-contents)

---

## Miscellaneous

### `assert`

```leo file=../../code_snippets/operators/standard/src/main.leo#assert_op
```

Checks whether the expression evaluates to a `true` boolean value, halting if evaluates to `false`.

#### Supported Types

| Expression |
| ---------- |
| `bool`     |

[Back to Top](#table-of-contents)

---

### `assert_eq`

```leo file=../../code_snippets/operators/standard/src/main.leo#assert_eq
```

Checks whether `first` and `second` are equal, halting if they are not equal.

#### Supported Types

| First       | Second      |
| ----------- | ----------- |
| `address`   | `address`   |
| `bool`      | `bool`      |
| `field`     | `field`     |
| `group`     | `group`     |
| `i8`        | `i8`        |
| `i16`       | `i16`       |
| `i32`       | `i32`       |
| `i64`       | `i64`       |
| `i128`      | `i128`      |
| `u8`        | `u8`        |
| `u16`       | `u16`       |
| `u32`       | `u32`       |
| `u64`       | `u64`       |
| `u128`      | `u128`      |
| `scalar`    | `scalar`    |
| `Signature` | `Signature` |
| `struct`    | `struct`    |
| `Record`    | `Record`    |

[Back to Top](#table-of-contents)

---

### `assert_neq`

```leo file=../../code_snippets/operators/standard/src/main.leo#assert_neq
```

Checks whether `first` and `second` are not equal, halting if they are equal.

#### Supported Types

| First       | Second      |
| ----------- | ----------- |
| `address`   | `address`   |
| `bool`      | `bool`      |
| `field`     | `field`     |
| `group`     | `group`     |
| `i8`        | `i8`        |
| `i16`       | `i16`       |
| `i32`       | `i32`       |
| `i64`       | `i64`       |
| `i128`      | `i128`      |
| `u8`        | `u8`        |
| `u16`       | `u16`       |
| `u32`       | `u32`       |
| `u64`       | `u64`       |
| `u128`      | `u128`      |
| `scalar`    | `scalar`    |
| `Signature` | `Signature` |
| `struct`    | `struct`    |
| `Record`    | `Record`    |

[Back to Top](#table-of-contents)

---

### `ternary`

```leo file=../../code_snippets/operators/standard/src/main.leo#ternary
```

Selects `first`, if `condition` is true, otherwise selects `second`, storing the result in `destination`.

#### Supported Types

| Condition | First       | Second      | Destination |
| --------- | ----------- | ----------- | ----------- |
| `bool`    | `bool`      | `bool`      | `bool`      |
| `bool`    | `field`     | `field`     | `field`     |
| `bool`    | `group`     | `group`     | `group`     |
| `bool`    | `i8`        | `i8`        | `i8`        |
| `bool`    | `i16`       | `i16`       | `i16`       |
| `bool`    | `i32`       | `i32`       | `i32`       |
| `bool`    | `i64`       | `i64`       | `i64`       |
| `bool`    | `i128`      | `i128`      | `i128`      |
| `bool`    | `u8`        | `u8`        | `u8`        |
| `bool`    | `u16`       | `u16`       | `u16`       |
| `bool`    | `u32`       | `u32`       | `u32`       |
| `bool`    | `u64`       | `u64`       | `u64`       |
| `bool`    | `u128`      | `u128`      | `u128`      |
| `bool`    | `scalar`    | `scalar`    | `scalar`    |
| `bool`    | `Signature` | `Signature` | `Signature` |

[Back to Top](#table-of-contents)

---
