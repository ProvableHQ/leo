# Invalid assignment target

## Example

This error currently occurs when a `static const` member or a member function
is used as the target of an assignment statement.

Erroneous code example:

```js
circuit Foo {
    static const static_const: u8 = 0;
}

function main() {
    Foo::static_const = 0;
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370011]: invalid assignment target
    --> test.leo:6:5
     |
   6 |     Foo::static_const = 0;
     |     ^^^^^^^^^^^^^^^^^
```

It's not possible to assign to `static const` members or member functions,
so this is not allowed syntax.
The solution is likely to rethink your approach to the problem you are solving.
