# src/ntzsearchtree.leo

## Build Guide

To compile and run this Leo program, run:
```bash
leo run
```

## The Algorithm

An 8-bit version of this algorithm is described in "Hacker's Delight, 2nd
edition" by Henry S. Warren, section 5-4, figure 5-22.

This algorithm contains a fully-explicated search tree to find where the lowest
order 1 bit is in a 32-bit input, and returns the number of trailing zeros for
each case.

In the second half of the function, the low 16 bits are zero, so the masks that
select the higher bits are larger.  To make the numbers more readable, we
multiply the first half constants by `65536u32` which will be constant-folded
during Leo compilation and should not increase the resulting circuit size.
