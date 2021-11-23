# Leo RFC 008: Built-in Declarations

## Authors

The Aleo Team.

## Status

IMPLEMENTED

## Summary

This RFC proposes a framework for making certain (top-level) declarations (e.g. type aliases) available in every Leo program without the need to write those declarations explicitly. These may be hardwired into the language or provided by standard libraries/packages; in the latter case, the libraries may be implicitly imported or required to be explicitly imported.

## Motivation

It is common for programming languages to provide predefined types, functions, etc.
that can be readily used in programs. The initial motivation for this in Leo was to have a type alias `string` for character arrays of unspecified sizes (array types of unspecified sizes and type aliases are discussed in separate RFCs), but the feature is clearly more general.

## Design

Leo supports five kinds of top-level declarations:

- Import declarations.
- Function declarations.
- Struct type declarations.
- Global constant declarations.
- Type alias declarations. (Proposed in a separate RFC.)

Leaving import declarations aside for the moment since they are "meta" in some sense
(as they bring in names of entities declared elsewhere),
it may make sense for any of the four kinds of declarations above to have built-in instances, i.e., we could have some built-in functions, struct types, global constants, and type aliases. These features are why this RFC talks of built-in declarations, more broadly than just built-in type aliases that inspired it.

The built-in status of the envisioned declarations will be done through explicitly declared standard library(stdlib) files. Then these stdlib files must expressly be imported, except the files found in stdlib/prelude/*. The ones found in the prelude are features determined to be helpful enough in standard programs and are auto-imported.

## Drawbacks

This does not seem to bring any drawbacks.

## Effect on Ecosystem

This change may interact with libraries and packages in some way.
But it should not be much different from standard libraries/packages.

## Alternatives

Some alternative approaches are:

1. Having all stdlib imports auto included.
2. Require that all stdlib imports are explicitly imported.

The differences between the two above approaches and the chosen one are just how many imports are imported explicitly.
