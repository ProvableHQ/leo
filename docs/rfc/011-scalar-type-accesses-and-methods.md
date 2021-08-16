# Leo RFC 011: Scalar Type Accesses And Methods

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

This RFC proposes two things:

1. The scalar types in Leo (integers, fields, etc.) can have static methods.
2. The scalar types in Leo (integers, fields, etc.) can have static constants.
3. Those values that have a scalar type can have methods directly on them.

## Motivation

This approach allows for a clean interface to provide built-in methods or static members for these basic types.

## Design

### Semantics

Firstly we would have to modify both the ABNF and parsing of Leo to allow static method calls onto a scalar type.

The ABNF would look as follows:

```abnf
; This is an existing old rule
scalar-type =  boolean-type / arithmetic-type / address-type / character-type

; This is an existing old rule
circuit-type = identifier / self-type

; Add this rule.
named-type = circuit-type / scalar-type ; new rule

; Modify this rule:
postfix-expression = primary-expression
                   / postfix-expression "." natural
                   / postfix-expression "." identifier
                   / identifier function-arguments
                   / postfix-expression "." identifier function-arguments
                   / named-type "::" identifier function-arguments ; this used to be a circuit-type
                   / named-type "::" identifier ; this is new to allow static members on 
                   / postfix-expression "[" expression "]"
                   / postfix-expression "[" [expression] ".." [expression] "]"
```

Now methods and static members would be first-class citizens of scalar types and their values. For example, the following could be done:

```ts
let x = 1u8.to_bits(); // A method call on on a scalar value itself
let x = u8::MAX; // A constant value on the scalar type
let y = u8::to_bits(1u8); // A static method on the scalar type
```

## Drawbacks

This change adds more complexity to the language.

## Effect on Ecosystem

None. The new parsing changes would not break any older programs.

## Alternatives

None.
