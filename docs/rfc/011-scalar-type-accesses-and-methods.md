# Leo RFC 011: Scalar Type Accesses And Methods

## Authors

The Aleo Team.

## Status

FINAL

## Summary

This RFC proposes three things:

1. The scalar types in Leo (integers, fields, etc.) can have static methods.
2. The scalar types in Leo (integers, fields, etc.) can have static constants.
3. Expressions of scalar type can have instance methods called on them.

## Motivation

This approach allows for a clean interface to provide built-in methods or static members for these basic types.

## Design

### Semantics

Firstly we would have to modify both the ABNF and parsing of Leo to allow static method calls onto a scalar type.

The ABNF would look as follows:

```abnf
; This is an existing old rule.
scalar-type =  boolean-type / arithmetic-type / address-type / character-type

; Add this rule.
named-type = identifier / self-type / scalar-type

; Modify this rule.
postfix-expression = primary-expression
                   / postfix-expression "." natural
                   / postfix-expression "." identifier
                   / identifier function-arguments
                   / postfix-expression "." identifier function-arguments
                   / named-type "::" identifier function-arguments ; this used to be identifier-or-self-type
                   / named-type "::" identifier ; this is new to allow member constants
                   / postfix-expression "[" expression "]"
                   / postfix-expression "[" [expression] ".." [expression] "]"

; Also need to add a new static member variable declaration rule to allow for static constant members.
member-constant-declaration = %s"static" %s"const" identifier ":" type = literal ";"

; We then need to modify the struct declaration rule.
struct-declaration = %s"struct" identifier
                      "{" *member-constant-declaration
                      [ member-variable-declarations ]
                      *member-function-declaration "}"
```

Now methods and static members would be first-class citizens of scalar types and their values. For example, the following could be done:

```ts
let x = 1u8.to_bits(); // A method call on on a scalar value itself
let x = u8::MAX; // A constant value on the scalar type
let y = u8::to_bits(1u8); // A static method on the scalar type
```

It also allows for static constants for structs in general:

```ts
struct Point {
  static SLOPE: u32 = 3;
  x: u32;
  y: u32;

  function new(x: u32, y: u32) -> Self {
    return Self {
      x,
      y: y + Self::SLOPE
    };
  }
}
```

## Drawbacks

This change adds more complexity to the language.

## Effect on Ecosystem

None. The new parsing changes would not break any older programs.

## Alternatives

None.
