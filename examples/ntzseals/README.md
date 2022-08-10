# src/ntzseals.leo

## Build Guide

To compile and run this Leo program, run:
```bash
leo run
```
## The Algorithm

This algorithm is detailed in "Hacker's Delight, 2nd edition"
by Henry S. Warren, section 5-4, figure 5-25.

First we isolate the rightmost `1` bit in the 32-bit input by
using the C idiom `x & (-x)`.  In Leo, the `-x` is
written as `0u32.sub_wrapped(x)`.

A constant was discovered with the property that when it was multiplied by
by 0, 1, 2, 4, ... 2**31, the 33 values all had different patterns of their
6 highest bits.  The constant was also chosen to make multiplication easy to do with
a small number of shifts and adds on conventional hardware.

In the algorithm, the 6 high bits are used as an index into a table,
where 31 of the entries are unused.  The 33 used values were chosen so
that they gave the correct number of trailing zeros for the inputs.

This algorithm was proposed by David Seal in the comp.sys.acorn.tech
newsgroup, February  16, 1994:
https://groups.google.com/g/comp.sys.acorn.tech/c/blRy-AiIQ-0/m/3JxNHeKN75IJ

A further post that includes the table used was made by Michael Williams in the
comp.arch.arithmetic newsgroup, December 4, 1998:
https://groups.google.com/g/comp.arch.arithmetic/c/yBt-QHRVEGE/m/stFPPMD0b7AJ
