---
id: standard_operators
title: Standard Operators
sidebar_label: Standard Operators
toc_min_heading_level: 2
toc_max_heading_level: 3
---

[general tags]: # "operators, standard_operators, assert, hash, commit, random, address, block"

## Table of Contents

| Name                                                        | Description                                               |
| ----------------------------------------------------------- | :-------------------------------------------------------- |
| [abs](#abs)                                                 | Absolute value                                            |
| [abs_wrapped](#abs_wrapped)                                 | Wrapping absolute value                                   |
| [add](#add)                                                 | Addition                                                  |
| [add_wrapped](#add_wrapped)                                 | Wrapping addition                                         |
| [and](#and)                                                 | Conjunction                                               |
| [assert](#assert)                                           | Assert boolean true                                       |
| [assert_eq](#assert_eq)                                     | Assert equality                                           |
| [assert_neq](#assert_neq)                                   | Assert non-equality                                       |
| [block.height](#blockheight)                                | Fetch the latest block height                             |
| [block.timestamp](#blocktimestamp)                          | Fetch the latest block timestamp                          |
| [Deserialize:from_bits::[TYPE] ](#deserializefrom_bitstype) | Deserialize bits to a data type                           |
| [div](#div)                                                 | Division                                                  |
| [div_wrapped](#div_wrapped)                                 | Wrapping division operation                               |
| [double](#double)                                           | Double                                                    |
| [group::GEN](#groupgen)                                     | group generator                                           |
| [Aleo::generator](#aleogenerator)                                   | Aleo group generator constant                             |
| [Aleo::generator_powers](#aleogenerator_powers)                     | Precomputed Aleo generator powers `[group; 251]`          |
| [gt](#gt)                                                   | Greater than comparison                                   |
| [gte](#gte)                                                 | Greater than or equal to comparison                       |
| [inv](#inv)                                                 | Multiplicative inverse                                    |
| [eq](#eq)                                                   | Equality comparison                                       |
| [neq](#neq)                                                 | Non-equality comparison                                   |
| [lt](#lt)                                                   | Less than comparison                                      |
| [lte](#lte)                                                 | Less than or equal to comparison                          |
| [mod](#mod)                                                 | Modulo                                                    |
| [mul](#mul)                                                 | Multiplication                                            |
| [mul_wrapped](#mul_wrapped)                                 | Wrapping multiplication                                   |
| [nand](#nand)                                               | Negated conjunction                                       |
| [neg](#neg)                                                 | Additive inverse                                          |
| [nor](#nor)                                                 | Negated disjunction                                       |
| [not](#not)                                                 | Logical negation                                          |
| [or](#or)                                                   | (Inclusive) disjunction                                   |
| [pow](#pow)                                                 | Exponentiation                                            |
| [pow_wrapped](#pow_wrapped)                                 | Wrapping exponentiation                                   |
| [rem](#rem)                                                 | Remainder                                                 |
| [rem_wrapped](#rem_wrapped)                                 | Wrapping remainder                                        |
| [self.address](#selfaddress)                                | Address of the current program                            |
| [self.caller](#selfcaller)                                  | Address of the calling user/program                       |
| [self.checksum](#selfcaller)                                | Checksum of a program                                     |
| [self.edition](#selfedition)                                | Version number of a program                               |
| [self.program_owner](#selfprogram_owner)                    | Address that submitted a program's deployment transaction |
| [self.signer](#selfsigner)                                  | Address of the top-level calling user                     |
| [Serialize::to_bits](#serializeto_bits)                     | Serialize data to bits                                    |
| [shl](#shl)                                                 | Shift left                                                |
| [shl_wrapped](#shl_wrapped)                                 | Wrapping shift left                                       |
| [shr](#shr)                                                 | Shift right                                               |
| [shr_wrapped](#shr_wrapped)                                 | Wrapping shift right                                      |
| [square_root](#square_root)                                 | Square root                                               |
| [square](#square)                                           | Square                                                    |
| [sub](#sub)                                                 | Subtraction                                               |
| [sub_wrapped](#sub_wrapped)                                 | Wrapping subtraction                                      |
| [ternary](#ternary)                                         | Ternary select                                            |
| [to_x_coordinate](#to_x_coordinate)                         | Extract x-coordinate of a group element                   |
| [to_y_coordinate](#to_y_coordinate)                         | Extract y-coordinate of a group element                   |
| [xor](#xor)                                                 | Exclusive conjunction                                     |

## Arithmetic Operators

### `abs`

```leo
let a: i8 = -1i8;
let b: i8 = a.abs(); // 1i8
```

Computes the absolute value of the input, checking for overflow, storing the result in the destination.

Note that execution will halt if the operation overflows. For cases where wrapping semantics are needed, see the [abs_wrapped](#abs_wrapped) instruction. This overflow happens when the input is the minimum value of a signed integer type. For example, `abs -128i8` would result in overflow, since `128` cannot be represented as an `i8`.

#### Supported Types

| Input  | Destination |
| ------ | :---------- |
| `i8`   | `i8`        |
| `i16`  | `i16`       |
| `i32`  | `i32`       |
| `i64`  | `i64`       |
| `i128` | `i128`      |

[Back to Top](#table-of-contents)

---

### `abs_wrapped`

```leo
let a: i8 = -128i8;
let b: i8 = a.abs_wrapped(); // -128i8
```

Compute the absolute value of the input, wrapping around at the boundary of the type, and storing the result in the destination.

#### Supported Types

| Input  | Destination |
| ------ | :---------- |
| `i8`   | `i8`        |
| `i16`  | `i16`       |
| `i32`  | `i32`       |
| `i64`  | `i64`       |
| `i128` | `i128`      |

[Back to Top](#table-of-contents)

---

### `add`

```leo
let a: u8 = 1u8;
let b: u8 = a + 1u8; // 2u8
let c: u8 = b.add(1u8); // 3u8
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

```leo
let a: u8 = 255u8;
let b: u8 = a.add_wrapped(1u8); // 0u8
```

Adds `first` with `second`, wrapping around at the boundary of the type, and storing the result in `destination`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | :---------- |
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

```leo
let a: u8 = 4u8;
let b: u8 = a / 2u8; // 2u8
let c: u8 = b.div(2u8); // 1u8
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
| ------- | ------- | :---------- |
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

```leo
let a: i8 = -128i8;
let b: i8 = a.div_wrapped(-1i8); // -128i8
```

Divides `first` by `second`, wrapping around at the boundary of the type, and storing the result in `destination`. Halts if `second` is zero.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | :---------- |
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

```leo
let a: u8 = 3u8.mod(2u8); // 1u8
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

```leo
let a: u8 = 2u8 * 2u8; // 4u8
let b: u8 = a.mul(2u8); // 8u8
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

```leo
let a: u8 = 128u8.mul_wrapped(2u8); // 0u8
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

```leo
let a: i8 = -1i8.neg(); // 1i8
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

```leo
let a: u8 = 2u8 ** 2u8; // 4u8
let b: u8 = a.pow(2u8); // 16u8
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

```leo
let a: u8 = 16u8.pow_wrapped(2u8); // 0u8
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

```leo
let a: u8 = 3u8 % 2u8; // 1u8
let b: u8 = 4u8.rem(2u8); // 0u8
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

```leo
let a: i8 = -128i8;
let b: i8 = a.rem_wrapped(-1i8); // 0i8
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

```leo
let a: u8 = 2u8 - 1u8; // 1u8
let b: u8 = a.sub(1u8); // 0u8
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

```leo
let a: u8 = 0u8.sub_wrapped(1u8); // 255u8
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

```leo
// Integer (bitwise) AND
let a: i8 = 1i8 & 1i8;
let b: i8 = 1i8.and(2i8);

// Boolean (logical) AND
let a: bool = true && true;
let b: bool = true.and(false);
```

Performs an AND operation on integer (bitwise) or boolean `first` and `second`,
storing the result in `destination`.

#### Supported Types

| First  | Second | Destination |
| ------ | ------ | :---------- |
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

```leo
let a: bool = true.nand(false); // true
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

```leo
let a: bool = false.nor(false); // true
```

Calculates the negated (inclusive) disjunction of `first` and `second`, storing the result in `destination`. The result is `true` if and only if both `first` and `second` are `false`.

#### Supported Type

| First  | Second | Destination |
| ------ | ------ | ----------- |
| `bool` | `bool` | `bool`      |

[Back to Top](#table-of-contents)

---

### `not`

```leo
let a: bool = true.not(); // false
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

```leo
// Integer (bitwise) OR
let a: i8 = 1i8 | 2i8;
let b: i8 = 1i8.or(2i8);

// Boolean (logical) OR
let a: bool = true || true;
let b: bool = true.or(false);
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

```leo
let a: u8 = 1u8 << 1u8; // 2u8
let b: u8 = a.shl(1u8); // 4u8
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

```leo
let a: u8 = 128u8.shl_wrapped(1u8); // 0u8
let b: i8 = 64i8.shl_wrapped(2u8); // -128i8
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

```leo
let a: u8 = 4u8 >> 1u8; // 2u8
let b: u8 = a.shr(1u8); // 1u8
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

```leo
let a: u8 = 128u8.shr_wrapped(7u8); // 1u8
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

```leo
let a: bool = true.xor(false); // true
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

```leo
let a: bool = 2u8 > 1u8; // true
let b: bool = 1u8.gt(1u8); // false
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

```leo
let a: bool = 2u8 >= 1u8; // true
let b: bool = 1u8.gte(1u8); // true
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

```leo
let a: bool = 1u8 == 1u8; // true
let b: bool = 1u8.eq(2u8); // false
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

```leo
let a: bool = 1u8 != 1u8; // false
let b: bool = 1u8.neq(2u8); // true
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

```leo
let a: bool = 1u8 < 2u8; // true
let b: bool = 1u8.lt(1u8); // false
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

```leo
let a: bool = 1u8 <= 2u8; // true
let b: bool = 1u8.lte(1u8); // true
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

### `block.height`

```leo
program example.aleo {
    fn matches(height: u32) -> Final {
        return final {
            assert_eq(height, block.height);
        };
    }
}
```

The `block.height` operator is used to fetch the latest block height in a Leo program. It represents the number of
blocks in the chain. In the above example, `block.height` is used in a `final { }` block to fetch the latest block
height in a program.

:::info

- The `block.height` operator can only be used inside a `final { }` block or inside a `final fn`. Using it outside will result in a compilation error.
- The `block.height` operator doesn't take any parameters.
  :::

[Back to Top](#table-of-contents)

---

### `block.timestamp`

```leo
program example.aleo {
    fn matches(timestamp: i64) -> Final {
        return final {
            assert_eq(timestamp, block.timestamp);
        };
    }
}
```

The `block.timestamp` operator is used to fetch the UNIX timestamp of the latest block in a Leo program. In the above example, `block.timestamp` is used in a `final { }` block to fetch the latest block timestamp in a program.

:::info

- The `block.timestamp` operator can only be used inside a `final { }` block or inside a `final fn`. Using it outside will result in a compilation error.
- The `block.timestamp` operator doesn't take any parameters.
  :::

[Back to Top](#table-of-contents)

---

### `self.address`

```leo
program example.aleo {
    fn get_program_address() -> address {
        return self.address;
    }
}
```

The `self.address` operator returns the address of the program that calls it. While programs are identified by their name (`{PROGRAM_NAME}.aleo`), under the hood they have a corresponding Aleo address.

:::info

- The `self.address` operator doesn't take any parameters.
  :::

[Back to Top](#table-of-contents)

---

### `self.caller`

```leo
program example.aleo {
    fn matches(addr: address) -> bool {
        return self.caller == addr;
    }
}
```

The `self.caller` operator returns the address of the account/program that invoked the current entry function. Note that if the function was called as part of an external program, this operation will return the address of the program, NOT the address of the top-level user.

:::info

- The `self.caller` operator doesn't take any parameters.
  :::

[Back to Top](#table-of-contents)

---

### `self.checksum`

```leo
program example.aleo {
    fn matches(checksum: [u8,32]) -> Final {
        return final {
            assert_eq(self.checksum, checksum);
        };
    }
}
```

The `self.checksum` operator returns a program's checksum, which is a unique identifier for the program's code.

You may also refer to another program's checksum with the following syntax:

```leo
import credits.aleo;
...
let ext_checksum: [u8, 32] = Program::checksum(credits.aleo);
```

:::info

- The `self.checksum` operator can only be used inside a `final { }` block or inside a `final fn`. Using it outside will result in a compilation error.
- The `self.checksum` operator doesn't take any parameters.
- To reference another program's checksum, you will need to import that program first.
  :::

[Back to Top](#table-of-contents)

---

### `self.edition`

```leo
program example.aleo {
    fn matches(edition: u16) -> Final {
        return final {
            assert_eq(self.edition, edition);
        };
    }
}
```

The `self.edition` operator returns a program's edition, which is the program's version number. A program's edition starts at zero and is incremented by one for each upgrade. The edition is tracked automatically on the network.

You may also refer to another program's edition with the following syntax:

```leo
import credits.aleo;
...
let ext_edition: u16 = Program::edition(credits.aleo);
```

:::info

- The `self.edition` operator can only be used inside a `final { }` block or inside a `final fn`. Using it outside will result in a compilation error.
- The `self.edition` operator doesn't take any parameters.
- To reference another program's edition, you will need to import that program first.
  :::

[Back to Top](#table-of-contents)

---

### `self.program_owner`

```leo
program example.aleo {
    fn matches(owner: address) -> Final {
        return final {
            assert_eq(self.program_owner, owner);
        };
    }
}
```

The `self.program_owner` operator returns the address that submitted the deployment transaction for a program.
You may also refer to another program's owner with the following syntax:

```leo
import credits.aleo;
...
let ext_owner: u16 = Program::owner(credits.aleo);
```

:::info

- The `self.program_owner` operator can only be used inside a `final { }` block or inside a `final fn`. Using it outside will result in a compilation error.
- The `self.program_owner` operator doesn't take any parameters.
- To reference another program's owner, you will need to import that program first.
  :::

[Back to Top](#table-of-contents)

---

### `self.signer`

```leo
program example.aleo {
    fn matches(addr: address) -> bool {
        return self.signer == addr;
    }
}
```

The `self.signer` operator returns the address of the account that invoked the top-level entry function. This will be the user account that signed the transaction.

:::info

- The `self.signer` operator doesn't take any parameters.
  :::

[Back to Top](#table-of-contents)

---

## Group/Field Specific Operators

### `group::GEN`

```leo
let g: group = group::GEN; // the group generator
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

```leo
let g: group = Aleo::generator;
```

Returns the generator point of the Aleo group. This is equivalent to `group::GEN` but expressed through the `Aleo` namespace for consistency with other `Aleo::*` intrinsics.

---

### `Aleo::generator_powers`

```leo
let powers: [group; 251] = Aleo::generator_powers;
```

Returns a precomputed array of the first 251 consecutive powers of the Aleo group generator: `[G^0, G^1, G^2, ..., G^250]`. Useful for efficient scalar multiplication in final blocks without recomputing the powers at runtime.

Only available inside `final { }` blocks.

---

### `double`

```leo
let a: group = 2group;
let b: group = a.double();
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

```leo
let a: field = 1field.inv();
```

Computes the multiplicative inverse of the input, storing the result in `destination`.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `field` | `field`     |

[Back to Top](#table-of-contents)

---

### `square`

```leo
let a: field = 1field.square(); // 1field
```

Squares the input, storing the result in `destination`.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `field` | `field`     |

[Back to Top](#table-of-contents)

---

### `square_root`

```leo
let a: field = 1field.square_root(); // 1field
```

Computes the square root of the input, storing the result in `destination`. If the input is a quadratic residue, the function returns the `smaller` of the two possible roots based on modular ordering. If the input is not a quadratic residue, execution halts.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `field` | `field`     |

[Back to Top](#table-of-contents)

---

### `to_x_coordinate`

```leo
let x: field = 0group.to_x_coordinate(); // 0field
```

Extracts the x-coordinate of the group element as a field element.

#### Supported Types

| Input   | Destination |
| ------- | ----------- |
| `group` | `field`     |

[Back to Top](#table-of-contents)

---

### `to_y_coordinate`

```leo
let y: field = 0group.to_y_coordinate(); // 1field
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

```leo
// Standard serialization (includes type metadata)
let bits: [bool; 58] = Serialize::to_bits(value);

// Raw serialization (no metadata, just raw bits)
let bits: [bool; 32] = Serialize::to_bits_raw(value);

// Works with arrays too
let bits: [bool; 128] = Serialize::to_bits_raw([1u32, 2u32, 3u32, 4u32]);
```

By appending `_raw` to the end of the function, the function will omit the metadata of a type and directly serialize the input bits.

#### Supported Types

| First     | Destination   | Destination (Raw) |
| --------- | :------------ | ----------------- |
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

```leo
// Standard deserialization (includes type metadata)
let bits1: [bool; 58] = Serialize::to_bits(1u32);
let value1: u32 = Deserialize::from_bits::[u32](bits1);

// Raw deserialization (no metadata, just raw bits)
let bits2: [bool; 32] = Serialize::to_bits_raw(1u32);
let value2: u32 = Deserialize::from_bits_raw::[u32](bits2);

// Works with arrays too
let bits3: [bool; 128] = Serialize::to_bits_raw([1u32, 2u32, 3u32, 4u32]);
let arr: [u32; 4] = Deserialize::from_bits_raw::[[u32; 4]](bits3);
```

By appending `_raw` to the end of the function, the function will omit the metadata of a type and directly serialize the input bits.

#### Supported Types

| TYPE      | Input         | Input (Raw)   | Destination |
| --------- | :------------ | ------------- | ----------- |
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

```leo
let a: bool = true;
let b: bool = false;

assert(a); // will not halt
assert(b); // program halts
```

Checks whether the expression evaluates to a `true` boolean value, halting if evaluates to `false`.

#### Supported Types

| Expression |
| ---------- |
| `bool`     |

[Back to Top](#table-of-contents)

---

### `assert_eq`

```leo
let a: u8 = 1u8;
let b: u8 = 2u8;

assert_eq(a, a); // will not halt
assert_eq(a, b); // program halts
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

```leo
let a: u8 = 1u8;
let b: u8 = 2u8;

assert_neq(a, b); // will not halt
assert_neq(a, a); // program halts
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

```leo
let a: u8 = true ? 1u8 : 2u8; // 1u8
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
