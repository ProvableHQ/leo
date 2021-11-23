# FAQs

#### For some given code, changing the value in a constant variable changes the number of constraints in the generated struct. Is this behavior correct?

**Yes**, take the integers as an example. In Leo, integers are represented as its binary decomposition,
with each bit occupying one field element (that takes on 0 or 1). Then, for an expression such as `a == 4u32`, the operation to evaluate equality
would comprise a linear pass of bitwise `AND` operations, comparing every bit in the **variable** value with each bit in the **constant** value.

As the constant value is already known to the compiler during struct synthesis, the compiler is already able to complete part of the equality evaluation,
by assuming that any bit in the constant value that is `0` will clearly evaluate to `0`. As such, depending on the value of the constant integer in your code,
the total number of constraints in the generate struct can vary.

To illustrate this, here are two examples to show the difference:
```
constant = 00000001
variable = abcdefgh
---------------------------------
output   = 0000000h (1 constraint)
```

```
constant = 01110001
variable = abcdefgh
---------------------------------
output   = 0bcd000h (4 constraints)
```
