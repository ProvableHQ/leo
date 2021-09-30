# Leo RFC 002: Bounded Recursion

## Authors

The Aleo Team.

## Status

FINAL

## Summary

This proposal provides support for recursion in Leo,
via a user-configurable limit to the allowed depth of the recursion.
If the recursion can be completely inlined without exceeding the limit,
compilation succeeds;
otherwise, an informative message is shown to the user,
who can try and increase the limit.
Compilation may also fail
if a circularity is detected before exceeding the limit.

Future analyses may also recognize cases in which the recursion terminates,
informing the user and setting or suggesting a sufficient limit.

A similar approach could be also used for loops in the future.
User-configurable limits may be also appropriate for
other compiler transformations that are known to terminate
but could result in a very large number of R1CS constraints.

## Motivation

Leo currently allows functions to call other functions,
but recursion is disallowed:
a function cannot call itself, directly or indirectly.
However, recursion is a natural programming idiom in some cases,
compared to iteration (i.e. loops).

## Background

### Function Inlining

Since R1CS are flat collections of constraints,
compiling Leo to R1CS involves _flattening_ the Leo code:
unrolling loops, inlining functions, decomposing arrays, etc.
Of interest to this RFC is the inlining of functions,
in which a function call is replaced with the function body,
after binding the formal parameters to the the actual arguments,
and taking care to rename variables if needed to avoid conflicts.

Since the `main` function is the entry point into a Leo program,
conceptually, for the purpose of this RFC,
we can think of function inlining as transitively inlining
all the functions into `main`
(this is just a conceptual model;
it does not mean that it should be necessarily implemented this way).
This is a simple example,
where '`===> {<description>}`' indicates a transformation
described in the curly braces:
```js
function f(x: u32) -> u32 {
    return 2 * x;
}
function main(a: u32) -> u32 {
    return f(a + 1);
}

===> {inline call f(a + 1)}

function main(a: u32) -> u32 {
    let x = a + 1; // bind actual argument to formal argument
    return 2 * x; // replace call with body
}
```

### Constants and Variables

A Leo program has two kinds of inputs: constants and variables;
both come from input files.
They are passed as arguments to the `main` functions:
the parameters marked with `const` receive the constant inputs,
while the other parameters receive the variable inputs.
Leo has constants and variables,
of which the just mentioned `main` parameters are examples;
constants may only depend on literals and other constants,
and therefore only on the constant inputs of the program;
variables have no such restrictions.

The distinction between constants and variables
is significant to the compilation of Leo to R1CS.
Even though specific values of both constant and variable inputs
are known when the Leo program is compiled and the zk-proof is generated,
the generated R1CS does not depend
on the specific values of the variable inputs;
it only depends on the specific values of the constant inputs.
Stated another way, Leo variables are represented by R1CS variables,
while Leo constants are folded into the R1CS.

For instance, in
```js
function main(base: u32, const exponent: u32) -> u32 {
    return base ** exponent; // raise base to exponent
}
```
the base is a variable while the exponent is a constant.
Both base and exponent are known, supplied in the input file,
e.g. the base is 2 and the exponent is 5.
However, only the information about the exponent being 5
is folded into the R1CS, which retains the base as a variable.
Conceptually, the R1CS corresponds to the _partially evaluated_ Leo program
```js
function main(base: u32) -> u32 {
    return base ** 5;
}
```
where the constant `exponent` has been replaced with its value 5.

This partial evaluation is carried out
as part of the Leo program flattening transformations mentioned earlier.
This also involves constant propagation and folding,
e.g. a constant expression `exponent + 1` is replaced with 6
when the constant `exponent` is known to be 5.
(The example program above does not need any constant propagation and folding.)

Circling back to the topic of Leo function inlining,
it is the case that, due to the aforementioned partial evaluation,
the `const` arguments of function calls have known values
when the flattening transformations are carried out.

### Inlining Recursive Functions

In the presence of recursion,
attempting to exhaustively inline function calls does not terminate in general.
However, in conjunction with the partial evaluation discussed above,
inlining of recursive functions may terminate, under appropriate conditions.

This is an example:
```js
function double(const count: u32, sum: u32) -> u32 {
    if count > 1 {
        return double(count - 1, sum + sum);
    } else {
        return sum + sum;
    }
}
function main(x: u32) -> u32 {
    return double(3, x);
}

===> {inline call double(3, x)}

function main(x: u32) -> u32 {
    let sum1 = x; // bind and rename parameter of function sum
    if 3 > 1 {
        return double(2, sum1 + sum1);
    } else {
        return sum1 + sum1;
    }
}

===> {evaluate 3 > 1 to true and simplify if statement}

function main(x: u32) -> u32 {
    let sum1 = x;
    return double(2, sum1 + sum1);
}

===> {inine call double(2, sum1 + sum1)}

function main(x: u32) -> u32 {
    let sum1 = x;
    let sum2  = sum1 + sum1; // bind and rename parameter of function sum
    if 2 > 1 {
        return double(1, sum2 + sum2);
    } else {
        return sum2 + sum2;
    }
}

===> {evaluate 2 > 1 to true and simplify if statement}

function main(x: u32) -> u32 {
    let sum1 = x;
    let sum2  = sum1 + sum1;
    return double(1, sum2 + sum2)
}

===> {inline call double(1, sum2 + sum2)}

function main(x: u32) -> u32 {
    let sum1 = x;
    let sum2  = sum1 + sum1;
    let sum3 = sum2 + sum2; // bind and rename parameter of function sum
    if 1 > 1 {
        return double(0, sum3 + sum3);
    } else {
        return sum3 + sum3;
    }
}

===> {evaluate 1 > 1 to false and simplify if statement}

function main(x: u32) -> u32 {
    let sum1 = x;
    let sum2  = sum1 + sum1;
    let sum3 = sum2 + sum2;
    return sum3 + sum3;
}
```

This is a slightly more complex example
```js
function double(const count: u32, sum: u32) -> u32 {
    if count > 1 && sum < 30 {
        return double(count - 1, sum + sum);
    } else {
        return sum + sum;
    }
}
function main(x: u32) -> u32 {
    return double(3, x);
}

===> {inline call double(3, x)}

function main(x: u32) -> u32 {
    let sum1 = x; // bind and rename parameter of function sum
    if 3 > 1 && sum1 < 30 {
        return double(2, sum1 + sum1);
    } else {
        return sum1 + sum1;
    }
}

===> {evaluate 3 > 1 to true and simplify if test}

function main(x: u32) -> u32 {
    let sum1 = x;
    if sum1 < 30 {
        return double(2, sum1 + sum1);
    } else {
        return sum1 + sum1;
    }
}

===> {inline call double(2, sum1 + sum1)}

function main(x: u32) -> u32 {
    let sum1 = x;
    if sum1 < 30 {
        let sum2 = sum1 + sum1; // bind and rename parameter of function sum
        if 2 > 1 && sum2 < 30 {
            return double(1, sum2 + sum2);
        } else {
            return sum2 + sum2;
        }
    } else {
        return sum1 + sum1;
    }
}

===> {evaluate 2 > 1 to true and simplify if test}

function main(x: u32) -> u32 {
    let sum1 = x;
    if sum1 < 30 {
        let sum2 = sum1 + sum1;
        if sum2 < 30 {
            return double(1, sum2 + sum2);
        } else {
            return sum2 + sum2;
        }
    } else {
        return sum1 + sum1;
    }
}

===> {inline call double(1, sum2 + sum2)}

function main(x: u32) -> u32 {
    let sum1 = x;
    if sum1 < 30 {
        let sum2 = sum1 + sum1;
        if sum2 < 30 {
            let sum3 = sum2 + sum2; // bind and rename parameter of function sum
            if 1 > 1 && sum3 < 30 {
                return double(0, sum3 + sum3);
            } else {
                return sum3 + sum3;
            }
        } else {
            return sum2 + sum2;
        }
    } else {
        return sum1 + sum1;
    }
}

===> {evaluate 1 > 1 to false and simplify if statement}

function main(x: u32) -> u32 {
    let sum1 = x;
    if sum1 < 30 {
        let sum2 = sum1 + sum1;
        if sum2 < 30 {
            let sum3 = sum2 + sum2;
            return sum3 + sum3;
        } else {
            return sum2 + sum2;
        }
    } else {
        return sum1 + sum1;
    }
}
```

But here is an example in which the inlining does not terminate:
```js
function forever(const n: u32) {
    forever(n);
}
function main() {
    forever(5);
}

===> {inline call forever(5)}

function main() {
    forever(5);
}

===> {inline call forever(5)}

...
```

## Design

### Configurable Limit

Our proposed approach to avoid non-termination
when inlining recursive functions is to
(i) keep track of the depth of the inlining call stack and
(ii) stop when a certain limit is reached.
If the limit is reached,
the compiler will provide an informative message to the user,
explaining which recursive calls caused the limit to be reached.
The limit is configurable by the user.
In particular, based on the informative message described above,
the user may decide to re-attempt compilation with a higher limit.

Both variants of the `double` example given earlier reach depth 3
(if we start with depth 0 at `main`).

The default limit (i.e. when the user does not specify one)
should be chosen in a way that
the compiler does not take too long to reach the limit.
Since inlining larger functions
takes more time than inlining smaller functions,
we may consider adjusting the default limit
based on some measure of the complexity of the functions.

In any case, compiler responsiveness is a broader topic.
As the Leo compiler sometimes performs expensive computations,
it may be important that it provide periodic output to the user,
to reassure them that the compiler is not stuck.

We will add a flag to the `leo` CLI whose long form is
```
--inline-limit
```
and whose short form is
```
-il
```
This option is followed by a number (more precisley, a positive integer)
that specifies the limit to the depth of the inlining stack.

The name of this option has been chosen
according to a `--...-limit` template
that may be used to specify other kinds of limits,
as discussed later.

In Aleo Studio, this compiler option is presumably specified
via GUI preferences and build configurations.

### Circularity Detection

Besides the depth of the inlining call stack,
the compiler could also keep track of
the values of the `const` arguments at each recursive call.
If the same argument values are encountered twice,
they indicate a circularity
(see the discussion on termination analysis below):
in that case, there is no need to continue inlining until the limit is reached,
and the compiler can show to the user the trace of circular calls.

This approach would readily reject the `forever` example given earlier.

## Drawbacks

This proposal does not appear to bring any real drawbacks,
other than making the compiler inevitably more complex.
But the benefits to support recursion justifies the extra complexity.

## Effect on Ecosystem

This proposal does not appear to have any direct effects on the ecosystem.
It simply enables certain Leo programs to be written in a more natural style.

## Alternatives

An alternative approach is to treat recursion analogously to loops.
That is, we could restrict the forms of allowed recursion
to ones whose inlining is known to terminate at compile time.

However, the configurable limit approach seems more flexible.
It does not even preclude a termination analysis (discussed below).
Furthermore, in practical terms,
non-termination is not much different from excessively long computation.
and the configurable limit approach may be uniformly suitable
to avoid both non-termination and excessively long computation (discussed below).

## Future Extensions

### Termination Analysis

In general, a recursive function
(a generic kind of function, not necessarily a Leo function)
terminates when
there exists a _measure_ of its arguments
that decreases at each recursive call,
under the tests that govern the recursive call,
according to a _well-founded relation_.
This is well known in theorem proving,
where termination of recursive functions
is needed for logical consistency.

For example, the mathematical factorial function
on the natural numbers (i.e. non-negative integers)
```
n! =def= [IF n = 0 THEN 1 ELSE n * (n-1)!]
```
terminates because, if `n` is not 0, we have that `n-1 < n`,
and `<` is well-founded on the natural numbers;
in this example, the measure of the argument is the argument itself.
(A relation is well-founded when
it has no infinite strictly decreasing sequence;
note that, in the factorial example,
we are considering the `<` relation on natural numbers only,
not on all the integers).

This property is undecidable in general,
but there are many cases in which termination can be proved automatically,
as routinely done in theorem provers.

In Leo, we are interested in
the termination of the inlining transformations.
Therefore, the termination condition above
must involve the `const` parameters of recursive functions:
a recursive inlining in Leo terminates when
there exists a measure of the `const` arguments
that decreases at each recursive call,
under the tests that govern the recursive call,
according to a well-founded relation.
The governing test must necessarily involve the `const` parameters,
but they may involve variable parameters as well,
as one of the `double` examples given earlier shows.

We could have the Leo compiler attempt to recognize
recursive functions whose `const` parameters
satisfy the termination condition given above.
(This does not have to involve any proof search;
the compiler could just recognize structures
for which a known proof can be readily instantiated.)
If the recognition succeed,
we know that the recursion inlining will terminate,
and also possibly in how many steps,
enabling the information to be presented to the user
in case the configurable limit is insufficient.
If the recognition fails,
the compiler falls back to inlining until
either inlining terminates or the limit is reached.

### Application to Loops

Loops are conceptually not different from recursion.
Loops and recursion are two ways to repeat computations,
and it is well-known that each can emulate the other in various ways.

Currenly Leo restricts the allowed loop statements
to a form whose unrolling always terminates at compile time.
If we were to use a similar approach for recursion,
we would only allow certain forms of recursion
whose inlining always terminates at compile time
(see the discussion above about termination analysis).

Turning things around,
we could consider allowing general forms of loops (e.g. `while` loops)
and use a configurable limit to unroll loops.
We could also detect circularities
(when the values of the local constants of the loop repeat themselves).
We could also perform a termination analysis on loops,
which in particular would readily recognize
the currently allowed loop forms to terminate.
All of this should be the topic of a separate RFC.

### Application to Potentially Slow Transformations

Some flattening transformations in the Leo compiler are known to terminate,
but they may take an excessively long time to do so.
Examples include decomposing large arrays into their elements
or decomposing large integers (e.g. of type `u128`) into their bits.
Long compilation times have been observed for cases like these.

Thus, we could consider using configurable limits
not only for flattening transformations that may not otherwise terminate,
but also for ones that may take a long time to do so.
This is a broader topic that should be discussed in a separate RFC.
