# Leo RFC 013: Constant Functions

## Author(s)

The Aleo Team.

## Status

IMPLEMENTED

## Summary

This RFC proposes the additional of an optional `const` modifier to function declarations
to explicitly designate functions that return constant values that can be calculated at compile time.

## Motivation

Explicitly designating constant functions makes user intention explicit
and simplifies the Leo compiler's checks, as explained below.

## Background

Leo code is partially evaluated on its `const` inputs prior to being translated to R1CS.
A function that returns a value that only depends on `const` inputs directly or indirectly,
can be partially evaluated away, without having to be inlined during flattening.

## Design

### Syntax

The ABNF grammar is extended by adding an optional `const` modifier to function declarations:
```
function-declaration = *annotation [ %s"const" ] %s"function" identifier
                       "(" [ function-parameters ] ")" [ "->" type ]
                       block
```

This applies to both top-level and member functions.

### Static Semantics

A `const` function declaration must satisfy the following conditions:
* All its parameters are `const`, including the `self` parameter for instance circuit member functions.
* The body does not reference the special `input` variable.
* The body only calls other `const` functions.

### Dynamic Semantics

This has no impact on the dynamic semantics of Leo, viewed as a traditional programming language.

### Flattening

Given that `const` expressions are evaluated completely during flattening,
the values of the arguments of a `const` function call are known during flattening,
and therefore the function call can be completely evaluated as well.

If the function is recursive (individually or with others),
the evaluation involves the bounded recursion analysis described in a separate RFC.

### Implementation Considerations

ASTs for function declarations are extended with a boolean flag `const_`.

If a `const` function has a non-`const` parameter,
an AST error occurs.

If the body of a `const` function references the `input` variable or calls a non-`const` function,
an ASG error occurs.

The description of static semantics, dynamic semantics, and flattening given above
are expressed in terms of Leo, because that is the user's view of the language.
In the implementation, flattening occurs after the Leo code is translated to the IR.

### Examples

```ts
const function len(const arr: [u8; _]) -> u32 {
    return arr.len();
}

circuit Sample {
    x: [char; 5]
    const function say_hi(const self) -> [char; 5] {
        return self.x;
    }
}
```

## Drawbacks

This extension does not appear to bring any drawbacks.

## Effect on Ecosystem

None.

## Alternatives

### No Constant Designation

Without an explicit designation of constant functions,
the Leo compiler needs to perform an inter-procedural analysis:
if `f` calls `g`, in order for `f` to be constant, also `g` must be constant.
In other words, the call graph must be taken into account.

In contrast, with the `const` designation,
an intra-procedural analysis suffices,
as discussed in the static semantics section above.

## Future Extensions

In other languages like Rust, `const` functions are not required to have all constant parameters.
They are just required to return constant results for constant arguments,
i.e. they must not access global variables and they must only call other `const` functions.
In other words, these `const` functions are polymorphic over "constancy".

This could be also realized in Leo, because type inference/checking determines `const` and non-`const` expressions.
This tells the compiler which function calls have all `const` arguments and which ones do not.
Therefore, the compiler can selectively evaluate, during flattening, only the calls of `const` functions on `const` arguments.
