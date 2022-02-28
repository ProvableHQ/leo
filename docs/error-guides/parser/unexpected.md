# Expected "x" -- got "y"

## Example

This error occurs when a specific token, e.g., `class` was encountered but a different one,
e.g., `circuit` was expected instead.

Erroneous code example:

```js
class A {}
```

The compiler will reject this code with:

```js
Error:     --> main.leo:1:1
     |
   1 | class A {}
     | ^^^^^
     |
     = expected 'import', 'circuit', 'function', 'test', '@' -- got 'class'
```

## Solutions

The error message above says that `class` cannot be used at that location,
and also lists a few tokens that are valid. Note that this is context specific,
and depends on what tokens preceded the current token.
Using the list of tokens that are valid, and knowing that `circuit A {}` is valid syntax,
we replace `class` with `circuit`...:

```js
circuit A {}
```

...and the error is now resolved.
