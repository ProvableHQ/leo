# Leo group operations.

## Run Guide

To run this program, run:
```bash
leo run main <inputs>
```

## Execute Guide

To execute this program, run:
```bash
leo execute main <inputs>
```

## Overview

This example shows how to do basic operations over groups.

It takes the input data from inputs/groups.in


## Documentation Group Element

The set of affine points on the elliptic curve passed into the Leo compiler forms a group.
A subset of those points, defined by a chosen generator point, forms a subgroup of the group.
Leo supports the set of points in this subgroup as a primitive data type.
Group elements are special since their values can be defined as coordinate pairs ```(x, y)group```.
The `group` type keyword group must be used when specifying a pair of group coordinates since implicit syntax would collide with normal tuple `(a, b)` values.

```
let a = 0group; // the zero of the group

let b = group::GEN; // the group generator

let c = (0, 1)group; // coordinate notation
```
