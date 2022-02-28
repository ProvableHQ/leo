# An empty `import` list

## Example

This error occurs when no sub-packages
or items were specified in an import list.

Erroneous code example:

```js
import gardening.();
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370002]: Cannot import empty list
    --> test.leo:1:18
     |
   1 | import gardening.();
     |                  ^^
```

...as the compiler does not know what to import in `gardening`.

## Solutions

There are different solutions to this problems.
Here are 2 of them to consider.

### Comment out the `import`

If don't know yet what to import from `gardening`,
comment out the `import` like so:

```js
// import gardening.();
```

Later, you can come back and specify what to import like below.

You can also remove the `import` line entirely,
which will have the same effect.

### Specify items to `import`

If you know that you'd like to import, for example,
the functions `water_flowers` and `prune`,
you can specify them in the import list like so:

```js
import gardening.(water_flowers, prune);
```
