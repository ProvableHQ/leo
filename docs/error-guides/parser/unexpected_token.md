# An unexpected token

## Example

This error occurs when the Leo compiler tries to parse your program.
More specifically, during a phase called 'lexing'.
In this phase, the compiler first takes your code,
consisting of characters, and interprets it as a list of tokens.
These tokens are a sort of *alphabet* internal to Leo.

Consider the English language. It only has 26 letters in its alphabet.
So there are some letters, e.g., `Γ` from the greek alphabet,
which would not fit if we tried to "tokenize" English.

Leo, while being a programming language, is similar here.
There are characters or sequences of characters,
that Leo does not understand and cannot lex into tokens.
Since this error occured, that is what has happened.

Erroneous code example:

```js
~
```

The compiler will reject this code with:

```js
Error [EPAR0370000]: ~
    --> test.leo:1:1
     |
   1 | ~
     | ^
```

## Solutions

What the solution to an unexpected token is depends on what you wanted to achieve.
Most likely, you made a typo somewhere.
For a more complete overview of valid Leo tokens, consult the [Leo grammar](https://github.com/AleoHQ/leo/blob/master/grammar/README.md).
