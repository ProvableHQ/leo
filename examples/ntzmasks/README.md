# src/ntzmasks.leo

## Build Guide

To compile and run this Leo program, run:
```bash
leo run
```

## The Algorithm

This algorithm is mentioned in "Hacker's Delight, 2nd edition"
by Henry S. Warren, section 5-4, figure 5-20.

It starts out by handling the all-zeros case.
Then `n` is initialized to `1` for the sole reason of saving an instruction at the end.
This means during the main body `n` is one more than the number
of trailing zeros so far detected.

The main body does a simple binary search of the 32-bit input for the
rightmost `1` bit.  The first check looks at the 16 bits
of the right half (low order) bits.  The condition
```
((x & 65535u32) == 0u32)
```
is true if the lower 16 bits are zero, in which case there
are at least 16 trailing zeros, added to `n`,
and `x` is shifted down by 16 bits to get ready for the next check.
If the first condition was false, we know
there is a `1` bit in the lower 16, so
we do not add anything to `n` and we do not shift `x`.

The second condition checks the lower 8 bits of the new `x`,
which are either the 17-to-24 lowest bits or the 8 lowest bits,
depending on whether the first statement shifted by 16 or not,
respectively.  If the second condition is true, we add
8 to the number of trailing zeros found so far, and shift right
by 8 bits. If the second condition is false, we go on to
the third condition.

This search is repeated two more times, accumulating the number
of trailing zeros (plus one).  The final return either keeps
the extra `1` bit by returning `n` if the final rightmost bit is 0,
or subtracts it from `n` if the final rightmost bit is `1`.
