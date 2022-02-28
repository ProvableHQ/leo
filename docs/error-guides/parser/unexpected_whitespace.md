# Unexpected whitespace

## Example

This error occurs when there was unexpected white space when in your program.
Typically, the white space occurs in a literal with a typed suffix.

Erroneous code example:

```js
function main() {
    let x = 1 u8;
}
```

The compiler will reject this code with:

```js
Error [EPAR0370004]: Unexpected white space between terms 1 and u8
    --> test.leo:2:13
     |
   2 |     let x = 1 u8;
     |             ^
```

## Solutions

The problem is solved by removing the white space between the literal and its suffix. So given the example above, we can fix it by writing:

```js
function main() {
    let x = 1u8;
}
```
