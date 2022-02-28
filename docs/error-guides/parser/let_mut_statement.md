# `let mut` is deprecated

## Example

This error occurs when a variable declaration is marked with `mut`.

Erroneous code example:

```js
function main() {
    let mut x = 0;
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370015]: let mut = ... is deprecated. `let` keyword implies mutabality by default.
    --> test.leo:2:5
     |
   2 |     let mut x = 0;
     |     ^^^^^^^
```

## Solution

As the `mut` modifier is implicitly assumed, the solution is to remove the `mut` modifier:

```js
function main() {
    let x = 0;
}
```
