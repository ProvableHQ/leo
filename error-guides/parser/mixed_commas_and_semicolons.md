# Mixed commas and semicolons in circuit definitions

## Example

This error occurs when mixing semicolons, `;`,
and commas, `,` together in the list of member variables in a circuit definition.

Erroneous code example:

```js
circuit A {
    foo: u8,
    bar: u16;
}
```

The compiler will reject this code with:

```js
Error [EPAR0370006]: Cannot mix use of commas and semi-colons for circuit member variable declarations.
    --> test.leo:3:13
     |
   3 |     bar: u16;
     |             ^
```

## Solutions

The solution is simply to consistently use `;` or `,` after each member variable,
and avoid mixing `;` and `,` together. So we could write either...:

```js
circuit A {
    foo: u8,
    bar: u16,
}
```

...or write...:

```js
circuit A {
    foo: u8;
    bar: u16;
}
```

...and the compiler would accept it.
