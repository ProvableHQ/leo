# src/ntzgaudet.leo

## Build Guide

To compile and run this Leo program, run:
```bash
leo run
```

## The Algorithm

This algorithm is described in "Hacker's Delight, 2nd edition" by Henry
S. Warren, section 5-4, section 5-24, as interesting due to being branch-free,
not using table lookups, and having parallelism.  It is attributed to Dean Gaudet
in private communication to Henry S. Warren.

First we isolate the rightmost `1` bit in the 32-bit input by
using the C idiom `x & (-x)`.  In Leo, the `-x` is
written as `0u32.sub_wrapped(x)`.  The result is stored in `y`.

Then we compute six intermediate variables that count different numbers
of trailing zeros.  The first variable, `bz`, just counts 1 if `y` is completely zero.

To get the other five variables, we do binary search in parallel, using 5 masks,
each looking at a different symmetric pattern of 16 bits.  For example, `b4` counts 16 if
the low 16 bits are zero and counts zero otherwise.  Then `b3` uses a mask `y &
0x00FF00FF` to count eight 0-bits if the result is zero and zero 0-bits
otherwise.  The masks for `b2`, `b1`, and `b0` can count four, two, and
one 0-bits similarly.

The varables `bz, b4, .., b0` are all independent, and their values are added up
for the result.
