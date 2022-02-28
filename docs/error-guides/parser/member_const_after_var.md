# `static const` after normal variables

## Example

This error occurs when `static const` circuit members occur after normal member variables.

Erroneous code example:

```js
circuit Foo {
    bar: u8,

    static const baz: bool = true;
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370020]: Member variables must come after member consts.
    --> test.leo:4:18
     |
   4 |     static const baz: bool = true;
     |                  ^^^^^^^^^^^^^^^^
```

## Solution

The issue can be solved by moving all `static const` members before normal member variables...:

```js
circuit Foo {
    static const baz: bool = true;

    bar: u8,
}
```
