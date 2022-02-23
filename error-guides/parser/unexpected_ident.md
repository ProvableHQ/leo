# Unexpected identifier: expected "x" -- got "y"

## Example

This error occurs when a specific *identifier*, e.g., `error` was expected but a different one,
e.g., `fail` was encountered instead.

Erroneous code example:

```js
function main() {
  console.fail("Houston we have a problem!");
}
```

The compiler will reject this code with:

```js
Error [EPAR0370007]: unexpected identifier: expected 'assert', 'error', 'log' -- got 'fail'
    --> test.leo:2:11
     |
   2 |   console.fail("Houston we have a problem!");
     |           ^^^^
```

## Solutions

The error message above says that `fail` cannot be used at that location,
and also lists a few identifiers that are valid. Note that this is context specific,
and depends on what preceded the valid tokens in the location.

The error message lists identifiers that are valid, e.g., `error`.
Here, since we used `.fail(...)`, we most likely wanted to trigger a compile error,
which `.error(...)` will achieve, so we use that instead...:

```js
function main() {
  console.error("Houston we have a problem!");
}
```

Note that this error currently only occurs when using `console`.
