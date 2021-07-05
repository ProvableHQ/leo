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

This proposal suggests adding countdown loops and inclusive loop ranges into Leo language.

# Motivation

In the current design of the language only incremental ranges are allowed. Though
in some cases there's a need for loops going in the reverse direction. This example
demonstrates the bubble sort algorithm where countdown loops are mocked:

```ts
function bubble_sort(mut a: [u32; 10]) -> [u32; 10] {
    for i in 0..9 { // i counts up
        for j in 0..9-i { // i is flipped
            if (a[j] > a[j+1]) {
                let tmp = a[j];
                a[j] = a[j+1];
                a[j+1] = tmp;
            }
        }
    }
    return a
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

# Drawbacks

- 

# Effect on Ecosystem

Suggested change should have no effect on ecosystem because of its backward compatibility.

# Alternatives

Coundown loops can be mocked manually. 
