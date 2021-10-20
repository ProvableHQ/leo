# Leo RFC 009: Conversions with Bits and Bytes

## Authors

The Aleo Team.

## Status

FINAL

## Summary

This RFC proposes the addition of natively implemented global functions to perform conversions between Leo values and sequences of bits or bytes in big endian or little endian order. This RFC also proposes a future transition from these functions to methods associated to the types.

## Motivation

Conversions to bits and bytes are fairly common in programming languages.
Use case include communication with the external world (since external data is sometimes represented as bits and bytes rather than higher-level data structures), and serialization/deserialization for cryptographic purposes (e.g. hashing data).

## Design

## Concepts

The Leo scalar values can be thought of as sequences of bits. Therefore, it makes sense to convert between values and their corresponding sequences of bits; the sequences of bits can be in little or big endian order (i.e. least vs. most significant bit first), naturally leading to two possible conversions. Obviously, the bits represent the values in base 2.

It also makes sense to convert between values and squences of bytes. Again, the bytes may be in little or big endian order.

It could also make sense to convert between integers consisting of `N` bits and sequences of "words" of `M` bits if `N` is a multiple of `M`, e.g. convert a `u32` into a sequence of two `u16`s, or convert a `u128` into a sequence of four `u32`s. However, the case in which `M` is 1 (bits) or 8 (bytes) is by far the most common, and therefore the initial focus of this RFC; nonetheless, it seems valuable to keep these possible generalizations in mind as we work though this initial design.

Another possible generalization is to lift these conversions to sequences, e.g. converting from a sequence of integer values to a sequence of bits or bytes by concatenating the results of converting the integer values, and converting from a sequence of bits or bytes to a sequence of integer values by grouping the bits or bytes into chunks and converting each chunk into an integer. For instance, a sequence of 4 `u32` values can be turned into a sequence of 32 bytes or a sequence of 128 bits. Note that, in these cases, the endianness only applies to the individual element conversion, not to the ordering of the integer values, which should be preserved by the conversion.

### Representation of Bits

In Leo's current type system, bits can be represented as `bool` values. These are not quite the numbers 0 and 1, but they are isomorphic, and it is easy to convert between booleans and bits:

```ts
// convert a boolean x to a bit:
(x ? 1 : 0)

// convert f bit y to a boolean:
(y == 1)
```

If Leo had a type `u1` for unsigned 1-bit integers, we could use that instead of `bool`. Separately from this RFC, such a type could be added. There is also an outstanding proposal (not in an RFC currently) to support types `uN` and `iN` for every positive `N`, in which case `u1` would be an instance of that.

### Representation of Bytes

The type `u8` is the natural way to represent a byte. The type `i8` is isomorphic to that, but we tend to think of bytes as unsigned.

### Representation of Sequences

This applies to the sequence of bits or bytes that a Leo integer converts to or from. E.g. a `u32` is converted to/from a sequence of bits or bytes.

Sequences in Leo may be naturally represented as arrays or tuples. Arrays are more flexible; in particular, they allow indexing via expressions rather than just numbers, unlike tuples. Thus, arrays are the natural choice to represent these sequences.

### Conversion Functions

With the implementation of the RFC for scalar type accesses we can provide bits and bytes access methods on Leo types.

For converting to bits and bytes this is represented as a call on the type itself e.g.

```ts
let bits = 1u8.to_bits_le();
```

For converting from bits and bytes this is represented as static call on a scalar type name e.g.

```ts
let u8_int = u8::from_bits([true, true, false, true, false, false, false, true]);
```

These functions are internally defined in the Leo language standard library associated with circuits. Later on during evaluation these methods are mapped to core calls. For example, the bits le functions for a u8 are defined on a u8 circuit as follows.

```ts
circuit u8 {
 function to_bits_le(self) -> [bool; 8] {
  return [false; 8];
 }

 function from_bits_le(bits: [bool; 8]) -> u8 {
  return 0u8;
 }
} 
```

Right now the blocks have dummy values defined on the Leo side. However, these values are never returned as the bodies are overwritten at evaluation time with native Rust code done from snarkVM. Note: in the future we will be able to declare function declarations without defining a body as per the native functions RFC.

The goal of this RFC is to provide the above demonstrated methods on each Leo scalar type, i.e. integers, addresses, chars, fields, and groups.

## Drawbacks

This does not seem to bring any drawbacks.

## Effect on Ecosystem

There is a future concern to consider with the above approach in that some of the types, mainly fields, chars, and groups, have a changing number of bits dependant on the curve specified. Right now Leo only supports one curve. However, in the future we will need to provide a different function declaration per curve. This would ideally be done through conditional compilation since we will know the curve during compilation time.

## Alternatives

None.
