# src/ntzsmallvals.leo

## Build Guide

To compile and run this Leo program, run:
```bash
leo run
```

## The Algorithm

This algorithm is mentioned in "Hacker's Delight, 2nd edition"
by Henry S. Warren, section 5-4, section 5-4, figure 5-21.

It is similar to the algorithm described in the `ntzmasks` example
(figure 5-20 in the book) in that it uses binary search to find the
number of trailing zeros.  However, instead of using masks to select
the lower `N` bits, it shifts `x` left (discarding high bits) into
another variable `y` to check if the result is nonzero, and it counts
down from 31 instead of up from 1.  Another difference is that the
constant values is this algorithm are smaller, so it is easier to
read.
