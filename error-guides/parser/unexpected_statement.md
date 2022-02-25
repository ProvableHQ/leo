# Unexpected statement: expected "x" -- got "y"

## Example

This error occurs when a statement, which isn't `if`, follows `else` directly.

Erroneous code example:

```js
function main ()  {
    if true {
        console.log("It was true.");
    } else
        console.log("It was false.");
}
```

The compiler will reject this code with:

```js
Error [EPAR0370008]: unexpected statement: expected 'Block or Conditional', got 'console.log("It was false.", );'
    --> test.leo:5:9
     |
   5 |         console.log("It was false.");
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## Solutions

To fix the problem, wrap the statement in a block, so by turning the snippet above into...:

```js
function main ()  {
    if true {
        console.log("It was true.");
    } else {
        console.log("It was false.");
    }
}
```

...the error is fixed.
