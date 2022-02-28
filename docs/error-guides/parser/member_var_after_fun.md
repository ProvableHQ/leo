# Member variables after circuit functions

## Example

This error occurs when circuit member variables occur after circuit member functions.

Erroneous code example:

```js
circuit Foo {
    function bar() {}

    baz: bool;
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370022]: Member functions must come after member variables.
    --> test.leo:4:5
     |
   4 |     baz: bool;
     |     ^^^
```

## Solution

The issue can be solved by moving all member variables before any circuit member functions...:

```js
circuit Foo {
    baz: bool;

    function bar() {}
}
```
