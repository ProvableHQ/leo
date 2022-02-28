# Unable to parse array dimensions

## Example

This error occurs when there is a syntax error in the array dimensions of an array initializer.

Erroneous code example:

```js
function main() {
    let x = [1; +];
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370018]: unable to parse array dimensions
    --> test.leo:2:13
     |
   2 |     let x = [1; +];
     |             ^
```

## Solution

In the case above, the error occurs due to the `+`.
The issue can be resolved by specifying the number of elements desired, e.g., `5`...:

```js
function main() {
    let x = [1; 5];
}
```
