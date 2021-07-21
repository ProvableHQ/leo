# Leo RFC 005: Countdown Loops

## Authors

- Max Bruce
- Collin Chin
- Alessandro Coglio
- Eric McCarthy
- Jon Pavlik
- Damir Shamanaev
- Damon Sicore
- Howard Wu

## Status

DRAFT

# Summary

This proposal suggests adding countdown loops and inclusive loop ranges into the Leo language.

# Motivation

In the current design of the language only incremental ranges are allowed. Though
in some cases there's a need for loops going in the reverse direction. This example
demonstrates the shaker sort algorithm where countdown loops are mocked:

```ts
function shaker_sort(a: [u32; 10], const rounds: u32) -> [u32; 10] {
    for k in 0..rounds {
        for i in 0..9 {
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }

        }
        for j in 0..9 { // j goes from 0 to 8
            let i = 8 - j; // j is flipped
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }
        }
    }
    return a;
}
```

Having a countdown loop in the example above could improve readability and 
usability of the language by making it more natural to the developer.

However, if we imagined this example using a countdown loop, we would see that 
it wouldn't be possible to count to 0; because the first bound of the range is
inclusive and the second is exclusive, and loops ranges must use only unsigned integers.

```ts
// loop goes 0,1,2,3,4,5,6,7,8
for i in 0..9 { /* ... */ }

// loop goes 9,8,7,6,5,4,3,2,1 
for i in 9..0 { /* ... */ }
```

Hence direct implementation of the coundown loop ranges would create asymmetry (1)
and would not allow loops to count down to 0 (2). To implement coundown loops and 
solve these two problems we suggest adding an inclusive range bounds.

# Design

## Coundown loops

Countdown ranges do not need any changes to the existing syntax. However their 
functionality needs to be implemented in the compiler.

```ts
for i in 5..0 {}
```

## Inclusive ranges

To solve loop asymmetry and to improve loop ranges in general we suggest adding 
inclusive range operator to Leo. Inclusive range would extend the second bound 
of the loop making it inclusive (instead of default - exclusive) 
therefore allowing countdown loops to reach 0 value.

```ts
// default loop: 0,1,2,3,4
for i in 0..5 {}

// inclusive range: 0,1,2,3,4,5
for i in 0..=5 {}
```

## Step and Direction

We remark that the step of both counting-up and counting-down loops is implicitly 1;
that is, the loop variable is incremented or decremented by 1.

Whether the loop counts up or down is determined by how the starting and ending bounds compare.
Note that the bounds are not necessarily literals;
they may be more complex `const` expressions, and thus in general their values are resolved at code flattening time.
Because of the type restrictions on bounds, their values are always non-negative integers.
If `S` is the integer value of the starting bound and `E` is the integer value of the ending bound,
there are several cases to consider:
1. If `S == E` and the ending bound is exclusive, there is no actual loop; the range is empty.
2. If `S == E` and the ending bound is inclusive, the loop consists of just one iteration; the loop counts neither up nor down.
3. If `S < E` and the ending bound is exclusive, the loop counts up, from `S` to `E-1`.
4. If `S < E` and the ending bound is inclusive, the loop counts up, from `S` to `E`.
5. If `S > E` and the ending bound is exclusive, the loop counts down, from `S` to `E+1`.
6. If `S > E` and the ending bound is inclusive, the loop counts down, from `S` to `E`.

Cases 3 and 5 consist of one or more iterations; cases 4 and 6 consist of two or more iterations.

## Example

The code example demostrated in the Motivation part of this document 
could be extended (or simplified) with the suggested syntax:

```ts
function shaker_sort(a: [u32; 10], const rounds: u32) -> [u32; 10] {
    for k in 0..rounds {
        for i in 0..9 { // i goes from 0 to 8
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }

        }
        for i in 8..=0 { // i goes from 8 to 0
            if a[i] > a[i + 1] {
                let tmp = a[i];
                a[i] = a[i + 1];
                a[i + 1] = tmp;
            }
        }
    }
    return a;
}
```

# Drawbacks

No obvious drawback.

# Effect on Ecosystem

Suggested change should have no effect on ecosystem because of its backward compatibility.

# Alternatives

## Mocking

Coundown loops can be mocked manually.

## Exclusive Starting Bounds

While the ability to designate the ending bound of a loop as either exclusive or inclusive is critical as discussed below,
we could also consider adding the ability to designate the starting bound of a loop as either exclusive or inclusive.
If we do that, we run into a sort of asymmetry in the defaults for starting and ending bounds:
the default for the starting bound is inclusive, while the default for ending bounds is exclusive.

The most symmetric but verbose approach is exemplified as follows:
* `0=..=5` for `0 1 2 3 4 5`
* `0<..=5` for `1 2 3 4 5`
* `0=..<5` for `0 1 2 3 4`
* `0<..<5` for `1 2 3 4`
* `5=..=0` for `5 4 3 2 1 0`
* `5>..=0` for `4 3 2 1 0`
* `5=..>0` for `5 4 3 2 1`
* `5>..>0` for `4 3 2 1`
That is, this approach makes exclusivensss and inclusiveness implicit.
The use of `<` vs. `>` also indicates a loop direction, which can be inferred anyhow when the `const` bounds are resolved,
so that would entail an additional consistency check,
namely that the inequality sign/signs is/are consistent with the inferred loop direction.

Within the symmetric approach above, there are different options for defaults.
The most symmetric default would be perhaps `=` for both bounds,
but that would be a different behavior from current Leo.
We could instead go for different defaults for starting and ending bounds,
i.e. `=` for the starting bound and `<` or `>` (depending on direction) for the ending bound.
