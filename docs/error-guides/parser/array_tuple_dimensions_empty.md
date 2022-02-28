# Empty array dimensions

## Example

This error occurs when specifying an empty tuple as the dimensions of an array.

Erroneous code example:

```js
function main() {
    let foo = [42; ()];
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370023]: Array dimensions specified as a tuple cannot be empty.
    --> test.leo:2:20
     |
   2 |     let foo = [42; ()];
     |                    ^^
```

## Solution

If you wanted a single dimensional array, you can achieve that by specifying the length like so:

```js
function main() {
    let foo = [42; 4];
}
```

This will give you the array `[42, 42, 42, 42]`.

If instead you wanted a multi-dimensional array, e.g., a 2 x 3 matrix, you can achieve that with:

```js
function main() {
    let foo = [42; (2, 3)];
}
```

Alternatively, you can use the simple syntax all the way instead:

```js
function main() {
    let foo = [[42; 2]; 3];
}
```
