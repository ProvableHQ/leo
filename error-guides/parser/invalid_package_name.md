# Invalid package name

## Example

This error occurs when a package name in an `import` contains invalid characters.

Erroneous code example:

```js
import FOO.bar;
```

The compiler will reject this code with, for example...:

```js
Error [EPAR0370012]: package names must be lowercase alphanumeric ascii with underscores
    --> test.leo:1:8
     |
   1 | import FOO.bar;
     |        ^^^
```

In this specific case, you probably meant `foo.bar` instead.
If so, so you can solve the problem with:

```js
import foo.bar;
```
