# An unexpected end of file

## Example

This error occurs when the Leo compiler tries to parse your program
and unexpectedly reaches the end of a `.leo` file.

Erroneous code example:

```js
function main() {
```

The compiler will reject this code with:

```js
Error [EPAR0370003]: unexpected EOF
    --> test.leo:1:17
     |
   1 | function main() {
     |                 ^
```

## Solutions

The problem typically occurs when there are unbalanced delimiters,
which we have an instance of above.
More specifically, in the example,
the issue is that there is no `}` to close the opening brace `{`.

An even simpler variant of this is:

```js
function main(
```

The solution here is to close the opening delimiter, in this case `(`.

## The general issue

To illustrate the heart of the problem, consider this invalid file:

```js
// ↳ main.leo
function
```

When parsing the file, the compiler expects something, in this case,
the function's name, but instead, the parser reaches the end of the file.
