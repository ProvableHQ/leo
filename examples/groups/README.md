# Leo group operations.

## Build Guide

To compile this program, run:
```bash
leo build
```

To run this program, run:
```bash
leo run main
```

## Overview 

This example shows how to do basic operations over groups.

It takes the input data from inputs/groups.in


## Documentation Group Element

The set of affine points on the elliptic curve passed into the Leo compiler forms a group. Leo supports this set as a primitive data type. Group elements are special since their values can be defined as coordinate pairs ```(x, y)group```. The group type keyword group must be used when specifying a pair of group coordinates since implicit syntax would collide with normal tuple (a, b) values.

```
let b = 0group; // the zero of the group

let a = 1group; // the group generator

let c = 2group; // 2 * the group generator

let d = (0, 1)group; // coordinate notation
```

