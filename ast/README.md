# leo-ast
[![Crates.io](https://img.shields.io/crates/v/leo-ast.svg?color=neon)](https://crates.io/crates/leo-ast)
[![Authors](https://img.shields.io/badge/authors-Aleo-orange.svg)](../AUTHORS)
[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](./LICENSE.md)

This directory contains the code for the AST of a Leo Program.

## Node Types

There are several types of nodes in the AST that then have further breakdowns.

All nodes store a Span, which is useful for tracking the lines and
columns of where the node was taken from in the Leo Program.

### [Program/File](./src/program.rs)

The top level nodes in a Leo Program.

#### [Imports](./src/imports/import.rs)

Represents an import statement in a Leo Program.
A list of these are stored on the Program.
It stores the path to an import and what is being imported.

**NOTE**: The import does not contain the source code of the imported Leo Program.

#### [Circuits](./src/circuits/circuit.rs)

A circuit node represents a defined Circuit in a Leo Program.
A order preserving map of these are stored on the Program.
Contains the Circuit's name, as well as it's members.
The members are a function, or a variable.
For both of them the Circuit preserves their names.

#### [Decorators](./src/annotation.rs)

An annotation node is a decorator that can be applied to a function.
Stored on the function themselves despite being a top-level node.
The node stores the name of the annotation, as well as any args passed to it.

#### [Functions](./src/functions/function.rs)

A function node represents a defined function in a Leo Program.
A order preserving map of these are stored on the Program.
A function node stores the following information:

- The annotations applied to the function.
- An identifier the name of the function.
- The inputs to the function, both their names and types.
- The output of the function as a type if it exists.
- The function body stored as a block statement.

#### [Global Consts](./src/program.rs)

A global const is a bit special and has no special node for itself, but rather is a definition statement.
A order preserving map of these are stored on the Program.

### [Types](./src/types/type_.rs)

The different types in a Leo Program.
Types themselves are not a node, but rather just information to be stored on a node.

#### Address

The address type follows the [BIP_0173](https://en.bitcoin.it/wiki/BIP_0173) format starting with `aleo1`.

#### Boolean

The boolean type consists of two values **true** and **false**.

#### Char

The char type resents a character from the inclusive range [0, 10FFFF].

#### Field

The field type an unsigned number up to the modulus length of the field.

#### Group

The group type a set of affine points on the elliptic curve passed.

#### IntegerType

The integer type represents a range of integer types.

##### U8

A integer in the inclusive range [0, 255].

##### U16

A integer in the inclusive range [0, 65535].

##### U32

A integer in the inclusive range [0, 4294967295].

##### U64

A integer in the inclusive range [0, 18446744073709551615].

##### U128

A integer in the inclusive range [0, 340282366920938463463374607431768211455].

##### I8

A integer in the inclusive range [-128, 127].

##### I16

A integer in the inclusive range [-32768, 32767].

##### I32

A integer in the inclusive range [-2147483648, 2147483647].

##### I64

A integer in the inclusive range [-9223372036854775808, 9223372036854775807].

##### I128

A integer in the inclusive range [-170141183460469231731687303715884105728, 170141183460469231731687303715884105727].

#### Array

The array type contains another type, then the number of elements of that type greater than 0.

#### Tuple

The tuple type contains n types, where n is greater than or equal to 0.

#### Circuit

The circuit type, every circuit represents a different type.

#### SelfType

The self type represented by `Self` and only usable inside a circuit.

### Statements

The statement level nodes in a Leo Program.

#### Assignment Statements

#### Block Statements

#### Conditional Statements

#### Console Statements

#### Definition Statements

#### Expression Statements

#### Iteration Statements

#### Return Statements

### Expressions

The expression nodes in a Leo Program.

#### ArrayAccess Expressions

#### ArrayInit Expressions

#### ArrayInline Expressions

#### ArrayRangeAccess Expressions

#### Binary Expressions

#### Call Expressions

#### CircuitInit Expressions

#### CircuitMemberAccess Expressions

#### CircuitStaticFunctionAccess Expressions

#### Identifier Expressions

#### Ternary Expressions

#### TupleAccess Expressions

#### TupleInit Expressions

#### Unary Expressions

#### Value Expressions

