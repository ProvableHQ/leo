# src/ntzdebruijn.leo

## Build Guide

To compile and run this Leo program, run:
```bash
leo run
```

## The Algorithm

This algorithm is detailed in "Hacker's Delight, 2nd edition"
by Henry S. Warren, section 5-4, figure 5-26.  Here is a summary.

After handling the all-zeros case,
we isolate the rightmost `1` bit in the 32-bit input by
using the C idiom `x & (-x)`.  In Leo, the `-x` is
written as `0u32.sub_wrapped(x)`.

A constant was discovered with the property that when it was multiplied
by 1, 2, 4, ... 2**31, the 32 values all had different high 5-bit patterns.
This constant is `0x04D7651F`.

In the algorithm, the 5 high bits are used as an index into the table
`{0, 1, 2, 24, 3, 19, ...}`.  The table's values were chosen so that
they gave the correct number of trailing zeros for the inputs.

For example, if the isolated bit has 4 trailing zeros, the number is 2**4.
The high 5 bits of `2**4 * 0x04D7651F` are `01001` which is 9.  The table
value at index 9 is therefore 4.

This algorithm was proposed by Danny Dubé in the comp.compression.research newsgroup:
https://groups.google.com/g/comp.compression.research/c/x0NaZ3CJ6O4/m/PfGuchA7o60J

A description of de Bruijn cycles and their use for bit indexing can be seen
here:  http://supertech.csail.mit.edu/papers/debruijn.pdf
