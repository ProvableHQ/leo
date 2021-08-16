# Leo RFC 009: Conversions with Bits and Bytes

## Authors

- Max Bruce
- Collin Chin
- Alessandro Coglio
- Eric McCarthy
- Jon Pavlik
- Damir Shamanaev
- Damon Sicore
- Howard Wu

## Status

DRAFT

# Summary

This RFC proposes the addition of natively implemented global functions to perform conversions
between Leo integer values and sequences of bits or bytes in big endian or little endian order.
This RFC also proposes a future transition from these functions to methods associated to the integer types.

# Motivation

Conversions of integers to bits and bytes are fairly common in programming languages.
Use case include communication with the external world
(since external data is sometimes represented as bits and bytes rather than higher-level data structures),
and serialization/deserialization for cryptographic purposes (e.g. hashing data).

# Design

## Concepts

The Leo integer values can be thought of sequences of bits.
Therefore, it makes sense to convert between integer values and their corresponding sequences of bits;
the sequences of bits can be in little or big endian order (i.e. least vs. most significant bit first),
naturally leading to two possible conversions.
Obviously, the bits represent the integers in base 2.

Since all the Leo integer values consist of multiples of 8 bits,
it also makes sense to convert between integer values and squences of bytes,
which represents the integers in base 256.
Again, the bytes may be in little or big endian order.

It could also make sense to convert between integers consisting of `N` bits
and sequences of "words" of `M` bits if `N` is a multiple of `M`,
e.g. convert a `u32` into a sequence of two `u16`s, or convert a `u128` into a sequence of four `u32`s.
However, the case in which `M` is 1 (bits) or 8 (bytes) is by far the most common,
and therefore the initial focus of this RFC;
nonetheless, it seems valuable to keep these possible generalizations in mind as we work though this initial design.

Another possible generalization is to lift these conversions to sequences,
e.g. converting from a sequence of integer values to a sequence of bits or bytes
by concatenating the results of converting the integer values,
and converting from a sequence of bits or bytes to a sequence of integer values
by grouping the bits or bytes into chunks and converting each chunk into an integer.
For instance, a sequence of 4 `u32` values can be turned into a sequence of 32 bytes or a sequence of 128 bits.
Note that, in these cases, the endianness only applies to the individual element conversion,
not to the ordering of the integer values, which should be preserved by the conversion.

Besides integers, it could make sense to consider converting other Leo values between bits and bytes,
namely characters, field elements, group elements, and addresses (but perhaps not booleans).
If this is further extended to aggregate values (tuples, arrays, and circuits),
then this moves towards a general serialization/deserialization library for Leo, which could be a separate feature.

## Representation of Bits

In Leo's current type system, bits can be represented as `bool` values.
These are not quite the numbers 0 and 1, but they are isomorphic, and it is easy to convert between booleans and bits:
```ts
// convert a boolean x to a bit:
(x ? 1 : 0)

// convert f bit y to a boolean:
(y == 1)
```

If Leo had a type `u1` for unsigned 1-bit integers, we could use that instead of `bool`.
Separately from this RFC, such a type could be added.
There is also an outstanding proposal (not in an RFC currently) to support types `uN` and `iN` for every positive `N`,
in which case `u1` would be an instance of that.

## Representation of Bytes

The type `u8` is the natural way to represent a byte.
The type `i8` is isomorphic to that, but we tend to think of bytes as unsigned.

## Representation of Sequences

This applies to the sequence of bits or bytes that a Leo integer converts to or from.
E.g. a `u32` is converted to/from a sequence of bits or bytes.

Sequences in Leo may be ntaurally represented as arrays or tuples.
Arrays are more flexible; in particular, they allow indexing via expressions rather than just numbers, unlike tuples.
Thus, arrays are the natural choice to represent these sequences.

## Conversion Functions

We propose the following global functions,
for which we write declarations without bodies below,
since the implementation is native.
(It is a separate issue whether the syntax below should be allowed,
in order to represent natively implemented functions,
or whether there should be a more explicit indication such as `native` in Java).

These are tentative names, which we can tweak.
What is more important is the selection of operations, and their input/output types.

### Conversions between Integers and Bits

```ts
// unsigned to bits, little and big endian
function u8_to_bits_le(x: u8) -> [bool; 8];
function u8_to_bits_be(x: u8) -> [bool; 8];
function u16_to_bits_le(x: u16) -> [bool; 16];
function u16_to_bits_be(x: u16) -> [bool; 16];
function u32_to_bits_le(x: u32) -> [bool; 32];
function u32_to_bits_be(x: u32) -> [bool; 32];
function u64_to_bits_le(x: u64) -> [bool; 64];
function u64_to_bits_be(x: u64) -> [bool; 64];
function u128_to_bits_le(x: u128) -> [bool; 128];
function u128_to_bits_be(x: u128) -> [bool; 128];

// signed to bits, little and big endian
function i8_to_bits_le(x: i8) -> [bool; 8];
function i8_to_bits_be(x: i8) -> [bool; 8];
function i16_to_bits_le(x: i16) -> [bool; 16];
function i16_to_bits_be(x: i16) -> [bool; 16];
function i32_to_bits_le(x: i32) -> [bool; 32];
function i32_to_bits_be(x: i32) -> [bool; 32];
function i64_to_bits_le(x: i64) -> [bool; 64];
function i64_to_bits_be(x: i64) -> [bool; 64];
function i128_to_bits_le(x: i128) -> [bool; 128];
function i128_to_bits_be(x: i128) -> [bool; 128];

// unsigned from bits, little and big endian
function u8_from_bits_le(x: [bool; 8]) -> u8;
function u8_from_bits_be(x: [bool; 8]) -> u8;
function u16_from_bits_le(x: [bool; 16]) -> u16;
function u16_from_bits_be(x: [bool; 16]) -> u16;
function u32_from_bits_le(x: [bool; 32]) -> u32;
function u32_from_bits_be(x: [bool; 32]) -> u32;
function u64_from_bits_le(x: [bool; 64]) -> u64;
function u64_from_bits_be(x: [bool; 64]) -> u64;
function u128_from_bits_le(x: [bool; 128]) -> u128;
function u128_from_bits_be(x: [bool; 128]) -> u128;

// signed from bits, little and big endian
function i8_from_bits_le(x: [bool; 8]) -> i8;
function i8_from_bits_be(x: [bool; 8]) -> i8;
function i16_from_bits_le(x: [bool; 16]) -> i16;
function i16_from_bits_be(x: [bool; 16]) -> i16;
function i32_from_bits_le(x: [bool; 32]) -> i32;
function i32_from_bits_be(x: [bool; 32]) -> i32;
function i64_from_bits_le(x: [bool; 64]) -> i64;
function i64_from_bits_be(x: [bool; 64]) -> i64;
function i128_from_bits_le(x: [bool; 128]) -> i128;
function i128_from_bits_be(x: [bool; 128]) -> i128;
```

### Conversions between Integers and Bytes

```ts
// unsigned to bytes, little and big endian
function u16_to_bytes_le(x: u16) -> [u8; 2];
function u16_to_bytes_be(x: u16) -> [u8; 2];
function u32_to_bytes_le(x: u32) -> [u8; 4];
function u32_to_bytes_be(x: u32) -> [u8; 4];
function u64_to_bytes_le(x: u64) -> [u8; 8];
function u64_to_bytes_be(x: u64) -> [u8; 8];
function u128_to_bytes_le(x: u128) -> [u8; 16];
function u128_to_bytes_be(x: u128) -> [u8; 16];

// signed to bytes, little and big endian
function i16_to_bytes_le(x: i16) -> [u8; 2];
function i16_to_bytes_be(x: i16) -> [u8; 2];
function i32_to_bytes_le(x: i32) -> [u8; 4];
function i32_to_bytes_be(x: i32) -> [u8; 4];
function i64_to_bytes_le(x: i64) -> [u8; 8];
function i64_to_bytes_be(x: i64) -> [u8; 8];
function i128_to_bytes_le(x: i128) -> [u8; 16];
function i128_to_bytes_be(x: i128) -> [u8; 16];

// unsigned from bytes, little and big endian
function u16_from_bytes_le(x: [u8; 2]) -> u16;
function u16_from_bytes_be(x: [u8; 2]) -> u16;
function u32_from_bytes_le(x: [u8; 4]) -> u32;
function u32_from_bytes_be(x: [u8; 4]) -> u32;
function u64_from_bytes_le(x: [u8; 8]) -> u64;
function u64_from_bytes_be(x: [u8; 8]) -> u64;
function u128_from_bytes_le(x: [u8; 16]) -> u128;
function u128_from_bytes_be(x: [u8; 16]) -> u128;

// signed from bytes, little and big endian
function i16_from_bytes_le(x: [u8; 2]) -> i16;
function i16_from_bytes_be(x: [u8; 2]) -> i16;
function i32_from_bytes_le(x: [u8; 4]) -> i32;
function i32_from_bytes_be(x: [u8; 4]) -> i32;
function i64_from_bytes_le(x: [u8; 8]) -> i64;
function i64_from_bytes_be(x: [u8; 8]) -> i64;
function i128_from_bytes_le(x: [u8; 16]) -> i128;
function i128_from_bytes_be(x: [u8; 16]) -> i128;
```

## Handling of the Native Functions

Given the relatively large number and regular structure of the functions above,
it makes sense to generate them programmatically (e.g. via Rust macros),
rather than enumerating all of them explicitly in the implementation.
It may also makes sense, at R1CS generation time,
to use generated or suitably parameterized code to recognize them and turn them into the corresponding gadgets.

## Transition to Methods

Once a separate proposal for adding methods to Leo scalar types is realized,
we may want to turn the global functions listed above into methods,
deprecating the global functions, and eventually eliminating them.

Conversions to bits or bytes will be instance methods of the integer types,
e.g. `u8` will include an instance method `to_bits_le` that takes no arguments and that returns a `[bool; 8]`.
Example:
```ts
let int: u8 = 12;
let bits: [bool; 8] = int.to_bits_le();
console.assert(bits == [false, false, true, true, false, false, false, false]); // 00110000 (little endian)
```

Conversions from bits or bytes will be static methods of the integer types,
e.g. `u8` will include a static metod `from_bits_le` that takes a `[bool; 8]` argument and returns a `u8`.
Example:
```ts
let bits: [bool; 8] = [false, false, true, true, false, false, false, false]; // 00110000 (little endian)
let int = u8::from_bits_le(bits);
console.assert(int == 12);
```

# Drawbacks

This does not seem to bring any drawbacks.

# Effect on Ecosystem

None.

# Alternatives

## Pure Leo Implementation

These conversions can be realized in Leo (i.e. without native implementations),
provided that Leo is extended with certain operations that are already separately planned:
* Integer division and remainder, along with type casts, could be used.
* Bitwise shifts and masks, along with type casts, could be used.

However, compiling the Leo code that realizes the conversions may result in less efficient R1CS than the native ones.

## Naming Bit and Byte Types Explicitly

Names like `u8_to_bits_le` and `u32_to_bytes_le` talk about bits and bytes,
therefore relying on a choice of representation for bits and bytes,
which is `bool` for bits and `u8` for bytes as explained above.
An alternative is to have names like `u8_to_bools_le` and `u32_to_u8s_le`,
which explicate the representation of bits and bytes in the name,
and open the door to additional conversions to different representations.
In particular, if and when Leo is extended with a type `u1` for bits,
there could be additional operations like `u8_to_u1s_le`.

This more explicit naming scheme also provides a path towards extending
bit and byte conversions to more generic "word" conversions,
such as `u64_to_u16s_le`, which would turn a `u64` into a `[u16; 4]`.
In general, it makes sense to convert between `uN` or `iN` and `[uM; P]` when `N == M * P`.
If Leo were extended with types `uN` and `iN` for all positive `N` as proposed elsewhere,
there could be a family of all such conversions.

## Methods Directly

Given that we eventually plan to use methods on scalar types for these conversions,
it may make sense to do that right away.
This is predicated on having support for methods on scalar types,
for which a separate RFC is in the works.

If we decide for this approach, we will revise the above proposal to reflect that.
The concepts and (essential) names and input/output types remain unchanged,
but the conversions are packaged in slightly different form.
