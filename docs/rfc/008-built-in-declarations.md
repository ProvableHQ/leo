# Leo RFC 008: Built-in Declarations

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

This RFC proposes a framework for making certain (top-level) declarations (e.g. type aliases)
available in every Leo program without the need to explicitly write those declarations.
These may be hardwired into the language, or provided by standard libraries/packages;
in the latter case, the libraries may be either implicitly imported or required to be explicitly imported.

# Motivation

It is common for programming languages to provide predefined types, functions, etc.
that can be readily used in programs.
The initial motivation for this in Leo was to have a type alias `string` for character arrays of unspecified sizes
(array types of unspecified sizes and type aliases are discussed in separate RFCs),
but the feature is clearly more general.

# Design

Leo supports four kinds of top-level declarations:
- Import declarations.
- Function declarations.
- Circuit type declarations.
- Global constant declarations.
- Type alias declarations. (Proposed in a separate RFC.)

Leaving import declarations aside for the moment since they are "meta" in some sense
(as they bring in names of entities declared elsewhere),
it may make sense for any of the four kinds of declarations above to have built-in instances,
i.e. we could have some built-in functions, circuit types, global constants, and type aliases.
This is why this RFC talks of built-in declarations, more broadly than just built-in type aliases that inspired it.

The built-in status of the envisioned declarations could be achieved in slightly different ways:
1. Their names could be simply available in any program,
   without any explicit declaration found anywhere for them.
2. They could be declared in some core library files explicitly,
   and be available in any program without needing to be explicitly import them,
   like `java.lang.String` in Java or `std::Option` in Rust.
3. They could be declared in some core library files explicitly,
   and be available only in programs that explicitly import them.

From a user's perspective, there is not a lot of difference between cases 1 and 2 above:
in both cases, the names are available; the only difference is that in case 2 the user can see the declaration somewhere.

Also note that case 2 could be seen as having an implicit (i.e. built-in) import of the library/libraries in question.
Again, imports are "meta" in this context, and what counts are really the other kinds of declarations.

In cases 2 and 3, a related but somewhat independent issue is whether those declarations have Leo definitions or not.
The Leo library already includes functions like the one for BLAKE2s that are not defined in Leo,
but rather "natively" in Rust/R1CS.

# Drawbacks

This does not seem to bring any drawbacks.

# Effect on Ecosystem

This may interact with libraries and packages in some way,
if we go with case 2 or 3 above.
But it should be not much different from regular libraries/packages.

# Alternatives

The 'Design' section above currently discusses a few alternatives,
rather than prescribing a defined approach.
When consensus is reached on one of the alternatives discussed there,
the others will be moved to this section.
