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

#### [IntegerType](./src/types/integer_type.rs)

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

### [Statements](./src/statements/statement.rs)

The statement level nodes in a Leo Program.

#### [Assignment Statements](./src/statements/assign/)

An assignment statement node stores the following:

- The operation.
  - **=**
  - **+=**
  - **-=**
  - **=**
  - **/=**
  - **=**
  - **&&=**
  - **||=**
- The assignee which is a variable that has context of any access expressions on it.
- The value which is an expression.

#### [Block Statements](./src/statements/block.rs)

A block statement node stores the following:

- The list of statements inside the block.

#### [Conditional Statements](./src/statements/conditional.rs)

A conditional statement node stores the following:

- The condition which is an expression.
- The block statement.
- The next block of the conditional if it exists.

#### [Console Statements](./src/statements/)

A console statement node stores the following:

- The console function being called which stores the type of console function it is and its arguments.

#### [Definition Statements](./src/statements/definition/mod.rs)

A definition statement node stores the following:

- The declaration type:
  - `let` for mutable definitions.
  - `const` for cosntant definitions.
- The names of the varaibles defined.
- The optional type.
- The values to be assigned to the varaibles.

#### [Expression Statements](./src/statements/expression.rs)

An expression statement node stores the following:

- The expression.

#### [Iteration Statements](./src/statements/iteration.rs)

A iteration statement node stores the following:

- The loop iterator variable name.
- The expression to define the starting loop value.
- The expression to define the stoping loop value.
- The block to run for the loop.

#### [Return Statements](./src/statements/return_statement.rs)

A return statement node stores the following:

- The expression that is being returned.

### Expressions

The expression nodes in a Leo Program.

#### [ArrayAccess Expressions](./src/expression/array_acces.rs)

An array access expression node stores the following:

- The array expression.
- The index represented by an expression.

#### [ArrayInit Expressions](./src/expression/array_init.rs)

An array init expression node stores the following:

- The element expression to fill the array with.
- The dimensions of the array to build.

#### [ArrayInline Expressions](./src/expression/array_inline.rs)

An array inline expression node stores the following:

- The elments of an array which is either an spread or an expression.

#### [ArrayRangeAccess Expressions](./src/expression/array_range_access.rs)

An array range access expression node stores the following:

- The array expression.
- The optional left side of the range of the array bounds to access.
- The optional right side of the range of the array bounds to access.

#### [Binary Expressions](./src/expression/binary.rs)

A binary expression node stores the following:

- The left side of the expression.
- The right side of the expression.
- The binary operation of the expression:
  - **+**
  - **-**
  - **\***
  - **/**
  - **\*\***
  - **||**
  - **&&**
  - **==**
  - **!=**
  - **>=**
  - **>**
  - **<=**
  - **<**

#### [Call Expressions](./src/expression/call.rs)

A call expression node stores the following:

- The function expression being called.
- The aruments a list of expressions.

#### [CircuitInit Expressions](./src/expression/circuit_init.rs)

A circuit init expression node stores the following:

- The name of the circuit expression being initialized.
- The aruments a list of expressions.

#### [CircuitMemberAccess Expressions](./src/expression/circuit_member_access.rs)

A circuit member access expression node stores the following:

- The circut expression being accessed.
- The name of the expression being accessed from the circuit.

#### [CircuitStaticFunctionAccess Expressions](./src/expression/circuit_static_function_access.rs)

A circuit static function access expression node stores the following:

- The circut expression being accessed.
- The name of the expression being statically accessed from the circuit.

#### [Identifier Expressions](./src/common/identifier.rs)

An identifer expression node stores the following:

- An identifier stores the string name.

#### [Ternary Expressions](./src/expression/ternary.rs)

A ternary expression node stores the following:

- The condition of the ternary stored as an expression.
- The expression returned if the condition is true.
- The expression returned if the condition is false.

#### [TupleAccess Expressions](./src/expression/tuple_access.rs)

A tuple access expression node stores the following:

- The tuple expression being accessed.
- The index a positive number greater than or equal to 0.

#### [TupleInit Expressions](./src/expression/tuple_init.rs)

A tuple init expression node stores the following:

- The element expressions to fill the tuple with.

#### [Unary Expressions](./src/expression/unary.rs)

An unary expression node stores the following:

- The inner expression.
- The unary operator:
  - **!**
  - **-**

#### [Value Expressions](./src/expression/value.rs)

A value expression node stores one of the following:

- Address and its value and span.
- Boolean and its value and span.
- Char and its value and span.
- Field and its value and span.
- Group and its value and span.
- Implicit and its value and span.
- Integer and its value and span.
- String and its value and span.
