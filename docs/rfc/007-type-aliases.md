# Leo RFC 007: Type Aliases

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

This RFC proposes the addition of type aliases to Leo,
i.e. identifiers that abbreviate types and can be used wherever the latter can be used.
A new top-level construct is proposed to define type aliases; no circularities are allowed.
Type aliases are expanded away during compilation.

# Motivation

Many programming languages provide the ability to create aliases (i.e. synonyms) of types, such as C's `typedef`.
The purpose may be to abbreviate a longer type,
such as an alias `matrix` for `[i32; (3, 3)]` in an application in which 3x3 matrices of 32-bit integers are relevant
(e.g. for 3-D rotations, even though fractional numbers may be more realistic).
The purpose may also be to clarify the purpose and use of an existing type,
such as an alias `balance` for `u64` in an application that keeps track of balances.

The initial motivation that inspired this RFC (along with other RFCs)
was the ability to have a type `string` for strings.
Strings are arrays of characters according to RFC 001.
With the array types of unspecified size proposed in RFC 006,
`[char; _]` becomes a generic type for strings, which is desirable to alias with `string`.

# Design

The ABNF grammar changes as follows:
```
; modified rule:
keyword = ...
        / %s"true"
        / %s"type" ; new
        / %s"u8"
        / ...

; new rule:
type-alias-declaration = %s"type" identifier "=" type ";"

; modified rule:
declaration = import-declaration
            / function-declaration
            / circuit-declaration
            / constant-declaration
            / type-alias-declaration ; new
```

A type alias declaration introduces the identifier to stand for the type.
Only top-level type alias declarations are supported;
they are not supported inside functions or circuit types.

There must be no direct or indirect circularity in the type aliases.
That is, it must be possible to expand all the type aliases away,
obtaining an equivalent program without any type aliases.

Note that the built-in `Self` is a bit like a type alias, standing for the enclosing circuit type;
and `Self` is replaced with the enclosing circuit type during canonicalization.
Thus, canonicalization could be a natural place to expand user-defined type aliases;
after all, type aliases introduce multiple ways to denote the same types
(and not just via direct aliasing, but also via indirect aliasing, or via aliasing of components),
and canonicalization serves exactly to reduce multiple ways to say the same thing to one canonical way.

On the other hand, expanding type aliases is more complicated than the current canonicalization transformations,
which are all local and relatively simple.
Expanding type aliases requires not only checking for circularities,
but also to take into account references to type aliases from import declarations.
For this reason, we may perform type alias expansion after canonicalization,
such as just before type checking and inference.
We could also make the expansion a part of the type checking and inference process,
which already transforms the program by inferring missing types,
so it could also expand type aliases away.

In any case, it seems beneficial to expand type aliases away
(whether during canonicalization or as part or preamble to type checking and inference)
prior to performing more processing of the program for eventual compilation to R1CS.

# Drawbacks

As other extensions of the language, this makes things inherently a bit more complicated.

# Effect on Ecosystem

None; this is just a convenience for the Leo developer.

# Alternatives

An alternative to creating a type alias
```
type T = U;
```
is to create a circuit type
```
circuit T { get: U }
```
that contains a single member variable.

This is clearly not equivalent to a type alias, because it involves conversions between `T` and `U`
```
T { get: u } // convert u:U to T
t.get // convert t:T to U
```
whereas a type alias involves no conversions:
if `T` is an alias of `U`, then `T` and `U` are the same type,
more precisely two syntactically different ways to designate the same semantic type.

While the conversions generally cause overhead in traditional programming languages,
this may not be the case for Leo's compilation to R1CS,
in which everything is flattened, including member variables of circuit types.
Thus, it may be the case that the circuit `T` above reduces to just its member `U` in R1CS.

It might also be argued that wrapping a type into a one-member-variable circuit type
could be a better practice than aliasing the type, to enforce better type separation and safety.

We need to consider the pros and cons of the two approaches,
particularly in light of Leo's non-traditional compilation target.
