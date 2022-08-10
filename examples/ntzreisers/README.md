# src/ntzreisers.leo

## Build Guide

To compile and run this Leo program, run:
```bash
leo run
```

## The Algorithm

This algorithm is detailed in "Hacker's Delight, 2nd edition"
by Henry S. Warren, section 5-4, figure 5-27.

First we isolate the rightmost `1` bit in the 32-bit input by
using the C idiom `x & (-x)`.  In Leo, the `-x` is
written as `0u32.sub_wrapped(x)`.

The smallest constant was found that has the property that when it was used to
divide the 33 arguments 0, 1, 2, ..., 2**31, the remainder is different for
each argument. This constant is 37.

In the algorithm, the remainder is used as an index into a table of size 37,
with 4 entries unused.  The table's values were chosen so that they give the
correct number of trailing zeros for the inputs.

This algorithm was proposed by John Reiser in the comp.arch.arithmetic newsgroup
on December 11, 1998:
https://groups.google.com/g/comp.arch.arithmetic/c/yBt-QHRVEGE/m/QcQ75P6tmJ4J
