# Leo RFC 009: Conversions with Bits and Bytes

## Authors

The Aleo Team.

## Status

FINAL

## Summary

This RFC proposes the addition of natively implemented scalar type methods to perform conversions
between Leo scalar values and sequences of bits or bytes.

## Motivation

Conversions of values to sequences of bits and bytes are fairly common in programming languages.
Use cases include communication with the external world
(since external data is often represented as bits and bytes rather than higher-level data structures),
and serialization/deserialization for cryptographic purposes (e.g. hashing data).

## Design

### Bits and Bit Sequences

The conversions represent:
* Bits as booleans, i.e. values of type `bool`.
* Bit sequences as arrays of booleans, i.e. values of types `[bool; N]` for suitable values of `N`.

### Bytes and Byte Sequences

The conversions represent:
* Bytes as 8-bit unsigned integers, i.e. values of type `u8`.
* Byte sequences as arrays of 8-bit unsigned integers, i.e. values of types `[u8; M]` for suitable values of `M`.

### Conversions to Bits and Bytes

Conversions from values of scalar type `T` to (sequences of) bits and bytes
are realized as instance methods associated to `T` of the form
```ts
function to_bits_le(self) -> [bool; N];
function to_bits_be(self) -> [bool; N];
function to_bytes_le(self) -> [u8; M];
function to_bytes_be(self) -> [u8; M];
```
where `le` stands for 'little endian', `be` stands for 'big endian',
and the values of `N` and `M` are described below for each `T`, along with the exact behavior of each conversion.

Thus, if `t` is an expression of type `T`:
* The expression `t.to_bits_le()` has type `[bool; N]`.
* The expression `t.to_bits_be()` has type `[bool; N]`.
* The expression `t.to_bytes_le()` has type `[u8; M]`.
* The expression `t.to_bytes_be()` has type `[u8; M]`.

These instance methods are provided by the Leo standard library.
They have no bodies, because they are implemented natively (in Rust), not in Leo.

### Conversions from Bits and Bytes

Conversions from (sequences of) bits and bytes to values of scalar type `T`
are realized as static methods associated to `T` of the form
```ts
function from_bits_le(bits: [bool; N]) -> T;
function from_bits_be(bits: [bool; N]) -> T;
function from_bytes_le(bytes: [u8; M]) -> T;
function from_bytes_be(bytes: [u8; M]) -> T;
```
where `le` stands for 'little endian', `be` stands for 'big endian',
and the values of `N` and `M` are described below for each `T`, along with the exact behavior of each conversion.

Thus, if `bits` is an expression of type `[bool; N]` and `bytes` is an expression of type `[u8; M]`:
* The expression `T::from_bits_le(bits)` has type `T`.
* The expression `T::from_bits_be(bits)` has type `T`.
* The expression `T::from_bytes_le(bytes)` has type `T`.
* The expression `T::from_bytes_be(bytes)` has type `T`.

These static methods are provided by the Leo standard library.
They have no bodies, because they are implemented natively (in Rust), not in Leo.

### Conversions with Integers

The Leo integer values have a natural representation as bits:
* Unsigned integer values of type `uN`, with `N` in {8, 16, 32, 64, 128},
  can be viewed as sequences of `N` bits,
  according to the usual positional representation in base 2.
* Signed integer values of type `iN`, with `N` in {8, 16, 32, 64, 128},
  can be viewed as sequences of `N` bits,
  according to the usual two's complement representation.

The bit sequences can be ordered in little or big endian,
based on whether the first bit in the sequence is the most or least significant one
(for signed integers, that is the sign bit).

Every integer of type `uN` or `iN` can be converted to a (little or big endian) sequence of `N` bits.
Every (little or big endian) sequence of `N` bits can be converted to an integer of type `uN` or `iN`.
These conversions are always well-defined.

Examples:
```ts
// unsigned:
let x: u8 = 1;
let xle: [bool; 8] = x.to_bits_le();
let xbe: [bool; 8] = x.to_bits_be();
console.assert(xle == [true, false, false, false, false, false, false, false]);
console.assert(xbe == [false, false, false, false, false, false, false, true]);
console.assert(x == u8::from_bits_le(xle));
console.assert(x == u8::from_bits_be(xbe));
// signed:
let y: i8 = -128;
let yle: [bool; 8] = y.to_bits_le();
let ybe: [bool; 8] = y.to_bits_be();
console.assert(yle == [false, false, false, false, false, false, false, true]);
console.assert(ybe == [true, false, false, false, false, false, false, false]);
console.assert(y == i8::from_bits_le(yle));
console.assert(y == i8::from_bits_be(ybe));
```

Because the five values of `N` above are all multiples of 8,
we can group `N` bits into `M` chunks of 8 bits each, i.e. `M` bytes, so that:
* Unsigned integer values of type `uN`, with `N` in {8, 16, 32, 64, 128},
  can be viewed as sequences of `M` bytes, with `M` in {1, 2, 4, 8, 16}.
* Signed integer values of type `iN`, with `N` in {8, 16, 32, 64, 128},
  can be viewed as sequences of `M` bytes, with `M` in {1, 2, 4, 8, 16}.

The byte sequences can be ordered in little or big endian,
based on whether the first byte in the sequence contains the 8 most or least significant bits
(for signed integers, the most significant bit is the sign bit).

Every integer of type `uN` or `iN` can be converted to a (little or big endian) sequence of `M = N/8` bytes.
Every (little or big endian) sequence of `M = N/8` bytes can be converted to an integer of type `uN` or `iN`.
These conversions are always well-defined.

Examples:
```ts
// unsigned:
let x: u32 = 10;
let xle: [u8; 4] = x.to_bytes_le();
let xbe: [u8; 4] = x.to_bytes_be();
console.assert(xle == [10, 0, 0, 0]);
console.assert(xbe == [0, 0, 0, 10]);
console.assert(x == u8::from_bytes_le(xle));
console.assert(x == u8::from_bytes_be(xbe));
// signed:
let y: i32 = -2147483648;
let yle: [u8; 4] = y.to_bytes_le();
let ybe: [u8; 4] = y.to_bytes_be();
console.assert(yle == [0, 0, 0, 128]);
console.assert(ybe == [128, 0, 0, 0]);
console.assert(y == u8::from_bytes_le(yle));
console.assert(y == u8::from_bytes_be(ybe));
```

Conversions between `u8` and bytes amount to
converting between the integer and the singleton array that contains the integer;
in this case, there is no difference between little and big endian order.
These conversions are available for completeness, but may not be very useful in practice.

Conversions between `i8` and bytes amount to
converting between the intger and the singleton array that contains the integer re-interpreted as unsigned;
in this case, there is no different between little and big endian order.
Thus, these conversions may be used to re-interpret between signed and unsigned bytes,
but arguably there should be more dedicated and comprehensive operations for that in Leo:
that is an independent extension of Leo,
which may make the conversions between `i8` and bytes just available for completeness, but not very useful in practice.

### Conversions with Field Elements

A field element is a non-negative integer less than the prime number that defines the field.
If the prime number is `N` bits long (i.e. it is represented in binary form as `N` bits `1xxx...xxx`),
then a field element can be viewed as an `N`-bit unsigned integer,
leading to natural conversions to `N` bits in little or big endian.

Every field element can be converted to a (little or big endian) sequence of `N` bits,
but not all (little or big endian) sequences of `N` bits represent a field element:
they may represent an `N`-bit unsigned integer that is greater than or equal to the prime number;
such integers must exist because the prime number cannot be a power of 2,
and therefore at least the sequence of `N` one bits `1...1` is not a field element.
Attempting to convert these bit sequences to field elements causes an error,
in the same sense as division by zero causes an error.

The value of `N` depends on the choice of elliptic curve.
Currently Leo supports one elliptic curve (Edwards BLS12), leading to a fixed value of `N`, but that will change:
we will independently extend Leo with mechanisms to handle different elliptic curves,
which will lead to different values of `N` for field element conversions.
For the currently supported curve, `N` is 253.

Since `N` is not a multiple of 8,
the conversions between field elements and byte sequences are defined
by adding 3 most significant zero bits to the bit sequence prior to grouping it into 8-bit chunks,
leading to `M = 32` bytes, where the 3 high bits of the most significant byte are always 0.

Every field element can be converted to a (little or big endian) sequence of `M` bytes,
but, similarly to the conversions from bits above,
attempting to convert some sequences of `M` bytes to a field element causes an error,
precisely when the numeric value is greater than or equal to the prime number.

Examples:
```ts
let x:field = 3;
console.assert(x.to_bits_le() == [true, true, ...[false; 251]]);
console.assert(x.to_bits_be() == [...[false; 251], true, true]);
console.assert(x.to_bytes_le() == [3, ...[0; 31]];
console.assert(x.to_bytes_be() == [...[0; 31], 3];
let y:field = field::from_bits_le([true; 253]); // error
let y:field = field::from_bits_be([true; 253]); // error
let y:field = field::from_bytes_le([255; 32]); // error
let y:field = field::from_bytes_be([255; 32]); // error
```

### Conversions with Group Elements

A group element consists of two field elements that satisfy the elliptic curve equation,
i.e. a group element is a point on the curve on the plane defined by the cartesian product of the field with itself.
The point consists of an _x_ and a _y_ coordinate.

A group element is converted to a sequence of `N` bits,
where `N` is twice the bit length of the prime number that defines the field,
by juxtaposing the `N/2` bits obtained from converting the _x_ and _y_ coordinates as field elements.
More precisely:
* The conversion of a group element (_x_, _y_) to big endian bits is `[...x, ...y]`,
  where `x` is the conversion of _x_ to big endian bits and `y` is the conversion of _y_ to big endian bits.
* The conversion of a group element (_x_, _y_) to little endian bits is `[...y, ...x]`,
  where `x` is the conversion of _x_ to little endian bits and `y` is the conversion of _y_ to little endian bits.

Reversing the order of the coordinates for little endian means that
the little endian conversion of a group element is the reverse of the big endian conversion of the same group element,
a property shared with the integer and field element conversions.
However, given that a group element is not a single number,
the notion of big and little endian as such does not directly apply to group elements,
but only to their individual coordinates.

Not all (little or big endian) sequences of `N` bits represent group elements.
Not only each `N/2`-bit half must represent a field element (see discussion for field elements above),
but also the resulting point (_x_, _y_) must satisfy the curve equation.
Attempting to convert to a group element a sequence of `N` bits that does not actually represent a group element
causes an error, in the same sense as division by zero causes an error.

Conversions with byte sequences are handled by juxtaposition of the byte sequence representations of _x_ and _y_:
* The conversion of a group element (_x_, _y_) to big endian bytes is `[...x, ...y]`,
  where `x` is the conversion of _x_ to big endian bytes and `y` is the conversion of _y_ to big endian bytes.
* The conversion of a group element (_x_, _y_) to little endian bytes is `[...y, ...x]`,
  where `x` is the conversion of _x_ to little endian bytes and `y` is the conversion of _y_ to little endian bytes.

For the currently supported elliptic curve in Leo, `N` is 506, and `M` is 64.
These valuee will change with the curve, when Leo is independently extended with support for more curves,
as discussed earlier for field elements.

### Conversions with Characters

A character is a Unicode code point, i.e. an integer between 0 and 10FFFFh.
Thus, it can be viewed as a 21-bit unsigned integers,
with bit conversions defined in a natural way,
and byte coversions defined by adding 3 most significant zero bits to reach 24 bits, which is 3 bytes.

However, currently characters are represented as field elements under the hood,
and this causes the conversions to use the same number of bits and bytes as the field conversions,
namely `N` is 253 and `M` is 32.

A character can be always converted to a (little or big endian) sequence of bits or bytes.
However, analogously to field and group elements,
attempting to convert to character a sequence whose numeric value exceeds 10FFFFh
causes an error in Leo.
This is the case also if `N` and `M` are reduced to be 21 and 3.

```ts
let x: char = 'A';
console.assert(x.to_bits_le() == [true, false, false, false, false, false, true, false, ...[false; 245]];
console.assert(x.to_bits_be() == [...[false; 245], false, true, false, false, false, false, false, true];
console.assert(x.to_bytes_le() == [65, ...[0; 31]];
console.assert(x.to_bytes_be() == [...[0; 31], 65];
let y: char = char::from_bits_le([true; 21]); // error
let y: char = char::from_bytes_le([255, 255, 255]); // error
```

### Conversions with Booleans

Converting between booleans and boolean sequences amount to
converting between the boolean and the singleton array that contains the boolean;
in this case, there is no difference between little and big endian order.
These conversions are available for completeness, but may not be very useful in practice.

```ts
console.assert(false.to_bits_le() == [false]);
console.assert(false.to_bits_be() == [false]);
console.assert(bool::from_bits_le([true]) == true);
console.assert(bool::from_bits_be([true]) == true);
```

### Conversions with Addresses

A Leo address is a sequence of 63 lowecase letters and decimal digits that starts with `aleo1`.
In this form, it is not a number or combination of numbers of any sort.

However, as documented elsewhere,
an address is essentially a public key that consists of the _x_ coordinate of a curve point.
The `aleo1...` sequence of 63 characters can be derived from that,
and the value of the _x_ coordinate can be derived from a (properly formed) `aleo1...` sequence of 63 characters.

Thus, an address can be treated in the same way as a field element for the purpose of bit/byte conversions.
This is what the bit/byte conversion functions for addresses do in Leo.

### Implementation Considerations

These conversions are internally implemented as member functions of _pseudo_ circuit types for the scalar types, e.g.
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
These are not real circuit types, because scalar types are disjoint from circuit types;
it is just an internal representation.

Currently the bodies return dummy values.
However, these dummy values are never returned,
because the bodies are overwritten at evaluation time with native Rust code done from snarkVM.
In the future we will be able to declare functions without a defining a body, as per the native functions RFC.

## Drawbacks

This does not seem to bring any drawbacks.

## Effect on Ecosystem

None.

## Alternatives

### Pure Leo Implementation

These conversions can be realized in Leo (i.e. without native implementations),
provided that Leo is extended with certain operations that are already separately planned:
* Integer division and remainder, along with type casts, could be used.
* Bitwise shifts and masks, along with type casts, could be used.

However, compiling the Leo code that realizes the conversions may result in less efficient R1CS than the native ones.

### Representation of Bits

Bits are normally thought of as the numbers 0 or 1 (i.e. binary digits), rather than the booleans `true` and `false`.
However, booleans are isomorphic to {0, 1}, and it is easy to convert between them:
```ts
// convert a boolean x to a bit:
(x ? 1 : 0)

// convert a bit y to a boolean:
(y == 1)
```

An alternative is to use `u8` values 0 and 1 for bits, or any other integer type in fact.
However, the conversions from bits would have to check that each integer is 0 or 1, since the type alone would not guarantee that.
Furthermore, Leo types `uN` and `iN` are compiled to `N` boolean-constrained field elements in R1CS, resulting in wasted constraints.
Given all of this, `bool` is the most appropriate representational choice for bits.

### Representation of Bytes

Since `u8` and `i8` are isomorphic, bytes could be represented as `i8` values instead of `u8` values.
However, we tend to think of bytes, in the sense of sequences of 8 bits,
as unsigned, numbered from 0 to 255 according to binary notation.

### Representation of Sequences

Leo provides two aggregate types whose components are organized as a sequence: tuples and arrays.
So an alternative is to represent bit and byte sequences as tuples instead of arrays.
However, arrays are much more flexible, and have elements of the same type unlike tuples.
Empty arrays are disallowed, but empty arrays are not needed for the conversions proposed in this RFC.
On the other hand, 1-tuples are (currently) disallowed in Leo, which would be problematic for some of the conversions.

### Shorter Conversions with Characters

As mentioned earlier, 21 bits and 3 bytes should suffice for characters.
This requires support on the snarkVM side.

### Shorter Conversions with Group Elements

Given the fact that group elements are only a subset of all the possible pairs of field elements,
we could use shorter bit and byte sequences.
In particular, for the Edwards BLS12 curve, just the x coordinate suffices.

## Future Extensions

### Interaction with Fine-Grained Integer Types

There is an independent proposal to extend Leo with fine-grained integer types `uN` and `iN`
for any positive integer value of `N`.

If that extension is done, `u1`, whose values are exactly 0 and 1, could be used for bits instead of `bool`.
The two types are isomorphic, but `u1` can be more readily used in arithmetic.

Also, if that extension is done, conversions to/from bits extend easily to all values of `N`.
For values of `N` that are not multiples of 8,
conversions to/from bytes can be handled by adding 0 bits as done for field elements (see above),
but that introduces the possibility that conversions may fail when the numeric value in the bytes is too large.
This is not a problem, as it can be handled as discussed above.

### Conversions with Words

Besides bits and bytes, words of different bit sizes are used in certain contexts.
There is no standardized terminology
(in fact, even bytes are not completely standard, which is why the term 'octet' is sometimes used),
but one can think of `P`-bit words for any positive integer value of `P`.
In this light, bits are 1-bit words and bytes are 8-bit words.

It may make sense to introduce conversions from Leo scalar types to words of different sizes besides bits and bytes.
Words consisting of `P` bits could be represented as values of type `uP`, provided it is a supported type;
note that the fine-grained integer types mentioned above would support words of every size.

If `N` is a multiple of `P`, then integers of `uN` or `iN` types
can be converted to/from sequences (i.e. arrays) of type `uP`.
Examples include converting a `u32` value to an array of two `u16` values, in big or little endian.
A regular naming scheme should be used for these conversions.

If `N` is not a multiple of `P`, it is still possible to convert to/from sequences of `P`-bit words,
but the conversions from words will need to handle error conditions.

### Conversions of Aggregate Values

Besides scalar types, Leo has aggregate types:
tuple types, array types, and circuit types.
It may make sense to add conversions between aggregate values and bit/byte/word sequences,
where an aggregate value is represented as a concatenation of
bit/byte/word sequences that represent the components (recursively).
In order for the conversions from bit/byte/word sequences to work,
variable-sized components must be encoded with some indication of their size,
e.g. using TLV (type-length-value or tag-length-value) approaches.
