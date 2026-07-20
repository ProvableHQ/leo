---
id: standard_library
title: Standard Library
sidebar_label: Standard Library
---

[general tags]: # "stdlib, std, hash, commit, signature, random, serialize, context"

The Leo standard library (`std`) is implicitly available in every Leo
program. There is no `program.json` entry to add and no `import` statement
to write: any item under `std::*` is reachable by its qualified path,
e.g. `std::hash::bhp256::hash_u64_to_field(x)` or `std::ctx::caller()`.

A program can opt out of the implicit injection by setting `"no_std": true`
in its `program.json`. With `no_std`, the same operations remain available
through the lower-level operator surface; see
[Cryptographic Operators](./operators/cryptographic_operators.md) and
[Intrinsics](./programs_in_practice/intrinsics.md).

## Scope and finalize context

Functions in `std` come in two flavors:

- A plain `fn` is callable from any transition function and from inside a
  `final { ... }` block.
- A `final fn` may only be called from inside another `final fn` body or
  a `final { ... }` async block. These functions touch on-chain operands
  (block height, randomness, mappings, signature verifiers) that the
  off-chain prover has no access to.

This page calls out the scope of every module that contains `final fn`
entries.

## Modules

- [`std::hash`](#stdhash) — cryptographic hashing
- [`std::commit`](#stdcommit) — hiding/binding commitments
- [`std::sig`](#stdsig) — Schnorr and ECDSA verification
- [`std::rand`](#stdrand) — finalize-context randomness
- [`std::serialize`](#stdserialize) — bit-level encoding and decoding
- [`std::grp`](#stdgrp) — group generators and coordinates
- [`std::ctx`](#stdctx) — execution context
- [`std::prog`](#stdprog) — on-chain metadata for imported programs

---

## `std::hash`

Cryptographic hash functions, grouped by algorithm family. Every algorithm
exposes the same set of `hash_<input>_to_<output>` wrappers; the algorithm
itself determines the security guarantees, the input constraints, and the
proving cost.

### Variants

Most algorithms expose two forms:

- `hash_<input>_to_<output>(x)` prepends a 26-bit type discriminator to
  the input before hashing. Use this form when the hash will be stored,
  compared across types, or embedded in a commitment; the tag prevents
  type confusion between values that happen to share a bit pattern.
- `hash_<input>_to_<output>_raw(x)` hashes the input's native bit
  representation directly with no tag. Use only when the type is fixed on
  both sides of the operation.

Keccak and SHA-3 additionally expose `hash_<input>_to_bits(x)` and
`hash_<input>_to_bits_raw(x)`, which return the full digest as a
`[bool; N]` where N matches the algorithm's output width (256, 384, or
512).

### `std::hash::bhp256`, `bhp512`, `bhp768`, `bhp1024`

BHP is the in-circuit hash family Aleo uses for record commitments and
state digests. It is collision-resistant and cheap to prove inside a
zk-SNARK, the right choice when the hash must be computed and verified
on-chain or inside a circuit. The number in the algorithm name is the
input-pad width in bits; larger pads amortize better over long inputs but
cost more to prove on short ones.

Each algorithm accepts any non-mapping, non-tuple, non-unit input.

```leo file=../code_snippets/standard_library/src/main.leo#std_hash_bhp256
```

| Input                                                                                  | Output                                                                                          |
| -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `bool`, `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`,          | `address`, `field`, `group`, `scalar`,                                                          |
| `field`, `group`, `scalar`, `address`                                                  | `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128` (truncated to bit width)   |

### `std::hash::keccak256`, `keccak384`, `keccak512`

Keccak is the pre-FIPS sponge construction from which SHA-3 was derived;
use it when interoperating with Ethereum or any ecosystem standardized on
the pre-standardization variant. Output bit width matches the suffix
(256, 384, or 512 bits).

```leo file=../code_snippets/standard_library/src/main.leo#std_hash_keccak
```

The non-raw `hash_<input>_to_<output>` functions accept any non-mapping
input. The `_raw`, `_to_bits`, and `_to_bits_raw` variants require
byte-aligned input; only integer types and arrays of bytes satisfy this.

### `std::hash::sha3_256`, `sha3_384`, `sha3_512`

The NIST-standardized SHA-3 variant of the Keccak sponge. Use SHA-3 when
interop requires a FIPS-compliant hash; for EVM-style interop, prefer
Keccak. Variants and input constraints are identical to the Keccak family.

### `std::hash::pedersen64`, `pedersen128`

Pedersen hashing is homomorphic, collision-resistant, and very cheap to
prove inside a zk-SNARK. Capacity is bounded by the suffix:

- `pedersen64` accepts `bool` and integer inputs of 32 bits or fewer.
- `pedersen128` accepts `bool` and integer inputs of 64 bits or fewer.

Inputs that exceed those bounds must use a BHP or Poseidon variant.

```leo file=../code_snippets/standard_library/src/main.leo#std_hash_pedersen
```

### `std::hash::poseidon2`, `poseidon4`, `poseidon8`

Poseidon is the SNARK-friendliest hash in the stdlib. Every step is a
field operation, so the hash costs an order of magnitude fewer circuit
constraints to prove than Keccak or SHA-3. Use Poseidon whenever the hash
is computed and verified in the same proof and EVM interop is not
required.

The numeric suffix is the sponge rate (field elements absorbed per
permutation). Higher rates absorb more data per step (cheaper per byte on
long inputs) but provide proportionally less security margin;
`poseidon2` is the safest default.

```leo file=../code_snippets/standard_library/src/main.leo#std_hash_poseidon
```

Inputs may be any non-mapping, non-tuple, non-unit value.

---

## `std::commit`

A commitment to a value `x` under a randomizer `r` is a value
`c = commit(x, r)` that hides `x` (no information about `x` leaks from
`c` while `r` remains secret) and binds the committer (producing the same
`c` with a different `x'` is infeasible).

The randomizer `r` is always a `scalar`. Sample `r` uniformly at random
for every commitment; reusing a randomizer across distinct values
destroys hiding. Repeated calls with the same `(x, r)` produce the same
output, which is what makes commitments useful for later revealing or
membership checks.

```leo file=../code_snippets/standard_library/src/main.leo#std_commit_bhp
```

### Algorithms

- `std::commit::bhp256`, `bhp512`, `bhp768`, `bhp1024`: accept any
  non-mapping input.
- `std::commit::pedersen64`: `bool` and integers up to 32 bits.
- `std::commit::pedersen128`: `bool` and integers up to 64 bits.

Each algorithm exposes `commit_<input>_to_<output>(x, r)` for every
combination of input type and the three supported output types
(`address`, `field`, `group`).

---

## `std::sig`

Signature verification. Every function in this module is a `final fn`:
signatures are checked on-chain so that validators reach consensus on the
result. Off-chain authentication should be implemented at the transition
level using account secrets, not via these functions.

Each verifier returns `true` if and only if the signature is
cryptographically valid for the supplied message and signer. Replay
protection, nonce tracking, and message-uniqueness checks remain the
caller's responsibility.

### Schnorr

```leo file=../code_snippets/standard_library/src/main.leo#std_sig_schnorr
```

`verify_schnorr(sig: signature, signer: address, message: field) -> bool`
checks an Aleo Schnorr signature produced by the account at `signer`. The
wrapper accepts a `field` message; callers signing other primitive types
should hash the value into a field first (e.g. with
`std::hash::bhp256::hash_to_field`).

### ECDSA (digest)

```leo file=../code_snippets/standard_library/src/main.leo#std_sig_ecdsa
```

Two digest-style verifiers are provided:

| Function                                               | Inputs                                                                                            |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------- |
| `verify_ecdsa_digest(sig, verifying_key, prehash)`     | `sig: [u8; 65]`, `verifying_key: [u8; 33]` (compressed secp256k1 public key), `prehash: [u8; 32]` |
| `verify_ecdsa_digest_eth(sig, eth_address, prehash)`   | `sig: [u8; 65]`, `eth_address: [u8; 20]`, `prehash: [u8; 32]`                                     |

The caller is responsible for computing `prehash` with the same hash
function the signer used; this verifier does no hashing of its own. Use
the `_eth` variant when the signer is identified by their Ethereum
address (for example, signatures produced by MetaMask).

---

## `std::rand`

Pseudo-random value generation in the finalize context. Every function
here is a `final fn` and draws from a ChaCha stream cipher seeded by the
current block's pre-finalize state. The randomness is **deterministic
for a given block**: two transactions that execute the same finalize
logic against the same input observe the same sequence of values, so all
validators reach consensus.

```leo file=../code_snippets/standard_library/src/main.leo#std_rand
```

These functions are appropriate for sampling lottery winners, jittering
reward schedules, or any on-chain randomness that does not need to be
unpredictable by the block proposer. The proposer can observe the seed
and selectively withhold or reorder transactions; randomness that must
resist a malicious proposer needs a commit-reveal scheme on top.

`chacha_<type>()` is defined for every Aleo primitive return type:
`address`, `bool`, `field`, `group`, `scalar`, `u8`–`u128`, `i8`–`i128`.

---

## `std::serialize`

Bit-level serialization and deserialization of primitive types. Every
Leo primitive has a canonical bit representation that this module makes
available as a fixed-size `[bool; N]` array. The encoding round-trips:
`from_bits(to_bits(x)) == x` for any value `x` of a wrapped type, and
likewise for the `_raw` variants.

```leo file=../code_snippets/standard_library/src/main.leo#std_serialize
```

### Tagged vs. raw encoding

- `to_bits` / `from_bits` prepend a 26-bit type discriminator to the
  value's native bit representation. The discriminator identifies the
  source type, so a deserializer rejects a bit string produced for a
  different type. Use the tagged form whenever the bits will be stored,
  hashed into a commitment, or transmitted between programs (any context
  where type confusion is a security issue).
- `to_bits_raw` / `from_bits_raw` use the native bit width of the type
  with no tag. The output array is shorter, but two values of different
  types may share the same bit pattern (e.g. `8u8` and `8i8`). Use the
  raw form inside a single algorithm where the types are fixed and known
  on both sides.

### Bit widths

Native widths: `bool = 1`, `uN/iN = N`, `field = group = address = 253`,
`scalar = 251`. The tagged form adds 26 bits to each width, so
`to_bits_field` returns `[bool; 279]` and `to_bits_raw_field` returns
`[bool; 253]`.

---

## `std::grp`

Group operations on the Aleo curve. The curve's elements support
addition, scalar multiplication, and conversion to and from affine
`(x, y)` coordinates over the base field. The arithmetic operations
(`+`, `*` by a `scalar`, `.double()`, `.neg()`, `==`, ...) are available
via Leo's built-in syntax and don't need a wrapper here; this module
covers the two well-known generators, the precomputed `H` powers, and
coordinate extraction.

```leo file=../code_snippets/standard_library/src/main.leo#std_grp
```

| Function                  | Returns         | Notes                                                                                                      |
| ------------------------- | --------------- | ---------------------------------------------------------------------------------------------------------- |
| `generator()`             | `group`         | The Ed25519 base point `G`.                                                                                |
| `aleo_generator()`        | `group`         | The Aleo account-key generator `H` (runtime-resolved; not constant-foldable).                              |
| `aleo_generator_powers()` | `[group; 251]`  | Precomputed `[H, 2H, 4H, ..., 2^250 · H]` table used by Aleo's account derivation.                         |
| `to_x_coordinate(g)`      | `field`         | The affine `x` coordinate of `g`.                                                                          |
| `to_y_coordinate(g)`      | `field`         | The affine `y` coordinate of `g`.                                                                          |

---

## `std::ctx`

Execution-context accessors for the current transition. Functions in
this module surface information about the program that is executing
right now: who called it, who signed the outer transaction, what the
current block height and timestamp are, and so on.

The module is split between off-chain wrappers (plain `fn`) and on-chain
wrappers (`final fn`).

### Off-chain (usable from any transition body)

```leo file=../code_snippets/standard_library/src/main.leo#std_ctx_offchain
```

| Function    | Returns    | Description                                                                                                                 |
| ----------- | ---------- | --------------------------------------------------------------------------------------------------------------------------- |
| `addr()`    | `address`  | The Aleo address of the program this transition belongs to.                                                                 |
| `caller()`  | `address`  | The immediate caller. Equal to `signer()` when invoked directly; equal to another program's address on cross-program calls. |
| `signer()`  | `address`  | The address that signed the outer transaction; unchanged across cross-program calls.                                        |

Use `caller()` for trust decisions about who is asking for the current
operation. Use `signer()` when the decision should track the originator
of the entire transaction.

### On-chain (finalize context only)

```leo file=../code_snippets/standard_library/src/main.leo#std_ctx_onchain
```

| Function           | Returns     | Description                                                                                |
| ------------------ | ----------- | ------------------------------------------------------------------------------------------ |
| `id()`             | `address`   | On-chain identifier of this program (the value users see in block explorers).              |
| `checksum()`       | `[u8; 32]`  | 32-byte deployment checksum; changes only when the program is upgraded with new bytecode.  |
| `edition()`        | `u16`       | Deployment edition; `0` is the initial deployment, each upgrade increments it by one.      |
| `program_owner()`  | `address`   | The address that owns this program (typically the deployer).                               |
| `block_height()`   | `u32`       | Height of the block containing the current transaction.                                    |
| `block_timestamp()`| `i64`       | Unix timestamp (seconds) of the block containing the current transaction.                  |
| `network_id()`     | `u16`       | Numeric identifier of the network (mainnet, testnet, canary, etc).                         |

---

## `std::prog`

On-chain metadata accessors for **imported** programs. Each function takes the program identifier as a **const generic argument**, so the target program is fixed at compile time — the AVM has no instruction for choosing a target dynamically. Use these to gate logic on a dependency program's deployed version, checksum, or owner.

All functions in this module are `final fn`s, so they can only be called from a `final { ... }` block, a `final fn`, or a `constructor`.

```leo file=../code_snippets/standard_library/src/main.leo#std_prog
```

| Function                                         | Returns     | Description                                                                                          |
| ------------------------------------------------ | ----------- | ---------------------------------------------------------------------------------------------------- |
| `checksum::[PROG]()`                             | `[u8; 32]`  | 32-byte deployment checksum of the program `PROG`; changes only when the program is upgraded.        |
| `edition::[PROG]()`                              | `u16`       | Deployment edition of the program `PROG`; `0` is the initial deployment, each upgrade increments it. |
| `program_owner::[PROG]()`                        | `address`   | Address that owns `PROG` (typically the deployer); halts at runtime for pre-upgradability programs.  |
| `function_checksum::[PROG, FN_NAME]()`           | `[u8; 32]`  | 32-byte checksum of function `FN_NAME` inside `PROG`. Useful for pinning a dependency's function.    |

The `PROG` argument must be a program-ID literal (`foo.aleo`); `FN_NAME` must be an identifier literal (`'bar'`).
