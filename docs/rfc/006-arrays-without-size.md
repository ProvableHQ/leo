# Leo RFC 006: Array Types with Unspecified Size

## Authors

The Aleo Team.

## Status

DRAFT

# Summary

This RFC proposes the addition, at the user level, of array types with unspecified size,
of the form `[T, _]`, where `T` is the element type and the underscore stands for an unspecified size.
It must be possible to infer the size at compile time.

When these types are used in a function,
different calls of the function (which are inlined) may resolve the sizes of these types to different values.

To make this extension more useful, this RFC also proposes the addition of
an operator to return the length of an array, whose result is resolved at compile time.

# Motivation

The initial motivation was the ability to have a type `string` for Leo strings,
which are currently represented as character arrays,
therefore requiring a size indication, i.e. `[char; <size>]`.
Allowing a `[char; _]` type, where `_` stands for an unspecified size,
makes it possible to define a type alias `string` for it,
once we also have an (orthogonal) extension of Leo to support type aliases.

However, allowing `[T; _]` for any `T` (not just `char`) is a more generally useful feature.
This kind of types is already used internally in the Leo compiler.
Allowing their use externally should provide additional convenience to the user.
Some examples are shown in the 'Design' section.

# Design

User-facing array types currently have a specified size, indicated as a positive integer according to the grammar and static semantics
(the grammar allows 0, via the `natural` rule, but the static semantics checks that the natural is not 0).
Internally, the Leo compiler uses array types with unspecified size in some cases, e.g. when taking a slice with non-literal bounds.
These internal unspecified sizes must be resolved at compile time (by evaluating the constant bound expressions), in order for compilation to succeed.

This RFC proposes to make array types with unspecified size available at the user level,
with the same requirement that their sizes must be resolved in order for compilation to succeed.

The ABNF grammar changes as follows:
```
; new rule:
array-dimension = natural / "_"

; modified rule:
array-dimensions = array-dimension
                 / "(" array-dimension *( "," array-dimension ) ")"
```
That is, an array dimension may be unspecified; this is also the case for multidimensional array types.

Note that `array-dimension` is also referenced in this rule of the ABNF grammar:
```
; existing rule:
array-repeat-construction = "[" expression ";" array-dimensions "]"
```
The compiler will enforce, post-parsing, that array dimensions in array repeat expressions are positive integers, i.e. non-zero naturals.
This will be part of the static semantics of Leo.

Array types may appear, either directly or within other types, in the following constructs:
- Constant declarations, global or local to functions.
- Variable declarations, local to functions.
- Function inputs.
- Function outputs.
- Member variable declarations.

Thus, those are also the places where array types with unspecified size may occur.

An array type with unspecified size that occurs in a global constant declaration must be resolved to a unique size.
On the other hand, an array type with unspecified size that occurs in a function
(whether a variable declaration, function input, or function output)
could be resolved to different sizes for different inlined calls of the function.
Finally, there seems to be no point in allowing array types of unspecified sizes in member variable declarations:
the circuit type must be completely known, including the types of its member variables;
therefore, this RFC prescribes that array types with unspecified size be disallowed in member variable declarations.
(This may be revisited if a good use case, and procedure for resolution, comes up.)

## Examples

In the following example, the array type with unspecified size obviates the need to explicate the size (3),
since it can be resolved by the compiler:
```
let x: [u8; _] = [1, 2, 3];
```
Currently it is possible to omit the type of `x` altogether of course,
but then at least one of the elements must have a type suffix, e.g. `1u8`.

Using an array type of unspecified size for a function input makes the function generic over the size:
```
function f(x: [u8; _]) ...
```
That is, `f` can take an array of `u8` of any size, and perform some generic computation on it,
because different inlined calls of `f` may resolve the size to different values (at compile time).
But this brings up the issue discussed below.

## Array Size Operator

Currently Leo has no array size operator, which makes sense because arrays have known sizes.
However, if we allow array types with unspecified size as explained above,
we may also need to extend Leo with an array size operator.

However, consider a function `f` as above, which takes as input an array of `u8` of unspecified size.
In order to do something with the array, e.g. add all its elements and return the sum,
`f` should be able to access the size of the array.

Thus, this RFC also proposed to extend Leo with such an operator.
A possibility is `<expression>.length`, where `<expression>` is an expression of array type.
A variation is `<expression>.len()`, if we want it look more like a built-in method on arrays.
Yet another option is `length(<expression>)`, which is more like a built-in function.
A shorter name could be `len`, leading to the three possibilities
`<expression>.len`, `<expression>.len()`, and `len(<expression>)`.
So one dimension of the choice is the name (`length` vs. `len`),
and another dimension is the style:
member variable style,
member function style,
or global function style.
The decision on the latter should be driven by broader considerations
of how we want to treat this kind of built-in operators.

Note that the result of this operator can, and in fact must, be calculated at compile time;
not as part of the Leo interpreter, but rather as part of the flattening of Leo to R1CS.
In other words, this is really a compile-time operator, akin to `sizeof` in C.

With that operator, the following function can be written:
```
function f(x: [u8; _]) -> u8 {
    let sum = 0u8;
    for i in 0..length(x) {
        sum += x[i];
    }
    return sum;
}
```

# Drawbacks

None, aside from inevitably making the language and compiler slightly more complex.

# Effect on Ecosystem

None.

# Alternatives

None.
