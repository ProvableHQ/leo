# Deprecated `mut` parameter

## Example

This error occurs when a function parameter is marked as `mut`.

Erroneous code example:

```js
circuit Foo {
    bar: u8,

    function bar(mut self) {
        self.bar = 0;
    }
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370019]: `mut self` is no longer accepted. Use `&self` if you would like to pass in a mutable reference to `self`
    --> test.leo:4:18
     |
   4 |     function bar(mut self) {
     |                  ^^^^^^^^
```

## Solution

As the `mut` modifier is implicitly assumed, the solution is to remove the `mut` modifier from `self`:

```js
circuit Foo {
    bar: u8,

    function bar(self) {
        self.bar = 0;
    }
}
```
