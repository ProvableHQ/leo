# src/ntzloops.leo

## Build Guide

To compile and run this Leo program, run:
```bash
leo run
```

## The Algorithm

This algorithm is described in "Hacker's Delight, 2nd edition"
by Henry S. Warren, section 5-4, figure 5-23, as a simple loop
for counting number of trailing zeros that is fast (on traditional architectures)
when the number of trailing zeros is small.  In that same figure, in the end-of-line
comments, there is code for an analogous simple loop that is fast when the number
of trailing zeros is large.

We start out by using the C idiom `~x & (x - 1)` to create a word with 1-bits
at the positions of the trailing zeros in `x` and 0-bits elsewhere.
If there are no trailing zeros in `x`, the formula returns zero.  This idiom is
expressed in Leo as `!x & x.sub_wrapped(1u32);`.

Then we simply count the 1-bits by right shifting until `x` is zero and
counting the number of shifts using the variable `n`.

To get the effect of a while loop in Leo, one must use a `for` loop with the
enough iterations to accommodate all possible inputs, and then check the
while condition within the for loop.  Once the condition is false, the
loop continues until finished but the `if` statement inside the loop prevents
any further operations.
