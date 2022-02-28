# Expected string "x" -- got "y"

## Example

This error occurs when a specific "string" (in reality a token),
was expected but a different one was expected instead.

Erroneous code example:

```js
function main ()  {
    let x: [u8; (!)] = [0];
}
```

The compiler will reject this code with:

```js
Error [EPAR0370009]: unexpected string: expected 'int', got '!'
    --> test.leo:2:18
     |
   2 |     let x: [u8; (!)] = [0];
     |                  ^

```

## Solutions

The error message "unexpected string" depends on the context.
In the example above, we need to replace `!` with `1`...:

```js
function main ()  {
    let x: [u8; 1] = [0];
}
```
