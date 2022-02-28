# Invalid address literal

## Example

This error occurs when a syntactically invalid address is specified.

Erroneous code example:

```js
function main() {
    let addr = aleo1Qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9;
}
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370001]: invalid address literal: 'aleo1Qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9'
    --> test.leo:2:16
     |
   2 |     let addr = aleo1Qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9;
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

A valid address literal must start with `aleo1`,
followed by 58 characters any of which can be either a lowercase letter,
or an ASCII digit (`0` to `9`).

In the example above, the problem is `Q`, an uppercase letter,
and the second character after `aleo1`.

## Solution

To fix the issue, we can write...:

```js
function main() {
    let addr = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9;
}
```

...and the compiler will accept it.

Note however that the compiler does not check whether the address is valid on-chain, but merely that the written program follows the rules of the language grammar.
