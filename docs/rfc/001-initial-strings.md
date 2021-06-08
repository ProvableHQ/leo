# Leo RFC 001: Initial String Support

## Authors

- Max Bruce
- Collin Chin
- Alessandro Coglio
- Eric McCarthy
- Pratyush Mishra
- Jon Pavlik
- Damir Shamanaev
- Damon Sicore
- Howard Wu

## Status

IMPLEMENTED

# Summary

The purpose of this proposal is to provide initial support for strings in Leo.
Since strings are sequences of characters,
the proposal inextricably also involves characters.
This proposal is described as initial,
because it provides some basic features that we may extend in the future;
the initial features should be sufficiently simple and conservative
that they should not limit the design of the future features.

This proposal adds a new scalar type for characters,
along with a new kind of literals to denote characters.
A string is then simply an array of characters,
but this proposal also adds a new kind of literals to denote strings
more directly than via character array construction expressions.
Along with equality and inequality, which always apply to every Leo type,
this proposal also introduces some operations on characters and strings
that can be implemented over time.

By not prescribing a new type for strings,
this initial proposal leaves the door open
to a future more flexible type of resizable strings.

# Motivation

Strings (and characters) are common in programming languages.
Use cases for Leo include
simple ones like URLs and token ticker symbols,
and more complex ones like Bech32 encoding,
edit distance in strings representing proteins,
and zero-knowledge proofs of occurrences or absences of patterns in textual logs.

# Design

Since strings are sequences of characters,
a design for strings inextricably also involves a design for characters.
Thus, we first present a design for both characters and strings.

## Characters

We add a new scalar type, `char` for characters.
In accord with Leo's strong typing,
this new type is separate from all the other scalar types.
Type casts (a future feature of Leo) will be needed
to convert between `char` and other types.

The set of values of type `char` is isomorphic to
the set of Unicode code points from 0 to 10FFFF (both inclusive).
That is, we support Unicode characters, more precisely code points
(this may include some invalid code points,
but it is simpler to allow every code point in that range).
A character is an atomic entity:
there is no notion of Unicode encoding (e.g. UTF-8) that applies here.

We add a new kind of literals for characters,
consisting of single characters or escapes,
surrounded by single quotes.
Any single Unicode character except a single quote is allowed,
e.g. `'a'`, `'*'`, and `'"'`.
Single quotes must be escaped with a backslash, i.e. `'\''`;
backslashes must be escaped as well, i.e. `'\\'`
We allow other backslash escapes
for commonly used characters that are not otherwise easily denoted.
This is the complete list of single-character backslash escapes:
* `\'` for code point 39 (single quote)
* `\"` for code point 34 (double quote)
* `\\` for code point 92 (backslash)
* `\n` for code point 10 (line feed)
* `\r` for code point 13 (carriage return)
* `\t` for core point 9 (horizontal tab)
* `\0` for code point 0 (the null character)

We also allow ASCII escapes of the form `\xOH`,
where `O` is an octal digit and `H` is a hexadecimal digit.
Both uppercase and lowercase hex digits are allowed.
The `x` must be lowercase.
These represent ASCII code points, i.e. from 0 to 127 (both inclusive).

We also allow Unicode escapes of the form `'\u{X}'`,
where `X` is a sequence of one to six hex digits.
Both uppercase and lowercase letters are allowed.
The `u` must be lowercase.
The value must be between 0 and 10FFFF, inclusive.

Note that this syntax for character literals is very close to the Rust syntax documented here (as of 2021-05-26):
https://doc.rust-lang.org/reference/tokens.html#character-literals
The only difference is that this syntax does not support Unicode escapes with underbars in them.
The following is true in Rust but not in this proposal for Leo:
`'\u{1_____0__F____F______FF__________________________}' == '\u{10FFFF}'`.

Note that the literal character is assembled by the compiler---for
creating literals, there is no need for the circuit to know
which code points are disallowed.

The equality operators `==` and `!=` are automatically available for `char`.
Given that characters are essentially code points,
we may also support the ordering operators `<`, `<=`, `>`, and `>=`;
these may be useful to check whether a character is in certain range.

Below is a list of possible operations we could support on characters.
It should be fairly easy to add more.
- [ ] `is_alphabetic` - Returns `true` if the `char` has the `Alphabetic` property.
- [ ] `is_ascii` - Returns `true` if the `char` is in the `ASCII` range.
- [ ] `is_ascii_alphabetic` - Returns `true` if the `char` is in the `ASCII Alphabetic` range.
- [ ] `is_lowercase` - Returns `true` if the `char` has the `Lowercase` property.
- [ ] `is_numeric` - Returns `true` if the `char` has one of the general categories for numbers.
- [ ] `is_uppercase` - Returns `true` if the `char` has the `Uppercase` property.
- [ ] `is_whitespace` - Returns `true` if the `char` has the `White_Space` property.
- [ ] `to_digit` - Converts the `char` to the given `radix` format.
- [ ] `from_digit` - Inverse of to_digit.
- [ ] `to_uppercase` - Converts lowercase to uppercase, leaving others unchanged.
- [ ] `to_lowercase` - Converts uppercase to lowercase, leaving others unchanged.

It seems natural to convert between `char` values
and `u8` or `u16` or `u32` values, under suitable range conditions;
perhaps also between `char` values and
(non-negative) `i8` or `i16` or `i32` values.
This will be accomplished as part of the type casting extension of Leo.

The following code sample illustrates three ways of defining characters:
character literal, single-character escapes, and Unicode escapes.

```js
function main() -> [char; 5] {

    // using char literals to form an array
    const world: [char; 5] = ['w', 'o', 'r', 'l', 'd'];

    // escaped characters
    const escaped: [char; 4] = ['\n', '\t', '\\', '\''];

    // unicode escapes - using emoji character 😊
    const smiling_face: char = '\u{1F60A}';

    return [smiling_face, ...escaped];
}
```

## Strings

In this initial design proposal, we do not introduce any new type for strings.
Instead, we rely on the fact that Leo already has arrays
and that arrays of characters can be regarded as strings.
Existing array operations, such as element and range access,
apply to these strings without the need of language extensions.

To ease the common use case of writing a string value in the code,
we add a new kind of literal for strings (i.e. character arrays),
consisting of a sequence of **one or more** single characters or escapes
surrounded by double quotes;
this is just syntactic sugar for the literal array construction.
Any Unicode character except double quote or backslash is allowed without escape.
Examples: `"Aleo"`, `"it's"`, and `"x + y"`.
Double quotes must be escaped with a backslash, e.g. `"say \"hi\""`;
backslashes must be escaped as well, e.g. `"c:\\dir"`.
We also allow the same backslash escapes allowed for character literals
(see the section on characters above).
We also allow the same Unicode escapes allowed in character literals
(described in the section on characters above).

Note that this syntax for string literals is very close to the Rust syntax documented here (as of 2021-05-26):
https://doc.rust-lang.org/reference/tokens.html#string-literals.
The main difference is that this syntax does not support the Rust `STRING_CONTINUE` syntax.
In this syntax a backslash may not be followed by a newline, and newlines have no special handling.
Another differences is that this syntax does **not** permit the empty string `""`.
Also, this syntax does not allow underbars in Unicode escapes in string literals.

The type of a string literal is `[char; N]`,
where `N` is the length of the string measured in characters,
i.e. the size of the array.
Note that in this language design there is no notion of Unicode encoding (e.g. UTF-8)
that applies to string literals.

The rationale for not introducing a new type for strings initially,
and instead, piggybacking on the existing array types and operations,
is twofold.
First, it is an economical design
that lets us reuse the existing array machinery,
both at the language level (e.g. readily use array operations)
and at the R1CS compilation level
(see the section on compilation to R1CS below).
Second, it leaves the door open to providing,
in a future design iteration,
a richer type for strings,
as discussed in the section about future extensions below.

Recall that empty arrays are disallowed in Leo.
(The reason is that arrays,
which must have a size known at compile time and are not resizable,
are flattened into their elements when compiling to R1CS;
thus, an empty array would be flattened into nothing.)
Therefore, in this initial design empty strings must be disallowed as well.
A future type of resizable strings will support empty strings.

Because array, and therefore string, sizes must be known at compile time,
there is no point to having an operation to return the length of a string.
This operation will be supported for a future type of resizable strings.

Below are some examples of array operations
that are also common for strings in other programming languages:
* `[...s1, ...s2]` concatenates the strings `s1` and `s2`.
* `[c, ...s]` adds the character `c` in front of the string `s`.
* `s[i]` extracts the `i`-th character from the string `s`.
* `s[1..]` removes the first character from the string `s`.

Below is a list of possible operations we could support on strings.
It should be fairly easy to add more.
- [ ] `u8` to `[char; 2]` hexstring, .., `u128` to `[char; 32]` hexstring.
- [ ] Field element to `[char; 64]` hexstring. (Application can test leading zeros and slice them out if it needs to return, say, a 40-hex-digit string.)
- [ ] Apply `to_uppercase` (see above) to every character.
- [ ] Apply `to_lowercase` (see above) to every character.

Note that the latter two could be also realize via simple loops through the string.

Given the natural conversions between `char` values and integer values discussed earlier,
it may be natural to also support element-wise conversions between strings and arrays of integers.
This may be accomplished as part of the type casting extensions of Leo.

The following code shows a string literal and its actual transformation into an
array of characters as well as possible array-like operations on strings:
concatenation and comparison.

```js
function main() -> bool {
    // double quotes create char array from string
    let hello: [char; 5] = "hello";
    let world: [char; 5] = ['w','o','r','l','d'];

    // string concatenation can be performed using array syntax
    let hello_world: [char; 11] = [...hello, ' ', ...world];

    // string comparison is also implemented via array type
    return hello_world == "hello world";
}
```

## Format Strings

Leo currently supports format strings as their own entity,
usable exclusively as first arguments of console print calls.
This proposal eliminates this very specific notion,
which is subsumed by the string literals described above.
In other words, a console print call
will take a string literal as the first argument,
which will be interpreted as a format string
according to the semantics of console print calls.
The internal UTF-32 string will be translated to UTF-8 for output.

## Circuit Types for Character and String Operations

The operations on characters and lists described earlier, e.g. `is_ascii`,
are provided as static member functions of two new built-in or library circuit types `Char` and `String`.
Thus, an example call is `Char::is_ascii(c)`.
This seems a general good way to organize built-in or library operations,
and supports the use of the same name with different circuit types,
e.g. `Char::to_uppercase` and `String::to_uppercase`.

These circuit types could also include constants, e.g. for certain ASCII characters.
However, currently Leo does not support constants in circuit types,
so that would have to be added separately first.

These two circuit types are just meant to collect static member functions for characters and strings.
They are not meant to be the types of characters and strings:
as mentioned previously, `char` is a new scalar (not circuit) type (like `bool`, `address`, `u8`, etc.)
and there is no string type as such for now, but we use character arrays for strings.
In the future we may want all the Leo types to be circuit types of some sort,
but that is a separate feature that would have to be designed and developed independently.

## Input and Output of Literal Characters and Strings

Since UTF-8 is a standard encoding, it would make sense for
the literal characters and strings in the `.in` file
to be automatically converted to UTF-32 by the Leo compiler.
However, the size of a string can be confusing since multiple
Unicode code points can be composed into a single glyph which
then appears to be a single character.  If a parameter of type `[char; 10]`
[if that is the syntax we decide on] is passed a literal string
of a different size, the error message should explain that the
size must be the number of codepoints needed to encode the string.

## Compilation to R1CS

So far, the discussion has been independent from R1CS
(except for a brief reference when discussing the rationale behind the design).
This is intentional because the syntax and semantics of Leo
should be understandable independently from the compilation of Leo to R1CS.
However, compilation to R1CS is a critical consideration
that affects the design of Leo.
This section discusses R1CS compilation considerations
for this proposal for characters and strings.

Values of type `char` can be represented directly as field elements,
since the prime of the field is (much) larger than 10FFFF.
This is more efficient than using a bit representation of characters.
By construction, field elements that represent `char` values
are never above 10FFFF.
Note that `field` and `char` remain separate types in Leo:
it is only in the compilation to R1CS
that everything is reduced to field elements.

Since strings are just arrays of characters,
there is nothing special about compiling strings to R1CS,
compared to other types of arrays.
In particular, the machinery to infer array sizes at compile time,
necessary for the flattening to R1CS,
applies to strings without exception.
String literals are just syntactic sugar for
suitable array inline construction expressions.

There are at least two approaches to implementing
ordering operations `<` and `<=` on `char` values.
Recalling that characters are represented as field values
that are (well) below `(p-1)/2` where `p` is the prime,
we can compare two field values `x` and `y`,
both below `(p-1)/2`, via the constraints
```
(2) (x - y) = (b0 + 2*b1 + 4*b2 + ...)
(b0) (1 - b0) = 0
(b1) (1 - b1) = 0
(b2) (1 - b2) = 0
...
```
that take the difference, double it, and convert to bits.
If `x >= y`, the difference is below `(p-1)/2`,
and doubling results in an even number below `p`,
with therefore `b0 = 0`.
If `x < y`, the difference is above `(p-1)/2` (when reduced modulo `p`),
and doubling results in an odd number when reduced modulo `p`,
with therefore `b0 = 1`.
Note that we need one variable and one constraint for every bit of `p`.
The other approach is to convert the `x` and `y` to bits
and compare them as integers;
in this case we only need 21 bits for each.
We need more analysis to determine which approach is more efficient.

The details of implementing other character and string operations in R1CS
will be fleshed out as each operation is added.

## Future Extensions

As alluded to in the section about design above,
for now, we are avoiding the introduction of a string type,
isomorphic to but separate from character arrays,
because we may want to introduce later a more flexible type of strings,
in particular, one that supports resizing.
This may be realized via a built-in or library circuit type
that includes a character array and a fill index.
This may be a special case of a built-in or library circuit type
for resizable vectors,
possibly realized via an array and a fill index.
This hypothetical type of resizable vectors
may have to be parameterized over the element type,
requiring an extension of the Leo type system
that is much more general than strings.

Because of the above considerations,
it seems premature to design a string type at this time,
provided that the simple initial design described in the section above
suffices to cover the initial use cases that motivate this RFC.

# Drawbacks

This proposal does not appear to bring any real drawbacks,
other than making the language inevitably slightly more complex.
But the need to support characters and strings justifies the extra complexity.

# Effect on Ecosystem

With the ability of Leo programs to process strings,
it may be useful to have external tools that convert Leo strings
to/from common formats, e.g. UTF-8.
See the discussion of input files in the design section.

# Alternatives

We could avoid the new `char` type altogether,
and instead, rely on the existing `u32` to represent Unicode code points,
and provide character-oriented operations on `u32` values.
(Note that both `u8` and `u16` are too small for 10FFFF,
and that signed integer types include negative integers
which are not Unicode code points:
this makes `u32` the obvious choice.)
However, many values of type `u32` are above 10FFFF,
and many operations on `u32` do not really make sense on code points.
We would probably want a notation for character literals anyhow,
which could be (arguably mis)used for non-character unsigned integers.
All in all, introducing a new type for characters
is consistent with Leo's strong typing approach.
Furthermore, for compilation to R1CS, `u32`,
even if restricted to the number of bits needed for Unicode code points,
is less efficient than the field representation described earlier
because `u32` requires a field element for each bit.

Instead of representing strings as character arrays,
we could introduce a new type `string`
whose values are finite sequences of zero or more characters.
These strings would be isomorphic to, but distinct form, character arrays.
However, for compilation to R1CS, it would be necessary to
perform the same kind of known-size analysis on strings
that is already performed on arrays,
possibly necessitating to include size as part of the type, i.e. `string(N)`,
which is obviously isomorphic to `[char; N]`.
Thus, using character arrays avoids duplication.
Furthermore, as noted in the section on future extensions,
this leaves the door open to
introducing a future type `string` for resizable strings.

Yet another option could be to use directly `field` to represent characters
and `[field; N]` to represent strings of `N` characters.
However, many values of type `field` are not valid Unicode code points,
and many field operations do not make sense for characters.
Thus, having a separate type `char` for characters seems better,
and more in accordance with Leo's strong typing.
