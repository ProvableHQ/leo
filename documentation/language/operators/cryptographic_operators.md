---
id: cryptographic_operators
title: Cryptographic Operators
sidebar_label: Cryptographic Operators
toc_min_heading_level: 2
toc_max_heading_level: 3
---

[general tags]: # "operators, cryptographic_operators, assert, hash, commit, random, address, block"

## Hashing vs. Committing

Many of the cryptographic operators have both `hash` and `commit` variants.

The `hash` variant is a one-way function that takes an input and produces a fixed-size output called a "hash" or "digest." It has a unique property that if even one bit of the input changes, the output hash will change completely, making it easy to see if data has been tampered with.

The `commit` variant is a wrapper around the `hash` variant that takes an additional parameter called a blinding factor, otherwise known as a **salt**. The **salt** is appended to the input value before hashing it, ensuring the output will be unique from just the simple hash of the raw input. So long as a different salt is used each time, this allows a user to commit to the same value multiple times without revealing that they've done so.

## Table of Contents

| Name                                                      | Description                                  |
| --------------------------------------------------------- | :------------------------------------------- |
| [BHP256::hash_to_TYPE](#bhp256hash_to_type)               | 256-bit input BHP hash                       |
| [BHP256::commit_to_TYPE](#bhp256commit_to_type)           | 256-bit input BHP commitment                 |
| [BHP512::hash_to_TYPE](#bhp512hash_to_type)               | 512-bit input BHP hash                       |
| [BHP512::commit_to_TYPE](#bhp512commit_to_type)           | 512-bit input BHP commitment                 |
| [BHP768::hash_to_TYPE](#bhp768hash_to_type)               | 768-bit input BHP hash                       |
| [BHP768::commit_to_TYPE](#bhp768commit_to_type)           | 768-bit input BHP commitment                 |
| [BHP1024::hash_to_TYPE](#bhp1024hash_to_type)             | 1024-bit input BHP hash                      |
| [BHP1024::commit_to_TYPE](#bhp1024commit_to_type)         | 1024-bit input BHP commitment                |
| [Pedersen64::hash_to_TYPE](#pedersen64hash_to_type)       | 64-bit input Pedersen hash                   |
| [Pedersen64::commit_to_TYPE](#pedersen64commit_to_type)   | 64-bit input Pedersen commitment             |
| [Pedersen128::hash_to_TYPE](#pedersen128hash_to_type)     | 128-bit input Pedersen hash                  |
| [Pedersen128::commit_to_TYPE](#pedersen128commit_to_type) | 128-bit input Pedersen commitment            |
| [Poseidon2::hash_to_TYPE](#poseidon2hash_to_type)         | Poseidon hash with input rate 2              |
| [Poseidon4::hash_to_TYPE](#poseidon4hash_to_type)         | Poseidon hash with input rate 4              |
| [Poseidon8::hash_to_TYPE](#poseidon8hash_to_type)         | Poseidon hash with input rate 8              |
| [Keccak256::hash_to_bits](#keccak256hash_to_bits)         | 256-bit input/output Keccak hash             |
| [Keccak256::hash_to_TYPE](#keccak256hash_to_type)         | 256-bit input Keccak hash                    |
| [Keccak384::hash_to_bits](#keccak384hash_to_bits)         | 384-bit input/output Keccak hash             |
| [Keccak384::hash_to_TYPE](#keccak384hash_to_type)         | 384-bit input Keccak hash                    |
| [Keccak512::hash_to_bits](#keccak512hash_to_bits)         | 512-bit input/output Keccak hash             |
| [Keccak512::hash_to_TYPE](#keccak512hash_to_type)         | 512-bit input Keccak hash                    |
| [SHA3_256::hash_to_bits](#sha3_256hash_to_bits)           | 256-bit input/output SHA3 hash               |
| [SHA3_256::hash_to_TYPE](#sha3_256hash_to_type)           | 256-bit input SHA3 hash                      |
| [SHA3_384::hash_to_bits](#sha3_384hash_to_bits)           | 384-bit input/output SHA3 hash               |
| [SHA3_384::hash_to_TYPE](#sha3_384hash_to_type)           | 384-bit input SHA3 hash                      |
| [SHA3_512::hash_to_bits](#sha3_512hash_to_bits)           | 512-bit input/output SHA3 hash               |
| [SHA3_512::hash_to_TYPE](#sha3_512hash_to_type)           | 512-bit input SHA3 hash                      |
| [ChaCha::rand_TYPE](#chacharand_type)                     | ChaCha RNG                                   |
| [signature::verify](#signatureverify)                     | Verify a Schnorr signature                   |
| [ECDSA::verify_digest](#ecdsaverify_digest)               | Verify an ECDSA signature against a pre-hash |
| [ECDSA::verify_keccak256](#ecdsaverify_keccak256)         | Verify an ECDSA signature using Keccak256    |
| [ECDSA::verify_keccak384](#ecdsaverify_keccak384)         | Verify an ECDSA signature using Keccak384    |
| [ECDSA::verify_keccak512](#ecdsaverify_keccak512)         | Verify an ECDSA signature using Keccak512    |
| [ECDSA::verify_sha3_256](#ecdsaverify_sha3_256)           | Verify an ECDSA signature using SHA3_256     |
| [ECDSA::verify_sha3_384](#ecdsaverify_sha3_384)           | Verify an ECDSA signature using SHA3_384     |
| [ECDSA::verify_sha3_512](#ecdsaverify_sha3_512)           | Verify an ECDSA signature using SHA3_512     |
| [Snark::verify](#snarkverify)                             | Verify a single Varuna ZK proof on-chain            |
| [Snark::verify_batch](#snarkverify_batch)                 | Batch-verify multiple Varuna ZK proofs on-chain     |

## Bowe-Hopwood-Pedersen (BHP)

### `BHP256::hash_to_TYPE`

```leo
let result: address = BHP256::hash_to_address(1u8);
let result: field = BHP256::hash_to_field(2i64);
let result: group = BHP256::hash_to_group(1field);
let result: scalar = BHP256::hash_to_scalar(1field);
let result: i8 = BHP256::hash_to_i8(1field);
let result: i16 = BHP256::hash_to_i16(1field);
let result: i32 = BHP256::hash_to_i32(1field);
let result: i64 = BHP256::hash_to_i64(1field);
let result: i128 = BHP256::hash_to_i128(1field);
let result: u8 = BHP256::hash_to_u8(1field);
let result: u16 = BHP256::hash_to_u16(1field);
let result: u32 = BHP256::hash_to_u32(1field);
let result: u64 = BHP256::hash_to_u64(1field);
let result: u128 = BHP256::hash_to_u128(1field);

// Raw hash variants
let result: address = BHP256::hash_to_address_raw(1u8);
let result: field = BHP256::hash_to_field_raw(2i64);
                        ...
```

Computes a Bowe-Hopwood-Pedersen hash on inputs of 256-bit chunks in `first`, storing the hash in `destination`. The produced hash will be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given input is smaller than 129 bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `BHP256::commit_to_TYPE`

```leo
let salt: scalar = ChaCha::rand_scalar();
let a: address = BHP256::commit_to_address(1u8, salt);
let b: field = BHP256::commit_to_field(2i64, salt);
let c: group = BHP256::commit_to_group(1field, salt);
```

Computes a Bowe-Hopwood-Pedersen commitment on inputs of 256-bit chunks in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment can be an `address`, `field` or, `group` value.

The instruction will halt if the given input is smaller than 129 bits.

#### Supported Types

| First     | Second   | Destination                 |
| --------- | -------- | :-------------------------- |
| `address` | `scalar` | `address`, `field`, `group` |
| `bool`    | `scalar` | `address`, `field`, `group` |
| `field`   | `scalar` | `address`, `field`, `group` |
| `group`   | `scalar` | `address`, `field`, `group` |
| `i8`      | `scalar` | `address`, `field`, `group` |
| `i16`     | `scalar` | `address`, `field`, `group` |
| `i32`     | `scalar` | `address`, `field`, `group` |
| `i64`     | `scalar` | `address`, `field`, `group` |
| `i128`    | `scalar` | `address`, `field`, `group` |
| `u8`      | `scalar` | `address`, `field`, `group` |
| `u16`     | `scalar` | `address`, `field`, `group` |
| `u32`     | `scalar` | `address`, `field`, `group` |
| `u64`     | `scalar` | `address`, `field`, `group` |
| `u128`    | `scalar` | `address`, `field`, `group` |
| `scalar`  | `scalar` | `address`, `field`, `group` |
| `struct`  | `scalar` | `address`, `field`, `group` |

[Back to Top](#table-of-contents)

---

### `BHP512::hash_to_TYPE`

```leo
let result: address = BHP512::hash_to_address(1u8);
let result: field = BHP512::hash_to_field(2i64);
let result: group = BHP512::hash_to_group(1field);
let result: scalar = BHP512::hash_to_scalar(1field);
let result: i8 = BHP512::hash_to_i8(1field);
let result: i16 = BHP512::hash_to_i16(1field);
let result: i32 = BHP512::hash_to_i32(1field);
let result: i64 = BHP512::hash_to_i64(1field);
let result: i128 = BHP512::hash_to_i128(1field);
let result: u8 = BHP512::hash_to_u8(1field);
let result: u16 = BHP512::hash_to_u16(1field);
let result: u32 = BHP512::hash_to_u32(1field);
let result: u64 = BHP512::hash_to_u64(1field);
let result: u128 = BHP512::hash_to_u128(1field);

// Raw hash variants
let result: address = BHP512::hash_to_address_raw(1u8);
let result: field = BHP512::hash_to_field_raw(2i64);
                        ...
```

Computes a Bowe-Hopwood-Pedersen hash on inputs of 512-bit chunks in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given input is smaller than 171 bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `BHP512::commit_to_TYPE`

```leo
let salt: scalar = ChaCha::rand_scalar();
let a: address = BHP512::commit_to_address(1u8, salt);
let b: field = BHP512::commit_to_field(2i64, salt);
let c: group = BHP512::commit_to_group(1field, salt);
```

Computes a Bowe-Hopwood-Pedersen commitment on inputs of 512-bit chunks in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given input is smaller than 171 bits.

#### Supported Types

| First     | Second   | Destination                 |
| --------- | -------- | :-------------------------- |
| `address` | `scalar` | `address`, `field`, `group` |
| `bool`    | `scalar` | `address`, `field`, `group` |
| `field`   | `scalar` | `address`, `field`, `group` |
| `group`   | `scalar` | `address`, `field`, `group` |
| `i8`      | `scalar` | `address`, `field`, `group` |
| `i16`     | `scalar` | `address`, `field`, `group` |
| `i32`     | `scalar` | `address`, `field`, `group` |
| `i64`     | `scalar` | `address`, `field`, `group` |
| `i128`    | `scalar` | `address`, `field`, `group` |
| `u8`      | `scalar` | `address`, `field`, `group` |
| `u16`     | `scalar` | `address`, `field`, `group` |
| `u32`     | `scalar` | `address`, `field`, `group` |
| `u64`     | `scalar` | `address`, `field`, `group` |
| `u128`    | `scalar` | `address`, `field`, `group` |
| `scalar`  | `scalar` | `address`, `field`, `group` |
| `struct`  | `scalar` | `address`, `field`, `group` |

[Back to Top](#table-of-contents)

---

### `BHP768::hash_to_TYPE`

```leo
let result: address = BHP768::hash_to_address(1u8);
let result: field = BHP768::hash_to_field(2i64);
let result: group = BHP768::hash_to_group(1field);
let result: scalar = BHP768::hash_to_scalar(1field);
let result: i8 = BHP768::hash_to_i8(1field);
let result: i16 = BHP768::hash_to_i16(1field);
let result: i32 = BHP768::hash_to_i32(1field);
let result: i64 = BHP768::hash_to_i64(1field);
let result: i128 = BHP768::hash_to_i128(1field);
let result: u8 = BHP768::hash_to_u8(1field);
let result: u16 = BHP768::hash_to_u16(1field);
let result: u32 = BHP768::hash_to_u32(1field);
let result: u64 = BHP768::hash_to_u64(1field);
let result: u128 = BHP768::hash_to_u128(1field);

// Raw hash variants
let result: address = BHP768::hash_to_address_raw(1u8);
let result: field = BHP768::hash_to_field_raw(2i64);
                        ...
```

Computes a Bowe-Hopwood-Pedersen hash on inputs of 768-bit chunks in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given input is smaller than 129 bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `BHP768::commit_to_TYPE`

```leo
let salt: scalar = ChaCha::rand_scalar();
let a: address = BHP768::commit_to_address(1u8, salt);
let b: field = BHP768::commit_to_field(2i64, salt);
let c: group = BHP768::commit_to_group(1field, salt);
```

Computes a Bowe-Hopwood-Pedersen commitment on inputs of 768-bit chunks in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given input is smaller than 129 bits.

#### Supported Types

| First     | Second   | Destination                 |
| --------- | -------- | :-------------------------- |
| `address` | `scalar` | `address`, `field`, `group` |
| `bool`    | `scalar` | `address`, `field`, `group` |
| `field`   | `scalar` | `address`, `field`, `group` |
| `group`   | `scalar` | `address`, `field`, `group` |
| `i8`      | `scalar` | `address`, `field`, `group` |
| `i16`     | `scalar` | `address`, `field`, `group` |
| `i32`     | `scalar` | `address`, `field`, `group` |
| `i64`     | `scalar` | `address`, `field`, `group` |
| `i128`    | `scalar` | `address`, `field`, `group` |
| `u8`      | `scalar` | `address`, `field`, `group` |
| `u16`     | `scalar` | `address`, `field`, `group` |
| `u32`     | `scalar` | `address`, `field`, `group` |
| `u64`     | `scalar` | `address`, `field`, `group` |
| `u128`    | `scalar` | `address`, `field`, `group` |
| `scalar`  | `scalar` | `address`, `field`, `group` |
| `struct`  | `scalar` | `address`, `field`, `group` |

[Back to Top](#table-of-contents)

---

### `BHP1024::hash_to_TYPE`

```leo
let result: address = BHP1024::hash_to_address(1u8);
let result: field = BHP1024::hash_to_field(2i64);
let result: group = BHP1024::hash_to_group(1field);
let result: scalar = BHP1024::hash_to_scalar(1field);
let result: i8 = BHP1024::hash_to_i8(1field);
let result: i16 = BHP1024::hash_to_i16(1field);
let result: i32 = BHP1024::hash_to_i32(1field);
let result: i64 = BHP1024::hash_to_i64(1field);
let result: i128 = BHP1024::hash_to_i128(1field);
let result: u8 = BHP1024::hash_to_u8(1field);
let result: u16 = BHP1024::hash_to_u16(1field);
let result: u32 = BHP1024::hash_to_u32(1field);
let result: u64 = BHP1024::hash_to_u64(1field);
let result: u128 = BHP1024::hash_to_u128(1field);

// Raw hash variants
let result: address = BHP1024::hash_to_address_raw(1u8);
let result: field = BHP1024::hash_to_field_raw(2i64);
                        ...
```

Computes a Bowe-Hopwood-Pedersen hash on inputs of 1024-bit chunks in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given input is smaller than 171 bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `BHP1024::commit_to_TYPE`

```leo
let salt: scalar = ChaCha::rand_scalar();
let a: address = BHP1024::commit_to_address(1u8, salt);
let b: field = BHP1024::commit_to_field(2i64, salt);
let c: group = BHP1024::commit_to_group(1field, salt);
```

Computes a Bowe-Hopwood-Pedersen commitment on inputs of 1024-bit chunks in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given input is smaller than 171 bits.

#### Supported Types

| First     | Second   | Destination                 |
| --------- | -------- | :-------------------------- |
| `address` | `scalar` | `address`, `field`, `group` |
| `bool`    | `scalar` | `address`, `field`, `group` |
| `field`   | `scalar` | `address`, `field`, `group` |
| `group`   | `scalar` | `address`, `field`, `group` |
| `i8`      | `scalar` | `address`, `field`, `group` |
| `i16`     | `scalar` | `address`, `field`, `group` |
| `i32`     | `scalar` | `address`, `field`, `group` |
| `i64`     | `scalar` | `address`, `field`, `group` |
| `i128`    | `scalar` | `address`, `field`, `group` |
| `u8`      | `scalar` | `address`, `field`, `group` |
| `u16`     | `scalar` | `address`, `field`, `group` |
| `u32`     | `scalar` | `address`, `field`, `group` |
| `u64`     | `scalar` | `address`, `field`, `group` |
| `u128`    | `scalar` | `address`, `field`, `group` |
| `scalar`  | `scalar` | `address`, `field`, `group` |
| `struct`  | `scalar` | `address`, `field`, `group` |

[Back to Top](#table-of-contents)

---

## Pedersen

### `Pedersen64::hash_to_TYPE`

```leo
let result: address = Pedersen64::hash_to_address(1u8);
let result: field = Pedersen64::hash_to_field(2i64);
let result: group = Pedersen64::hash_to_group(1field);
let result: scalar = Pedersen64::hash_to_scalar(1field);
let result: i8 = Pedersen64::hash_to_i8(1field);
let result: i16 = Pedersen64::hash_to_i16(1field);
let result: i32 = Pedersen64::hash_to_i32(1field);
let result: i64 = Pedersen64::hash_to_i64(1field);
let result: i128 = Pedersen64::hash_to_i128(1field);
let result: u8 = Pedersen64::hash_to_u8(1field);
let result: u16 = Pedersen64::hash_to_u16(1field);
let result: u32 = Pedersen64::hash_to_u32(1field);
let result: u64 = Pedersen64::hash_to_u64(1field);
let result: u128 = Pedersen64::hash_to_u128(1field);

// Raw hash variants
let result: address = Pedersen64::hash_to_address_raw(1u8);
let result: field = Pedersen64::hash_to_field_raw(2i64);
                        ...
```

Computes a Pedersen hash up to a 64-bit input in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given `struct` value exceeds the 64-bit limit.

#### Supported Types

| First    | Destination                                                                                               |
| -------- | :-------------------------------------------------------------------------------------------------------- |
| `bool`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `Pedersen64::commit_to_TYPE`

```leo
let salt: scalar = ChaCha::rand_scalar();
let a: address = Pedersen64::commit_to_address(1u8, salt);
let b: field = Pedersen64::commit_to_field(2i64, salt);
let c: group = Pedersen64::commit_to_group(1field, salt);
```

Computes a Pedersen commitment up to a 64-bit input in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given `struct` value exceeds the 64-bit limit.

#### Supported Types

| First    | Second   | Destination                 |
| -------- | -------- | :-------------------------- |
| `bool`   | `scalar` | `address`, `field`, `group` |
| `i8`     | `scalar` | `address`, `field`, `group` |
| `i16`    | `scalar` | `address`, `field`, `group` |
| `i32`    | `scalar` | `address`, `field`, `group` |
| `u8`     | `scalar` | `address`, `field`, `group` |
| `u16`    | `scalar` | `address`, `field`, `group` |
| `u32`    | `scalar` | `address`, `field`, `group` |
| `struct` | `scalar` | `address`, `field`, `group` |

[Back to Top](#table-of-contents)

---

### `Pedersen128::hash_to_TYPE`

```leo
let result: address = Pedersen128::hash_to_address(1u8);
let result: field = Pedersen128::hash_to_field(2i64);
let result: group = Pedersen128::hash_to_group(1field);
let result: scalar = Pedersen128::hash_to_scalar(1field);
let result: i8 = Pedersen128::hash_to_i8(1field);
let result: i16 = Pedersen128::hash_to_i16(1field);
let result: i32 = Pedersen128::hash_to_i32(1field);
let result: i64 = Pedersen128::hash_to_i64(1field);
let result: i128 = Pedersen128::hash_to_i128(1field);
let result: u8 = Pedersen128::hash_to_u8(1field);
let result: u16 = Pedersen128::hash_to_u16(1field);
let result: u32 = Pedersen128::hash_to_u32(1field);
let result: u64 = Pedersen128::hash_to_u64(1field);
let result: u128 = Pedersen128::hash_to_u128(1field);

// Raw hash variants
let result: address = Pedersen128::hash_to_address_raw(1u8);
let result: field = Pedersen128::hash_to_field_raw(2i64);
                        ...
```

Computes a Pedersen hash up to a 128-bit input in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given `struct` value exceeds the 64-bit limit.

#### Supported Types

| First    | Destination                                                                                               |
| -------- | :-------------------------------------------------------------------------------------------------------- |
| `bool`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `Pedersen128::commit_to_TYPE`

```leo
let salt: scalar = ChaCha::rand_scalar();
let a: address = Pedersen128::commit_to_address(1u8, salt);
let b: field = Pedersen128::commit_to_field(2i64, salt);
let c: group = Pedersen128::commit_to_group(1field, salt);
```

Computes a Pedersen commitment up to a 128-bit input in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given `struct` value exceeds the 128-bit limit.

#### Supported Types

| First    | Second   | Destination                 |
| -------- | -------- | :-------------------------- |
| `bool`   | `scalar` | `address`, `field`, `group` |
| `i8`     | `scalar` | `address`, `field`, `group` |
| `i16`    | `scalar` | `address`, `field`, `group` |
| `i32`    | `scalar` | `address`, `field`, `group` |
| `i64`    | `scalar` | `address`, `field`, `group` |
| `u8`     | `scalar` | `address`, `field`, `group` |
| `u16`    | `scalar` | `address`, `field`, `group` |
| `u32`    | `scalar` | `address`, `field`, `group` |
| `u64`    | `scalar` | `address`, `field`, `group` |
| `struct` | `scalar` | `address`, `field`, `group` |

[Back to Top](#table-of-contents)

---

## Poseidon

### `Poseidon2::hash_to_TYPE`

```leo
let result: address = Poseidon2::hash_to_address(1u8);
let result: field = Poseidon2::hash_to_field(2i64);
let result: group = Poseidon2::hash_to_group(1field);
let result: scalar = Poseidon2::hash_to_scalar(1field);
let result: i8 = Poseidon2::hash_to_i8(1field);
let result: i16 = Poseidon2::hash_to_i16(1field);
let result: i32 = Poseidon2::hash_to_i32(1field);
let result: i64 = Poseidon2::hash_to_i64(1field);
let result: i128 = Poseidon2::hash_to_i128(1field);
let result: u8 = Poseidon2::hash_to_u8(1field);
let result: u16 = Poseidon2::hash_to_u16(1field);
let result: u32 = Poseidon2::hash_to_u32(1field);
let result: u64 = Poseidon2::hash_to_u64(1field);
let result: u128 = Poseidon2::hash_to_u128(1field);

// Raw hash variants
let result: address = Poseidon2::hash_to_address_raw(1u8);
let result: field = Poseidon2::hash_to_field_raw(2i64);
                        ...
```

Calculates a Poseidon hash with an input rate of 2, from an input in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.
s

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `Poseidon4::hash_to_TYPE`

```leo
let result: address = Poseidon4::hash_to_address(1u8);
let result: field = Poseidon4::hash_to_field(2i64);
let result: group = Poseidon4::hash_to_group(1field);
let result: scalar = Poseidon4::hash_to_scalar(1field);
let result: i8 = Poseidon4::hash_to_i8(1field);
let result: i16 = Poseidon4::hash_to_i16(1field);
let result: i32 = Poseidon4::hash_to_i32(1field);
let result: i64 = Poseidon4::hash_to_i64(1field);
let result: i128 = Poseidon4::hash_to_i128(1field);
let result: u8 = Poseidon4::hash_to_u8(1field);
let result: u16 = Poseidon4::hash_to_u16(1field);
let result: u32 = Poseidon4::hash_to_u32(1field);
let result: u64 = Poseidon4::hash_to_u64(1field);
let result: u128 = Poseidon4::hash_to_u128(1field);

// Raw hash variants
let result: address = Poseidon4::hash_to_address_raw(1u8);
let result: field = BHPPoseidon4256::hash_to_field_raw(2i64);
                        ...
```

Calculates a Poseidon hash with an input rate of 4, from an input in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `Poseidon8::hash_to_TYPE`

```leo
let result: address = Poseidon8::hash_to_address(1u8);
let result: field = Poseidon8::hash_to_field(2i64);
let result: group = Poseidon8::hash_to_group(1field);
let result: scalar = Poseidon8::hash_to_scalar(1field);
let result: i8 = Poseidon8::hash_to_i8(1field);
let result: i16 = Poseidon8::hash_to_i16(1field);
let result: i32 = Poseidon8::hash_to_i32(1field);
let result: i64 = Poseidon8::hash_to_i64(1field);
let result: i128 = Poseidon8::hash_to_i128(1field);
let result: u8 = Poseidon8::hash_to_u8(1field);
let result: u16 = Poseidon8::hash_to_u16(1field);
let result: u32 = Poseidon8::hash_to_u32(1field);
let result: u64 = Poseidon8::hash_to_u64(1field);
let result: u128 = Poseidon8::hash_to_u128(1field);

// Raw hash variants
let result: address = Poseidon8::hash_to_address_raw(1u8);
let result: field = Poseidon8::hash_to_field_raw(2i64);
                        ...
```

Calculates a Poseidon hash with an input rate of 8, from an input in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

## Keccak

### `Keccak256::hash_to_bits`

```leo
let result: [bool; 256] = Keccak256::hash_to_bits(1u8);
let result: [bool; 256] = Keccak256::hash_to_bits(2i64);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);
let result: [bool; 256] = Keccak256::hash_to_bits(1field);

// Raw hash variants
let result: [bool; 256] = Keccak256::hash_to_bits_raw(1u8);
let result: [bool; 256] = Keccak256::hash_to_bits_raw(2i64);
                        ...
```

Computes a Keccak256 hash on inputs of 256-bit chunks in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | :------------ |
| `address` | `[bool; 256]` |
| `bool`    | `[bool; 256]` |
| `field`   | `[bool; 256]` |
| `group`   | `[bool; 256]` |
| `i8`      | `[bool; 256]` |
| `i16`     | `[bool; 256]` |
| `i32`     | `[bool; 256]` |
| `i64`     | `[bool; 256]` |
| `i128`    | `[bool; 256]` |
| `u8`      | `[bool; 256]` |
| `u16`     | `[bool; 256]` |
| `u32`     | `[bool; 256]` |
| `u64`     | `[bool; 256]` |
| `u128`    | `[bool; 256]` |
| `scalar`  | `[bool; 256]` |
| `struct`  | `[bool; 256]` |

[Back to Top](#table-of-contents)

---

### `Keccak256::hash_to_TYPE`

```leo
let result: address = Keccak256::hash_to_address(1u8);
let result: field = Keccak256::hash_to_field(2i64);
let result: group = Keccak256::hash_to_group(1field);
let result: scalar = Keccak256::hash_to_scalar(1field);
let result: i8 = Keccak256::hash_to_i8(1field);
let result: i16 = Keccak256::hash_to_i16(1field);
let result: i32 = Keccak256::hash_to_i32(1field);
let result: i64 = Keccak256::hash_to_i64(1field);
let result: i128 = Keccak256::hash_to_i128(1field);
let result: u8 = Keccak256::hash_to_u8(1field);
let result: u16 = Keccak256::hash_to_u16(1field);
let result: u32 = Keccak256::hash_to_u32(1field);
let result: u64 = Keccak256::hash_to_u64(1field);
let result: u128 = Keccak256::hash_to_u128(1field);

// Raw hash variants
let result: address = Keccak256::hash_to_address_raw(1u8);
let result: field = Keccak256::hash_to_field_raw(2i64);
                        ...
```

Computes a Keccak256 hash on inputs of 256-bit chunks in `first`, storing the hash in `destination`.
The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `Keccak384::hash_to_bits`

```leo
let result: [bool; 384] = Keccak384::hash_to_bits(1u8);
let result: [bool; 384] = Keccak384::hash_to_bits(2i64);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);
let result: [bool; 384] = Keccak384::hash_to_bits(1field);

// Raw hash variants
let result: [bool; 384] = Keccak384::hash_to_bits_raw(1u8);
let result: [bool; 384] = Keccak384::hash_to_bits_raw(2i64);
                        ...
```

Computes a Keccak384 hash on inputs of 384-bit chunks in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | :------------ |
| `address` | `[bool; 384]` |
| `bool`    | `[bool; 384]` |
| `field`   | `[bool; 384]` |
| `group`   | `[bool; 384]` |
| `i8`      | `[bool; 384]` |
| `i16`     | `[bool; 384]` |
| `i32`     | `[bool; 384]` |
| `i64`     | `[bool; 384]` |
| `i128`    | `[bool; 384]` |
| `u8`      | `[bool; 384]` |
| `u16`     | `[bool; 384]` |
| `u32`     | `[bool; 384]` |
| `u64`     | `[bool; 384]` |
| `u128`    | `[bool; 384]` |
| `scalar`  | `[bool; 384]` |
| `struct`  | `[bool; 384]` |

[Back to Top](#table-of-contents)

---

### `Keccak384::hash_to_TYPE`

```leo
let result: address = Keccak384::hash_to_address(1u8);
let result: field = Keccak384::hash_to_field(2i64);
let result: group = Keccak384::hash_to_group(1field);
let result: scalar = Keccak384::hash_to_scalar(1field);
let result: i8 = Keccak384::hash_to_i8(1field);
let result: i16 = Keccak384::hash_to_i16(1field);
let result: i32 = Keccak384::hash_to_i32(1field);
let result: i64 = Keccak384::hash_to_i64(1field);
let result: i128 = Keccak384::hash_to_i128(1field);
let result: u8 = Keccak384::hash_to_u8(1field);
let result: u16 = Keccak384::hash_to_u16(1field);
let result: u32 = Keccak384::hash_to_u32(1field);
let result: u64 = Keccak384::hash_to_u64(1field);
let result: u128 = Keccak384::hash_to_u128(1field);

// Raw hash variants
let result: address = Keccak384::hash_to_address_raw(1u8);
let result: field = Keccak384::hash_to_field_raw(2i64);
                        ...
```

Computes a Keccak384 hash on inputs of 384-bit chunks in `first`, storing the hash in `destination`.
The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `Keccak512::hash_to_bits`

```leo
let result: [bool; 512] = Keccak512::hash_to_bits(1u8);
let result: [bool; 512] = Keccak512::hash_to_bits(2i64);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);
let result: [bool; 512] = Keccak512::hash_to_bits(1field);

// Raw hash variants
let result: [bool; 512] = Keccak512::hash_to_bits_raw(1u8);
let result: [bool; 512] = Keccak512::hash_to_bits_raw(2i64);
                        ...
```

Computes a Keccak512 hash on inputs of 512-bit chunks in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | :------------ |
| `address` | `[bool; 512]` |
| `bool`    | `[bool; 512]` |
| `field`   | `[bool; 512]` |
| `group`   | `[bool; 512]` |
| `i8`      | `[bool; 512]` |
| `i16`     | `[bool; 512]` |
| `i32`     | `[bool; 512]` |
| `i64`     | `[bool; 512]` |
| `i128`    | `[bool; 512]` |
| `u8`      | `[bool; 512]` |
| `u16`     | `[bool; 512]` |
| `u32`     | `[bool; 512]` |
| `u64`     | `[bool; 512]` |
| `u128`    | `[bool; 512]` |
| `scalar`  | `[bool; 512]` |
| `struct`  | `[bool; 512]` |

[Back to Top](#table-of-contents)

---

### `Keccak512::hash_to_TYPE`

```leo
let result: address = Keccak512::hash_to_address(1u8);
let result: field = Keccak512::hash_to_field(2i64);
let result: group = Keccak512::hash_to_group(1field);
let result: scalar = Keccak512::hash_to_scalar(1field);
let result: i8 = Keccak512::hash_to_i8(1field);
let result: i16 = Keccak512::hash_to_i16(1field);
let result: i32 = Keccak512::hash_to_i32(1field);
let result: i64 = Keccak512::hash_to_i64(1field);
let result: i128 = Keccak512::hash_to_i128(1field);
let result: u8 = Keccak512::hash_to_u8(1field);
let result: u16 = Keccak512::hash_to_u16(1field);
let result: u32 = Keccak512::hash_to_u32(1field);
let result: u64 = Keccak512::hash_to_u64(1field);
let result: u128 = Keccak512::hash_to_u128(1field);

// Raw hash variants
let result: address = Keccak512::hash_to_address_raw(1u8);
let result: field = Keccak512::hash_to_field_raw(2i64);
                        ...
```

Computes a Keccak512 hash on inputs of 512-bit chunks in `first`, storing the hash in `destination`.
The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

## SHA3

### `SHA3_256::hash_to_bits`

```leo
let result: [bool; 256] = SHA3_256::hash_to_bits(1u8);
let result: [bool; 256] = SHA3_256::hash_to_bits(2i64);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);
let result: [bool; 256] = SHA3_256::hash_to_bits(1field);

// Raw hash variants
let result: [bool; 256] = SHA3_256::hash_to_bits_raw(1u8);
let result: [bool; 256] = SHA3_256::hash_to_bits_raw(2i64);
                        ...
```

Computes a SHA3_256 hash from an input in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | :------------ |
| `address` | `[bool; 256]` |
| `bool`    | `[bool; 256]` |
| `field`   | `[bool; 256]` |
| `group`   | `[bool; 256]` |
| `i8`      | `[bool; 256]` |
| `i16`     | `[bool; 256]` |
| `i32`     | `[bool; 256]` |
| `i64`     | `[bool; 256]` |
| `i128`    | `[bool; 256]` |
| `u8`      | `[bool; 256]` |
| `u16`     | `[bool; 256]` |
| `u32`     | `[bool; 256]` |
| `u64`     | `[bool; 256]` |
| `u128`    | `[bool; 256]` |
| `scalar`  | `[bool; 256]` |
| `struct`  | `[bool; 256]` |

[Back to Top](#table-of-contents)

---

### `SHA3_256::hash_to_TYPE`

```leo
let result: address = SHA3_256::hash_to_address(1u8);
let result: field = SHA3_256::hash_to_field(2i64);
let result: group = SHA3_256::hash_to_group(1field);
let result: scalar = SHA3_256::hash_to_scalar(1field);
let result: i8 = SHA3_256::hash_to_i8(1field);
let result: i16 = SHA3_256::hash_to_i16(1field);
let result: i32 = SHA3_256::hash_to_i32(1field);
let result: i64 = SHA3_256::hash_to_i64(1field);
let result: i128 = SHA3_256::hash_to_i128(1field);
let result: u8 = SHA3_256::hash_to_u8(1field);
let result: u16 = SHA3_256::hash_to_u16(1field);
let result: u32 = SHA3_256::hash_to_u32(1field);
let result: u64 = SHA3_256::hash_to_u64(1field);
let result: u128 = SHA3_256::hash_to_u128(1field);

// Raw hash variants
let result: address = SHA3_256::hash_to_address_raw(1u8);
let result: field = SHA3_256::hash_to_field_raw(2i64);
                        ...
```

Calculates a SHA3_256 hash from an input in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `SHA3_384::hash_to_bits`

```leo
let result: [bool; 384] = SHA3_384::hash_to_bits(1u8);
let result: [bool; 384] = SHA3_384::hash_to_bits(2i64);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);
let result: [bool; 384] = SHA3_384::hash_to_bits(1field);

// Raw hash variants
let result: [bool; 384] = SHA3_384::hash_to_bits_raw(1u8);
let result: [bool; 384] = SHA3_384::hash_to_bits_raw(2i64);
                        ...
```

Computes a SHA3_384 hash from an input in in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | :------------ |
| `address` | `[bool; 384]` |
| `bool`    | `[bool; 384]` |
| `field`   | `[bool; 384]` |
| `group`   | `[bool; 384]` |
| `i8`      | `[bool; 384]` |
| `i16`     | `[bool; 384]` |
| `i32`     | `[bool; 384]` |
| `i64`     | `[bool; 384]` |
| `i128`    | `[bool; 384]` |
| `u8`      | `[bool; 384]` |
| `u16`     | `[bool; 384]` |
| `u32`     | `[bool; 384]` |
| `u64`     | `[bool; 384]` |
| `u128`    | `[bool; 384]` |
| `scalar`  | `[bool; 384]` |
| `struct`  | `[bool; 384]` |

[Back to Top](#table-of-contents)

---

### `SHA3_384::hash_to_TYPE`

```leo
let result: address = SHA3_384::hash_to_address(1u8);
let result: field = SHA3_384::hash_to_field(2i64);
let result: group = SHA3_384::hash_to_group(1field);
let result: scalar = SHA3_384::hash_to_scalar(1field);
let result: i8 = SHA3_384::hash_to_i8(1field);
let result: i16 = SHA3_384::hash_to_i16(1field);
let result: i32 = SHA3_384::hash_to_i32(1field);
let result: i64 = SHA3_384::hash_to_i64(1field);
let result: i128 = SHA3_384::hash_to_i128(1field);
let result: u8 = SHA3_384::hash_to_u8(1field);
let result: u16 = SHA3_384::hash_to_u16(1field);
let result: u32 = SHA3_384::hash_to_u32(1field);
let result: u64 = SHA3_384::hash_to_u64(1field);
let result: u128 = SHA3_384::hash_to_u128(1field);

// Raw hash variants
let result: address = SHA3_384::hash_to_address_raw(1u8);
let result: field = SHA3_384::hash_to_field_raw(2i64);
                        ...
```

Calculates a SHA3_384 hash from an input in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

### `SHA3_512::hash_to_bits`

```leo
let result: [bool; 512] = SHA3_512::hash_to_bits(1u8);
let result: [bool; 512] = SHA3_512::hash_to_bits(2i64);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);
let result: [bool; 512] = SHA3_512::hash_to_bits(1field);

// Raw hash variants
let result: [bool; 512] = SHA3_512::hash_to_bits_raw(1u8);
let result: [bool; 512] = SHA3_512::hash_to_bits_raw(2i64);
                        ...
```

Computes a SHA3_512 hash from an input in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | :------------ |
| `address` | `[bool; 512]` |
| `bool`    | `[bool; 512]` |
| `field`   | `[bool; 512]` |
| `group`   | `[bool; 512]` |
| `i8`      | `[bool; 512]` |
| `i16`     | `[bool; 512]` |
| `i32`     | `[bool; 512]` |
| `i64`     | `[bool; 512]` |
| `i128`    | `[bool; 512]` |
| `u8`      | `[bool; 512]` |
| `u16`     | `[bool; 512]` |
| `u32`     | `[bool; 512]` |
| `u64`     | `[bool; 512]` |
| `u128`    | `[bool; 512]` |
| `scalar`  | `[bool; 512]` |
| `struct`  | `[bool; 512]` |

[Back to Top](#table-of-contents)

---

### `SHA3_512::hash_to_TYPE`

```leo
let result: address = SHA3_512::hash_to_address(1u8);
let result: field = SHA3_512::hash_to_field(2i64);
let result: group = SHA3_512::hash_to_group(1field);
let result: scalar = SHA3_512::hash_to_scalar(1field);
let result: i8 = SHA3_512::hash_to_i8(1field);
let result: i16 = SHA3_512::hash_to_i16(1field);
let result: i32 = SHA3_512::hash_to_i32(1field);
let result: i64 = SHA3_512::hash_to_i64(1field);
let result: i128 = SHA3_512::hash_to_i128(1field);
let result: u8 = SHA3_512::hash_to_u8(1field);
let result: u16 = SHA3_512::hash_to_u16(1field);
let result: u32 = SHA3_512::hash_to_u32(1field);
let result: u64 = SHA3_512::hash_to_u64(1field);
let result: u128 = SHA3_512::hash_to_u128(1field);

// Raw hash variants
let result: address = SHA3_512::hash_to_address_raw(1u8);
let result: field = SHA3_512::hash_to_field_raw(2i64);
                        ...
```

Calculates a SHA3_512 hash from an input in `first`, storing the hash in `destination`. The produced hash will always be an arithmetic (`u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`,`i64`,`i128`, `field`, `group`, or `scalar`) or `address` value, as specified via `hash_to_TYPE` at the end of the function.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | :-------------------------------------------------------------------------------------------------------- |
| `address` | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `bool`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `field`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `group`   | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `i128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u8`      | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u16`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u32`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u64`     | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `u128`    | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `scalar`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |
| `struct`  | `address`, `field`, `group`, `scalar`, `i8`, `i16`, `i32`,`i64`,`i128`, `u8`, `u16`, `u32`, `u64`, `u128` |

[Back to Top](#table-of-contents)

---

## ChaCha

### `ChaCha::rand_TYPE`

```leo
let result: address = ChaCha::rand_address();
let result: bool = ChaCha::rand_bool();
let result: field = ChaCha::rand_field();
let result: group = ChaCha::rand_group();
let result: i8 = ChaCha::rand_i8();
let result: i16 = ChaCha::rand_i16();
let result: i32 = ChaCha::rand_i32();
let result: i64 = ChaCha::rand_i64();
let result: i128 = ChaCha::rand_i128();
let result: u8 = ChaCha::rand_u8();
let result: u16 = ChaCha::rand_u16();
let result: u32 = ChaCha::rand_u32();
let result: u64 = ChaCha::rand_u64();
let result: u128 = ChaCha::rand_u128();
let result: scalar = ChaCha::rand_scalar();
```

Returns a random value with the destination type.

:::info
This operation can only be used inside a `final { }` block or inside a `final fn`.
:::

#### Supported Types

| Destination |
| ----------- |
| `address`   |
| `bool`      |
| `field`     |
| `group`     |
| `i8`        |
| `i16`       |
| `i32`       |
| `i64`       |
| `i128`      |
| `u8`        |
| `u16`       |
| `u32`       |
| `u64`       |
| `u128`      |
| `scalar`    |

[Back to Top](#table-of-contents)

---

## Schnorr Signatures

### `signature::verify`

```leo
program example.aleo {
    fn verify_field(s: signature, a: address, v: field) {
        let first: bool = signature::verify(s, a, v);
        let second: bool = s.verify(a, v);
        assert_eq(first, second);
    }
}
```

Verifies that the signature `first` was signed by the address `second` with respect to the field `third`, storing the result in `destination`. This verification follows the [Schnorr signature scheme](https://en.wikipedia.org/wiki/Schnorr_signature), which is a digital signature algorithm where the signer generates a random nonce, commits to it, computes a challenge using a hash function, and produces a signature by combining the nonce, challenge, and private key. The verifier checks the validity by reconstructing the challenge and ensuring consistency with the public key and message.

#### Supported Types

A `Message` is any literal or `struct` type.

| First       | Second    | Third     | Destination |
| ----------- | --------- | --------- | ----------- |
| `signature` | `address` | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

## ECDSA Signatures

### `ECDSA::verify_digest`

```leo
let valid: bool = ECDSA::verify_digest(sig, addr, digest);
let valid: bool = ECDSA::verify_digest_eth(sig, eth_addr, digest);
```

Verifies that the signature `first` was signed by the private key corresponding to the address `second` with respect to the field `third`, storing the result in `destination`. This function assumes that value passed as `third` is the output from a hash function that was previously computed. The standard version of `verify_digest` assume that `second` is a 33-byte ECDSA public key, while the `verify_digest_eth` version assumes that `second` is a 20-byte Ethereum address.

This verification follows the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm); an algorithm widely used across many other blockchains and legacy systems.

#### Supported Types

| First      | Second     | Second (Eth) | Third      | Destination |
| ---------- | ---------- | ------------ | ---------- | ----------- |
| `[u8;65] ` | `[u8; 33]` | `[u8; 20]`   | `[u8; 32]` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_keccak256`

```leo
let valid: bool = ECDSA::verify_keccak256(sig, addr, msg);
let valid: bool = ECDSA::verify_keccak256_raw(sig, addr, msg);
let valid: bool = ECDSA::verify_keccak256_eth(sig, eth_addr, msg);
```

Verifies that the signature `first` was signed by the private key corresponding to the address `second` with respect to the field `third`, storing the result in `destination`. This function assumes that value passed as `third` is the plaintext message bytes, which will be hashed using the Keccak256 algorithm. The standard version of `verify_keccak256` will include the Aleo specific metadata alongside the input, while the `verify_keccak256_raw` version will exclude the metadata. The `verify_keccak256_eth` will both exclude the metadata and assume that `second` is a 20-byte Ethereum address.

This verification follows the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm); an algorithm widely used across many other blockchains and legacy systems.

#### Supported Types

A `Message` is any byte-aligned type.

| First      | Second     | Second (Eth) | Third     | Destination |
| ---------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65] ` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_keccak384`

```leo
let valid: bool = ECDSA::verify_keccak384(sig, addr, msg);
let valid: bool = ECDSA::verify_keccak384_raw(sig, addr, msg);
let valid: bool = ECDSA::verify_keccak384_eth(sig, eth_addr, msg);
```

Verifies that the signature `first` was signed by the private key corresponding to the address `second` with respect to the field `third`, storing the result in `destination`. This function assumes that value passed as `third` is the plaintext message bytes, which will be hashed using the Keccak384 algorithm. The standard version of `verify_keccak384` will include the Aleo specific metadata alongside the input, while the `verify_keccak384_raw` version will exclude the metadata. The `verify_keccak384_eth` will both exclude the metadata and assume that `second` is a 20-byte Ethereum address.

This verification follows the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm); an algorithm widely used across many other blockchains and legacy systems.

#### Supported Types

A `Message` is any byte-aligned type.

| First      | Second     | Second (Eth) | Third     | Destination |
| ---------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65] ` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_keccak512`

```leo
let valid: bool = ECDSA::verify_keccak512(sig, addr, msg);
let valid: bool = ECDSA::verify_keccak512_raw(sig, addr, msg);
let valid: bool = ECDSA::verify_keccak512_eth(sig, eth_addr, msg);
```

Verifies that the signature `first` was signed by the private key corresponding to the address `second` with respect to the field `third`, storing the result in `destination`. This function assumes that value passed as `third` is the plaintext message bytes, which will be hashed using the Keccak512 algorithm. The standard version of `verify_keccak512` will include the Aleo specific metadata alongside the input, while the `verify_keccak512_raw` version will exclude the metadata. The `verify_keccak512_eth` will both exclude the metadata and assume that `second` is a 20-byte Ethereum address.

This verification follows the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm); an algorithm widely used across many other blockchains and legacy systems.

#### Supported Types

A `Message` is any byte-aligned type.

| First      | Second     | Second (Eth) | Third     | Destination |
| ---------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65] ` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_sha3_256`

```leo
let valid: bool = ECDSA::verify_sha3_256(sig, addr, msg);
let valid: bool = ECDSA::verify_sha3_256_raw(sig, addr, msg);
let valid: bool = ECDSA::verify_sha3_256_eth(sig, eth_addr, msg);
```

Verifies that the signature `first` was signed by the private key corresponding to the address `second` with respect to the field `third`, storing the result in `destination`. This function assumes that value passed as `third` is the plaintext message bytes, which will be hashed using the SHA3_256 algorithm. The standard version of `verify_sha3_256` will include the Aleo specific metadata alongside the input, while the `verify_sha3_256_raw` version will exclude the metadata. The `verify_sha3_256_eth` will both exclude the metadata and assume that `second` is a 20-byte Ethereum address.

This verification follows the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm); an algorithm widely used across many other blockchains and legacy systems.

#### Supported Types

A `Message` is any byte-aligned type.

| First      | Second     | Second (Eth) | Third     | Destination |
| ---------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65] ` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_sha3_384`

```leo
let valid: bool = ECDSA::verify_sha3_384(sig, addr, msg);
let valid: bool = ECDSA::verify_sha3_384_raw(sig, addr, msg);
let valid: bool = ECDSA::verify_sha3_384_eth(sig, eth_addr, msg);
```

Verifies that the signature `first` was signed by the private key corresponding to the address `second` with respect to the field `third`, storing the result in `destination`. This function assumes that value passed as `third` is the plaintext message bytes, which will be hashed using the SHA3_384 algorithm. The standard version of `verify_sha3_384` will include the Aleo specific metadata alongside the input, while the `verify_sha3_384_raw` version will exclude the metadata. The `verify_sha3_384_eth` will both exclude the metadata and assume that `second` is a 20-byte Ethereum address.

This verification follows the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm); an algorithm widely used across many other blockchains and legacy systems.

#### Supported Types

A `Message` is any byte-aligned type.

| First      | Second     | Second (Eth) | Third     | Destination |
| ---------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65] ` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_sha3_512`

```leo
let valid: bool = ECDSA::verify_sha3_512(sig, addr, msg);
let valid: bool = ECDSA::verify_sha3_512_raw(sig, addr, msg);
let valid: bool = ECDSA::verify_sha3_512_eth(sig, eth_addr, msg);
```

Verifies that the signature `first` was signed by the private key corresponding to the address `second` with respect to the field `third`, storing the result in `destination`. This function assumes that value passed as `third` is the plaintext message bytes, which will be hashed using the SHA3_512 algorithm. The standard version of `verify_sha3_512` will include the Aleo specific metadata alongside the input, while the `verify_sha3_512_raw` version will exclude the metadata. The `verify_sha3_512_eth` will both exclude the metadata and assume that `second` is a 20-byte Ethereum address.

This verification follows the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm); an algorithm widely used across many other blockchains and legacy systems.

#### Supported Types

A `Message` is any byte-aligned type.

| First      | Second     | Second (Eth) | Third     | Destination |
| ---------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65] ` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

## zk-SNARKs

### `Snark::verify`

```leo
let valid: bool = Snark::verify(vk, version, inputs, proof);
```

Verifies a single Varuna ZK proof on-chain. Only callable from inside a `final { }` block.

| Argument  | Type          | Description                                   |
| --------- | ------------- | --------------------------------------------- |
| `vk`      | `[u8; N]`     | Serialized verifying key (1-D byte array)      |
| `version` | `u8`          | Varuna version identifier                     |
| `inputs`  | `[field; N]`  | Public inputs (1-D field array)               |
| `proof`   | `[u8; N]`     | Serialized proof (1-D byte array)             |

Returns `bool`.

[Back to Top](#table-of-contents)

---

### `Snark::verify_batch`

```leo
let valid: bool = Snark::verify_batch(vks, version, inputs, proof);
```

Batch-verifies multiple Varuna ZK proofs on-chain. Only callable from inside a `final { }` block. The number of verifying keys (`M`) must equal the number of circuits in `inputs` (`K`).

| Argument  | Type               | Description                                                             |
| --------- | ------------------ | ----------------------------------------------------------------------- |
| `vks`     | `[[u8; N]; M]`     | Array of serialized verifying keys                                      |
| `version` | `u8`               | Varuna version identifier                                               |
| `inputs`  | `[[[field; N]; M]; K]` | Public inputs — outer dimension is circuits, middle is instances    |
| `proof`   | `[u8; N]`          | Serialized proof (1-D byte array)                                       |

Returns `bool`.

[Back to Top](#table-of-contents)

---

---
