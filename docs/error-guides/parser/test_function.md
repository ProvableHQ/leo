# `test function` is deprecated

## Example

This error occurs when a function is prefixed with `test`.

Erroneous code example:

```js
test function foo() {
    // logic...
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370016]: "test function..." is deprecated. Did you mean @test annotation?
    --> test.leo:1:1
     |
   1 | test function foo() {
     | ^^^^
```

## Solution

The `test function` syntax is deprecated, but you can achieve the same result with `@test function`:

```js
@test
function foo() {
    // logic...
}
```
