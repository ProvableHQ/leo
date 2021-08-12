# Leo RFC 001: Initial String Support

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

This RFC proposes that the primitive types in Leo (integers, fields, etc.) are treated as circuits. The actual type primitives would still exist within the compiler and language. However, whenever a type primitive is encountered, it would be treated as a circuit(similar to Java types being objects). Note that this would not affect circuit synthesis.

## Motivation

Several languages have their types represented by an object within the language itself. This approach allows for a clean interface to provide built-in methods or static members for these basic types.

## Design

### Semantics

Leo already has support for circuits. As such, the change would occur mainly during the parsing stage. However, we would still validate the parsing of literals/tuple-expressions/array-expressions the same way; the resulting type would just be a circuit. When we encounter either implicit types or an explicitly typed system, they would be parsed as a circuit.

We could implement the circuits internally as follows:

```ts
circuit U8 {
  data: u8
}
```

As for AST clarity's sake, these circuits should still be written to the AST the same way as the current primitives are.

It would benefit us to store these circuits in a separate list of circuits on the AST that are pre-defined.

All current operations on these circuits representing the basic functions should still work. These operations can easily be implemented by just leveraging the actual type primitive stored within the circuit.

Now methods and static members would be first-class citizens of the circuits treated similarly to current operations like existing primitive operators. For example, in the following:

```ts
let x = 1u8.to_bits();
```

the method call would be treated as a gadget call. This implementation would save us implementing Leo methods the same way as blake2s currently is.

## Drawbacks

This change adds more complexity to the language.

## Effect on Ecosystem

None. All currently valid Leo programs would still be valid after this proposal.

## Alternatives

For implementing built-in operators, we could also allow operator overloading and then have a default overloaded method on these circuits.
