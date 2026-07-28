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

The `hash` variant is a one-way function. It takes an input and produces a fixed-size hash or digest.
If one input bit changes, the output hash changes completely. Thus, a hash can show a change to the data.

The `commit` variant wraps the `hash` variant and takes an additional blinding factor, or **salt**.
The function adds the salt to the input before it calculates the hash. This operation gives a different output for the raw input.
If you use a different salt each time, you can commit the same value more than once without disclosure.

## Table of Contents

| Name                                                      | Description                                     |
| --------------------------------------------------------- | ----------------------------------------------- |
| [BHP256::hash_to_TYPE](#bhp256hash_to_type)               | 256-bit input BHP hash                          |
| [BHP256::commit_to_TYPE](#bhp256commit_to_type)           | 256-bit input BHP commitment                    |
| [BHP512::hash_to_TYPE](#bhp512hash_to_type)               | 512-bit input BHP hash                          |
| [BHP512::commit_to_TYPE](#bhp512commit_to_type)           | 512-bit input BHP commitment                    |
| [BHP768::hash_to_TYPE](#bhp768hash_to_type)               | 768-bit input BHP hash                          |
| [BHP768::commit_to_TYPE](#bhp768commit_to_type)           | 768-bit input BHP commitment                    |
| [BHP1024::hash_to_TYPE](#bhp1024hash_to_type)             | 1024-bit input BHP hash                         |
| [BHP1024::commit_to_TYPE](#bhp1024commit_to_type)         | 1024-bit input BHP commitment                   |
| [Pedersen64::hash_to_TYPE](#pedersen64hash_to_type)       | 64-bit input Pedersen hash                      |
| [Pedersen64::commit_to_TYPE](#pedersen64commit_to_type)   | 64-bit input Pedersen commitment                |
| [Pedersen128::hash_to_TYPE](#pedersen128hash_to_type)     | 128-bit input Pedersen hash                     |
| [Pedersen128::commit_to_TYPE](#pedersen128commit_to_type) | 128-bit input Pedersen commitment               |
| [Poseidon2::hash_to_TYPE](#poseidon2hash_to_type)         | Poseidon hash with input rate 2                 |
| [Poseidon4::hash_to_TYPE](#poseidon4hash_to_type)         | Poseidon hash with input rate 4                 |
| [Poseidon8::hash_to_TYPE](#poseidon8hash_to_type)         | Poseidon hash with input rate 8                 |
| [Keccak256::hash_to_bits](#keccak256hash_to_bits)         | 256-bit input/output Keccak hash                |
| [Keccak256::hash_to_TYPE](#keccak256hash_to_type)         | 256-bit input Keccak hash                       |
| [Keccak384::hash_to_bits](#keccak384hash_to_bits)         | 384-bit input/output Keccak hash                |
| [Keccak384::hash_to_TYPE](#keccak384hash_to_type)         | 384-bit input Keccak hash                       |
| [Keccak512::hash_to_bits](#keccak512hash_to_bits)         | 512-bit input/output Keccak hash                |
| [Keccak512::hash_to_TYPE](#keccak512hash_to_type)         | 512-bit input Keccak hash                       |
| [SHA3_256::hash_to_bits](#sha3_256hash_to_bits)           | 256-bit input/output SHA3 hash                  |
| [SHA3_256::hash_to_TYPE](#sha3_256hash_to_type)           | 256-bit input SHA3 hash                         |
| [SHA3_384::hash_to_bits](#sha3_384hash_to_bits)           | 384-bit input/output SHA3 hash                  |
| [SHA3_384::hash_to_TYPE](#sha3_384hash_to_type)           | 384-bit input SHA3 hash                         |
| [SHA3_512::hash_to_bits](#sha3_512hash_to_bits)           | 512-bit input/output SHA3 hash                  |
| [SHA3_512::hash_to_TYPE](#sha3_512hash_to_type)           | 512-bit input SHA3 hash                         |
| [ChaCha::rand_TYPE](#chacharand_type)                     | ChaCha RNG                                      |
| [signature::verify](#signatureverify)                     | Verify a Schnorr signature                      |
| [ECDSA::verify_digest](#ecdsaverify_digest)               | Verify an ECDSA signature against a pre-hash    |
| [ECDSA::verify_keccak256](#ecdsaverify_keccak256)         | Verify an ECDSA signature using Keccak256       |
| [ECDSA::verify_keccak384](#ecdsaverify_keccak384)         | Verify an ECDSA signature using Keccak384       |
| [ECDSA::verify_keccak512](#ecdsaverify_keccak512)         | Verify an ECDSA signature using Keccak512       |
| [ECDSA::verify_sha3_256](#ecdsaverify_sha3_256)           | Verify an ECDSA signature using SHA3_256        |
| [ECDSA::verify_sha3_384](#ecdsaverify_sha3_384)           | Verify an ECDSA signature using SHA3_384        |
| [ECDSA::verify_sha3_512](#ecdsaverify_sha3_512)           | Verify an ECDSA signature using SHA3_512        |
| [Snark::verify](#snarkverify)                             | Verify a single Varuna ZK proof on-chain        |
| [Snark::verify_batch](#snarkverify_batch)                 | Batch-verify multiple Varuna ZK proofs on-chain |

## Bowe-Hopwood-Pedersen (BHP)

### `BHP256::hash_to_TYPE`

```leo file=../../code_snippets/operators/crypto/src/main.leo#bhp256_hash
```

Computes a Bowe-Hopwood-Pedersen hash on inputs of 256-bit chunks in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given input is smaller than 129 bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#bhp256_commit
```

Computes a Bowe-Hopwood-Pedersen commitment on inputs of 256-bit chunks in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment can be an `address`, `field` or, `group` value.

The instruction will halt if the given input is smaller than 129 bits.

#### Supported Types

| First     | Second   | Destination                 |
| --------- | -------- | --------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#bhp512_hash
```

Computes a Bowe-Hopwood-Pedersen hash on inputs of 512-bit chunks in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given input is smaller than 171 bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#bhp512_commit
```

Computes a Bowe-Hopwood-Pedersen commitment on inputs of 512-bit chunks in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given input is smaller than 171 bits.

#### Supported Types

| First     | Second   | Destination                 |
| --------- | -------- | --------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#bhp768_hash
```

Computes a Bowe-Hopwood-Pedersen hash on inputs of 768-bit chunks in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given input is smaller than 129 bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#bhp768_commit
```

Computes a Bowe-Hopwood-Pedersen commitment on inputs of 768-bit chunks in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given input is smaller than 129 bits.

#### Supported Types

| First     | Second   | Destination                 |
| --------- | -------- | --------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#bhp1024_hash
```

Computes a Bowe-Hopwood-Pedersen hash on inputs of 1024-bit chunks in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given input is smaller than 171 bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#bhp1024_commit
```

Computes a Bowe-Hopwood-Pedersen commitment on inputs of 1024-bit chunks in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given input is smaller than 171 bits.

#### Supported Types

| First     | Second   | Destination                 |
| --------- | -------- | --------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#pedersen64_hash
```

Computes a Pedersen hash up to a 64-bit input in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given `struct` value exceeds the 64-bit limit.

#### Supported Types

| First    | Destination                                                                                               |
| -------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#pedersen64_commit
```

Computes a Pedersen commitment up to a 64-bit input in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given `struct` value exceeds the 64-bit limit.

#### Supported Types

| First    | Second   | Destination                 |
| -------- | -------- | --------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#pedersen128_hash
```

Computes a Pedersen hash up to a 128-bit input in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

The instruction will halt if the given `struct` value exceeds the 64-bit limit.

#### Supported Types

| First    | Destination                                                                                               |
| -------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#pedersen128_commit
```

Computes a Pedersen commitment up to a 128-bit input in `first`, and some randomness in `second`, storing the commitment in `destination`. Randomness should always be a `scalar` value, and the produced commitment will always be a `group` value.

The instruction will halt if the given `struct` value exceeds the 128-bit limit.

#### Supported Types

| First    | Second   | Destination                 |
| -------- | -------- | --------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#poseidon2_hash
```

Calculates a Poseidon hash with an input rate of 2, from an input in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#poseidon4_hash
```

Calculates a Poseidon hash with an input rate of 4, from an input in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#poseidon8_hash
```

Calculates a Poseidon hash with an input rate of 8, from an input in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#keccak256_hash_to_bits
```

Computes a Keccak256 hash on inputs of 256-bit chunks in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | ------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#keccak256_hash
```

Computes a Keccak256 hash on inputs of 256-bit chunks in `first`, storing the hash in `destination`.
The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#keccak384_hash_to_bits
```

Computes a Keccak384 hash on inputs of 384-bit chunks in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | ------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#keccak384_hash
```

Computes a Keccak384 hash on inputs of 384-bit chunks in `first`, storing the hash in `destination`.
The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#keccak512_hash_to_bits
```

Computes a Keccak512 hash on inputs of 512-bit chunks in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | ------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#keccak512_hash
```

Computes a Keccak512 hash on inputs of 512-bit chunks in `first`, storing the hash in `destination`.
The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#sha3_256_hash_to_bits
```

Computes a SHA3_256 hash from an input in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | ------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#sha3_256_hash
```

Calculates a SHA3_256 hash from an input in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#sha3_384_hash_to_bits
```

Computes a SHA3_384 hash from an input in in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | ------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#sha3_384_hash
```

Calculates a SHA3_384 hash from an input in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#sha3_512_hash_to_bits
```

Computes a SHA3_512 hash from an input in `first`, storing the hash in `destination`. The produced hash will be an array of bits.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination   |
| --------- | ------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#sha3_512_hash
```

Calculates a SHA3_512 hash from an input in `first`, storing the hash in `destination`. The `hash_to_TYPE` suffix specifies the output type. The supported types table gives all valid output types.

By appending `_raw` to the end of the function, the hash function will omit metadata of a variable and directly hash the input bits.

#### Supported Types

| First     | Destination                                                                                               |
| --------- | --------------------------------------------------------------------------------------------------------- |
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#chacha_rand
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

```leo file=../../code_snippets/operators/crypto/src/main.leo#signature_verify
```

Verifies signature `first` against address `second` and field `third`. The function stores the result in `destination`.
This verification uses the [Schnorr signature scheme](https://en.wikipedia.org/wiki/Schnorr_signature).
The signer generates a random nonce and commits to it. Then, the signer calculates a hash challenge.

The signer combines the nonce, challenge, and private key to produce the signature.
The verifier reconstructs the challenge and compares it with the public key and message.

#### Supported Types

A `Message` is any literal or `struct` type.

| First       | Second    | Third     | Destination |
| ----------- | --------- | --------- | ----------- |
| `signature` | `address` | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

## ECDSA Signatures

### `ECDSA::verify_digest`

```leo file=../../code_snippets/operators/crypto/src/main.leo#ecdsa_verify_digest
```

Verifies signature `first` against key or address `second` and value `third`. The function stores the result in `destination`.
The function treats `third` as a precalculated hash. `verify_digest` treats `second` as a 33-byte ECDSA public key.
`verify_digest_eth` treats `second` as a 20-byte Ethereum address.

This verification uses the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm). Many blockchains and legacy systems use this algorithm.

#### Supported Types

| First     | Second     | Second (Eth) | Third      | Destination |
| --------- | ---------- | ------------ | ---------- | ----------- |
| `[u8;65]` | `[u8; 33]` | `[u8; 20]`   | `[u8; 32]` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_keccak256`

```leo file=../../code_snippets/operators/crypto/src/main.leo#ecdsa_verify_keccak256
```

Verifies signature `first` against key or address `second` and value `third`. The function stores the result in `destination`. The function treats `third` as plaintext message bytes. It hashes these bytes with Keccak256.

`verify_keccak256` includes the Aleo-specific metadata. `verify_keccak256_raw` excludes this metadata. `verify_keccak256_eth` excludes the metadata and treats `second` as a 20-byte Ethereum address.

This verification uses the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm). Many blockchains and legacy systems use this algorithm.

#### Supported Types

A `Message` is any byte-aligned type.

| First     | Second     | Second (Eth) | Third     | Destination |
| --------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65]` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_keccak384`

```leo file=../../code_snippets/operators/crypto/src/main.leo#ecdsa_verify_keccak384
```

Verifies signature `first` against key or address `second` and value `third`. The function stores the result in `destination`. The function treats `third` as plaintext message bytes. It hashes these bytes with Keccak384.

`verify_keccak384` includes the Aleo-specific metadata. `verify_keccak384_raw` excludes this metadata. `verify_keccak384_eth` excludes the metadata and treats `second` as a 20-byte Ethereum address.

This verification uses the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm). Many blockchains and legacy systems use this algorithm.

#### Supported Types

A `Message` is any byte-aligned type.

| First     | Second     | Second (Eth) | Third     | Destination |
| --------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65]` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_keccak512`

```leo file=../../code_snippets/operators/crypto/src/main.leo#ecdsa_verify_keccak512
```

Verifies signature `first` against key or address `second` and value `third`. The function stores the result in `destination`. The function treats `third` as plaintext message bytes. It hashes these bytes with Keccak512.

`verify_keccak512` includes the Aleo-specific metadata. `verify_keccak512_raw` excludes this metadata. `verify_keccak512_eth` excludes the metadata and treats `second` as a 20-byte Ethereum address.

This verification uses the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm). Many blockchains and legacy systems use this algorithm.

#### Supported Types

A `Message` is any byte-aligned type.

| First     | Second     | Second (Eth) | Third     | Destination |
| --------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65]` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_sha3_256`

```leo file=../../code_snippets/operators/crypto/src/main.leo#ecdsa_verify_sha3_256
```

Verifies signature `first` against key or address `second` and value `third`. The function stores the result in `destination`. The function treats `third` as plaintext message bytes. It hashes these bytes with SHA3_256.

`verify_sha3_256` includes the Aleo-specific metadata. `verify_sha3_256_raw` excludes this metadata. `verify_sha3_256_eth` excludes the metadata and treats `second` as a 20-byte Ethereum address.

This verification uses the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm). Many blockchains and legacy systems use this algorithm.

#### Supported Types

A `Message` is any byte-aligned type.

| First     | Second     | Second (Eth) | Third     | Destination |
| --------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65]` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_sha3_384`

```leo file=../../code_snippets/operators/crypto/src/main.leo#ecdsa_verify_sha3_384
```

Verifies signature `first` against key or address `second` and value `third`. The function stores the result in `destination`. The function treats `third` as plaintext message bytes. It hashes these bytes with SHA3_384.

`verify_sha3_384` includes the Aleo-specific metadata. `verify_sha3_384_raw` excludes this metadata. `verify_sha3_384_eth` excludes the metadata and treats `second` as a 20-byte Ethereum address.

This verification uses the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm). Many blockchains and legacy systems use this algorithm.

#### Supported Types

A `Message` is any byte-aligned type.

| First     | Second     | Second (Eth) | Third     | Destination |
| --------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65]` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

### `ECDSA::verify_sha3_512`

```leo file=../../code_snippets/operators/crypto/src/main.leo#ecdsa_verify_sha3_512
```

Verifies signature `first` against key or address `second` and value `third`. The function stores the result in `destination`. The function treats `third` as plaintext message bytes. It hashes these bytes with SHA3_512.

`verify_sha3_512` includes the Aleo-specific metadata. `verify_sha3_512_raw` excludes this metadata. `verify_sha3_512_eth` excludes the metadata and treats `second` as a 20-byte Ethereum address.

This verification uses the [ECDSA signature scheme](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm). Many blockchains and legacy systems use this algorithm.

#### Supported Types

A `Message` is any byte-aligned type.

| First     | Second     | Second (Eth) | Third     | Destination |
| --------- | ---------- | ------------ | --------- | ----------- |
| `[u8;65]` | `[u8; 33]` | `[u8; 20]`   | `Message` | `bool`      |

[Back to Top](#table-of-contents)

---

## zk-SNARKs

### `Snark::verify`

```leo file=../../code_snippets/operators/crypto/src/main.leo#snark_verify
```

Verifies a single Varuna ZK proof on-chain. Only callable from inside a `final { }` block.

| Argument  | Type         | Description                               |
| --------- | ------------ | ----------------------------------------- |
| `vk`      | `[u8; N]`    | Serialized verifying key (1-D byte array) |
| `version` | `u8`         | Varuna version identifier                 |
| `inputs`  | `[field; N]` | Public inputs (1-D field array)           |
| `proof`   | `[u8; N]`    | Serialized proof (1-D byte array)         |

Returns `bool`.

[Back to Top](#table-of-contents)

---

### `Snark::verify_batch`

```leo file=../../code_snippets/operators/crypto/src/main.leo#snark_verify_batch
```

Batch-verifies multiple Varuna ZK proofs on-chain. Only callable from inside a `final { }` block. The number of verifying keys (`M`) must equal the number of circuits in `inputs` (`K`).

| Argument  | Type                   | Description                                                      |
| --------- | ---------------------- | ---------------------------------------------------------------- |
| `vks`     | `[[u8; N]; M]`         | Array of serialized verifying keys                               |
| `version` | `u8`                   | Varuna version identifier                                        |
| `inputs`  | `[[[field; N]; M]; K]` | Public inputs — outer dimension is circuits, middle is instances |
| `proof`   | `[u8; N]`              | Serialized proof (1-D byte array)                                |

Returns `bool`.

[Back to Top](#table-of-contents)

---

---
