# Illegal spread expression in array initializer

## Example

This error occurs when a spread expression, e.g., `...foo` occurs in an array initializer.

Erroneous code example:

```js
function main() {
    let foo = [0, 1];
    let array = [...foo; 3];
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370010]: illegal spread in array initializer
    --> test.leo:3:17
     |
   3 |     let array = [...foo; 3];
     |                 ^^^^^^^
```

## Solution

The Leo language does not allow `...foo` as the element to repeat
in an array repeat expression like the one above.
This is because `foo` is not an element but rather a full array.
One could imagine that the expression above means `[...foo, ...foo, ...foo]`.
That is, `...foo` repeated as many times as was specified in the array size.
However, that is ambiguous with `[element; 3]` resulting in an array with size `3`.

To solve the issue, disambiguate your intention.
Most likely, you really wanted `[...foo, ...foo, ...foo]`, so the solution is to write that out...:

```js
function main() {
    let foo = [0, 1];
    let array = [...foo, ...foo, ...foo];
}
```
