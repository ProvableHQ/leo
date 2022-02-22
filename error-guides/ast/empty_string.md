# An empty string `""`

## Example

This error occurs when an empty string literal was specified.

Erroneous code example:

```js
function main() {
    let empty_string = "";
}
```

The compiler will reject this code with:

```java
Error:     --> main.leo:2:24
     |
   2 |     let empty_string = "";
     |                        ^^
     |
     = Cannot constrcut an empty string: it has the type of [char; 0] which is not possible.
```

As the error indicates, the type of `""`, the empty string, would be `[char; 0]`.
The type is not, as one might expect in languages like Rust or Java,
a `String` or `str`, where the size is statically unknown.
Rather, string literals in Leo are arrays of `char`s.
So given that `""` is an array type with size `0`,
the Leo compiler will reject the program, as it would have done with e.g...:
```js
function main() {
    let empty: [u8; 0] = [];
}
```

## Solutions

You will not be able to use `""`, but all is not lost.
Depending on what you want to achieve in your program, there may be solutions.
For example, if you want to select between two strings,
you can pad the other strings with whitespace to represent emptiness.

```js
function main() {
    let condition = false;
    let a_or_empty = condition ? "a" : " ";
}
```

Here, `" "` represents the empty string but is of the same type as `"a"`.
