# Deprecated `mut` parameter

## Example

This error occurs when a function parameter is marked as `mut`.

Erroneous code example:

```js
circuit Foo {
    function bar(mut x: u8) {
        x = 0;
    }
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370014]: function func(mut a: u32) { ... } is deprecated. Passed variables are mutable by default.
    --> test.leo:2:18
     |
   2 |     function bar(mut x: u8) {
     |                  ^^^^^
```

## Solution

As the `mut` modifier is implicitly assumed, the solution is to remove the `mut` modifier:

```js
circuit Foo {
    function bar(x: u8) {
        x = 0;
    }
}
```
