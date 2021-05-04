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

DRAFT

# Summary

The purpose of this proposal is to provide initial support for strings in Leo.
Since strings are sequences of characters,
the proposal inextricably also involves characters.
This proposal is described as 'initial'
because it provides some basic features that we may extend in the future;
the initial features should be sufficiently simple and conservative
that they should not limit the design of the future features.

This proposal adds a new scalar type for characters
along with a new kind of literals to denote characters.
A string is then simply as an array of characters,
but this proposal also adds a new kind of literals to denote strings
more directly than via character array construction expressions.
Along with equality and inequality, which always apply to every Leo type,
this proposal also introduces operations for
_[TODO: Summarize initial set of built-in or library operations
on characters and strings.]_.

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
_[TODO: Add more use cases if needed.]_

# Design

Since strings are sequences of characters,
a design for strings inextricably also involves a design for characters.
Thus, we first present a design for characters, then for strings.
After that, we discuss the relation with Leo's existing format strings.
We conclude this design section
with a discussion of possible future extensions.

## Characters

We add a new scalar type `char` for characters.
In accord with Leo's strong typing,
this new type is separate from all the other scalar types.

The set of values of type `char` is isomorphic to
the set of Unicode code points from 0 to 10FFFFh (both inclusive).
That is, we support Unicode characters, more precisely code points
(this may include some invalid code points,
but it is simpler to allow every code point in that range).
A character is an atomic entity:
there is no notion of Unicode encoding (e.g. UTF-8) that applies here.

We add a new kind of literals for characters,
consisting of single characters or escapes,
surrounded by single quotes.
Any single Unicode character except single quote is allowed,
e.g. `'a'`, `'*'`, and `'"'`.
Single quotes must be escaped with backslash, i.e. `'\''`;
backslashes must be escaped as well, i.e. `'\\'`
We allow other backslash escapes
for commonly used characters that are not otherwise easily denoted,
namely _[TODO: Decide which other escapes we want to allow, e.g. `'\n'`.]_
We also allow Unicode escapes of the form `'\u{X}'`,
where `X` is a sequence of one or more hex digits
(both uppercase and lowercase letters are allowed)
whose value must be between 0 and 10FFFFh.
Note that the literal character is assembled by the compiler---for
creating literals there is no need for the circuit to know
which codepoints are disallowed.
_[TODO: Do we want a different notation for Unicode escapes?
Note that the `{` `}` delimiters are motivated by the fact that
there may be a varying number of hex digits in this notation.]_

_[TODO: Which (initial) built-in or library operations
do we want to provide for `char` values?]_

## Strings

In this initial design proposal, we do not introduce any new type for strings.
Instead, we rely on the fact that Leo already has arrays,
and that arrays of characters can be regarded as strings.
Existing array operations, such as element and range access,
apply to these strings without the need of language extensions.

To ease the common use case of writing a string value in the code,
we add a new kind of literal for strings (i.e. character arrays),
consisting of a sequence of one or more single characters or escapes
surrounded by double quotes;
this is just syntactic sugar.
Any single Unicode character except double quote is allowed,
e.g. `""`, `"Aleo"`, `"it's"`, and `"x + y"`.
Double quotes must be escaped with backslash, e.g. `"say \"hi\""`;
backslashes must be escaped as well, e.g. `"c:\\dir"`.
We allow the same backslash escapes allowed for character literals
(see the section on characters above).
_[TODO: There is a difference in the treatment of single and double quotes:
the former are allowed in string literals but not character literals,
while the latter are allowed in character literals but not string literals;
this asymmetry is also present in Java.
However, for simplicity we may want to symmetrically disallow
both single and double quotes in both character and string literals.]_
We also allow the same Unicode escapes allowed in character literals,
(described in the section on characters above).
In any case, the type of a string literal is `[char; N]`,
where `N` is the length of the string measured in characters,
i.e. the size of the array.
Note that there is no notion of Unicode encoding (e.g. UTF-8)
that applies to string literals.

The rationale for not introducing a new type for strings initially,
and instead piggyback on the existing array types and operations,
is twofold.
First, it is an economical design
that lets us reuse the existing array machinery,
both at the language level (e.g. readily use array operations)
and at the R1CS compilation level
(see the section on compilation to R1CS below).
Second, it leaves the door open to providing,
in a future design iteration,
a richer type for strings,
as disccused in the section about future extensions below.

_[TODO: Which (initial) built-in or library operations
do we want to provide for `[char; N]` values that are not already
available with the existing array operations?]_
* u8 to [char; 2] hexstring, .., u128 to [char; 32] hexstring
* field element to [char; 64] hexstring. (Application can test leading zeros and slice them out if it needs to return, say, a 40-hex-digit string)
* [TODO: more?]

## Input and Output of Literal Characters and Strings

Since UTF-8 is a standard encoding, it would make sense for
the literal characters and strings in the .in file
to be automatically converted to UTF-32 by the Leo compiler.
However, the size of a string can be confusing, since multiple
Unicode code points can be composed into a single glyph which
then appears to be a single character.  If a parameter of type `[char; 10]`
[if that is the syntax we decide on] is passed a literal string 
of a different size, the error message should explain that the 
size must be the number of codepoints needed to encode the string.

## Format Strings

Leo currently supports format strings as their own entity,
usable exclusively as first arguments of console print calls.
This proposal eliminates this very specific notion,
which is subsumed by the string literals described above.
In other words, a console print call
will simply take a string literal as first argument,
which will be interpreted as a format string
according to the semantics of console print calls.
The internal UTF-32 string will be translated to UTF-8 for output.

## Compilation to R1CS

So far the discussion has been independent from R1CS
(except for a brief reference when discussing the rationale behind the design).
This is intentional, because the syntax and semantics of Leo
should be understandable independently from the compilation of Leo to R1CS.
However, compilation to R1CS is a critical consideration
that affects the design of Leo.
This section discusses R1CS compilation considerations
for this proposal for characters and strings.

Values of type `char` can be represented directly as field elements,
since the prime of the field is (much) larger than 10FFFFh.
This is more efficient than using a bit representation of characters.
By construction, field elements that represent `char` values
are never above 10FFFFh.
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

## Future Extensions

As alluded to in the section about design above,
for now we are avoiding the introduction of a string type,
isomorphic to but separate from character arrays,
because we may want to introduce later a more flexible type of strings,
in particular one that supports resizing.
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

# Alternatives

We could avoid the new `char` type altogether,
and instead rely on the existing `u32` to represent Unicode code points,
and provide character-oriented operations on `u32` values.
(Note that both `u8` and `u16` are too small for 10FFFFh,
and that signed integer types include negative integers
which are not Unicode code points:
this makes `u32` the obvious choice.)
However, many values of type `u32` are above 10FFFFh,
and many operations on `u32` do not really make sense on code points.
We would probably want a notation for character literals anyhow,
which could be (arguably mis)used for non-character unsigned integers.
All in all, introducing a new type for characters
is consistent with Leo's strong typing approach.
Furthermore, for compilation to R1CS, `u32`,
even if restricted to the number of bits needed for Unicode code points,
is less efficient than the field representation described earlier,
because `u32` requires a field element for each bit.

Instead of representing strings as character arrays,
we could introduce a new type `string`
whose values are finite sequences of zero or more characters.
These strings would be isomorphic to, but distinct form, character arrays.
However, for compilation to R1CS, it would be necessary to
perform the same kind of known-size analysis on strings
that is already performed on arrays,
possibly necessitating to include a size as part of the type, i.e. `string(N)`,
which is obviously isomorphic to `[char; N]`.
Thus, using character arrays avoids the duplication.
Furthermore, as noted in the section on future extensions,
this leaves the door open to
introducing a future type `string` for resizable strings.

Yet another option could be to use directly `field` to represent characters
and `[field; N]` to represent strings of `N` characters.
However, many values of type `field` are not valid Unicode code points,
and many field operations do not make sense for characters.
Thus, having a separate type `char` for characters seems better,
and more in accordance with Leo's strong typing.
