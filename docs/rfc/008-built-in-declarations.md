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

## Summary

This RFC proposes a framework for making certain (top-level) declarations (e.g. type aliases) available in every Leo program without the need to write those declarations explicitly. These may be hardwired into the language or provided by standard libraries/packages; in the latter case, the libraries may be implicitly imported or required to be explicitly imported.

## Motivation

It is common for programming languages to provide predefined types, functions, etc.
that can be readily used in programs. The initial motivation for this in Leo was to have a type alias `string` for character arrays of unspecified sizes (array types of unspecified sizes and type aliases are discussed in separate RFCs), but the feature is clearly more general.

## Design

Leo supports four kinds of top-level declarations:

- Import declarations.
- Function declarations.
- Circuit type declarations.
- Global constant declarations.
- Type alias declarations. (Proposed in a separate RFC.)

Leaving import declarations aside for the moment since they are "meta" in some sense
(as they bring in names of entities declared elsewhere),
it may make sense for any of the four kinds of declarations above to have built-in instances, i.e., we could have some built-in functions, circuit types, global constants, and type aliases. These features are why this RFC talks of built-in declarations, more broadly than just built-in type aliases that inspired it.

The built-in status of the envisioned declarations will be done through explicitly declared core library files. Then these core library files must be explicitly imported. This way helps avoid unnecessary code bloat in the compilation, and any user asked for AST snapshots.

## Drawbacks

This does not seem to bring any drawbacks.

## Effect on Ecosystem

This change may interact with libraries and packages in some way,
if we go with case 2 or 3 above.
But it should not be much different from standard libraries/packages.

## Alternatives

Some alternative approaches are:

1. Their names could be simply available in any program,
   without any explicit declaration found anywhere for them.
2. They could be declared in some core library files explicitly
 and be available in any program without explicitly importing them,
 like `java.lang.String` in Java or `std::Option` in Rust.

From a user's perspective, there is not a lot of difference between cases 1 and 2 above:
in both cases, the names are available; the only difference is that in case 2, the user can see the declaration somewhere.

Also, note that case 2 could be seen as having an implicit (i.e., built-in) import of the library/libraries in question. Again, imports are "meta" in this context, and what counts are the other kinds of declarations.

In cases 2 and the decided upon design choice, a related but somewhat independent issue is whether those declarations have Leo definitions or not. The Leo library already includes functions like BLAKE2s that are not defined in Leo but rather "natively" in Rust/R1CS though some of this may be subject to change for native definitions(see the separate RFC on those).
