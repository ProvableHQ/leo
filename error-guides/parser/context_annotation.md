# `@context function` is deprecated

## Example

This error occurs when a function is prefixed with `@context`.

Erroneous code example:

```js
@context()
function foo() {
    // logic...
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370017]: "@context(...)" is deprecated. Did you mean @test annotation?
    --> test.leo:1:2
     |
   1 | @context()
     |  ^^^^^^^
```

## Solution

The `@context function` syntax is deprecated, but you can use `@test function` instead:

```js
@test
function foo() {
    // logic...
}
```
