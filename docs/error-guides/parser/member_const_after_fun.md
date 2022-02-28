# `static const` after circuit functions

## Example

This error occurs when `static const` circuit members occur after circuit member functions.

Erroneous code example:

```js
circuit Foo {
    function bar() {}

    static const baz: bool = true;
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370021]: Member functions must come after member consts.
    --> test.leo:4:18
     |
   4 |     static const baz: bool = true;
     |                  ^^^^^^^^^^^^^^^^
```

## Solution

The issue can be solved by moving all `static const` members before circuit member functions...:

```js
circuit Foo {
    static const baz: bool = true;

    function bar() {}
}
```
