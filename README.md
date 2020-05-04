# The Leo Language
* All code examples can be copied and pasted into `main.leo` directly and executed with cargo run
* Programs should be formatted:
    1. Import definitions
    2. Struct definitions
    3. Function definitions

### Integers:
Currently, all integers are parsed as u32.
You can choose to explicitly add the type or let the compiler interpret implicitly.
```rust
function main() -> u32 {
    let a : u32 = 1u32 + 1u32; // explicit type
    let b = 1 - 1; // implicit type
    let c = 2 * 2;
    let d = 4 / 2;
    let e = 2 ** 3;
    return a
}
```

### Field Elements:
Field elements must have the type added explicitly.
```rust
function main() -> fe {
    let f : fe = 21888242871839275222246405745257275088548364400416034343698204186575808495617fe;
    let a = 1fe + 1fe;
    let b = 1fe - 1fe;
    let c = 2fe * 2fe;
    let d = 4fe / 2fe;
    return a
}
```

### Operator Assignment Statements:
```rust
function main() -> u32 {
  let a = 10;
  a += 5;
  a -= 10;
  a *= 5;
  a /= 5;
  a **= 2;

  return a
}
```


### Booleans:
```rust
function main() -> bool {
    let a : bool = true || false;
    let b = false && false;
    let c = 1 == 1;
    return a
}
```

### Arrays:
Leo supports static arrays with fixed length.
Array type must be explicitly stated
```rust
function main() -> u32[2] {
    // initialize an integer array with integer values
    let a : u32[3] = [1, 2, 3];

    // set a member to a value
    a[2] = 4;

    // initialize an array of 4 values all equal to 42
    let b = [42; 4];

    // initialize an array of 5 values copying all elements of b using a spread
    let c = [1, ...b];

    // initialize an array copying a slice from `c`
    let d = c[1..3];

    // initialize a field array
    let e = [5fe; 2];

    // initialize a boolean array
    let f = [true, false || true, true];

    // return an array
    return d
}
```
### Structs:
```rust
struct Point {
    u32 x
    u32 y
}
function main() -> u32 {
    Point p = Point {x: 1, y: 0}
    return p.x
}
```
```rust
struct Foo {
    bool x
}
function main() -> Foo {
    let f = Foo {x: true};
    f.x = false;
    return f
}
```

### Assert Equals:
This will enforce that the two values are equal in the constraint system.

```rust
function main() {
  assert_eq(45, 45);
  
  assert_eq(2fe, 2fe);
  
  assert_eq(true, true);
}
```

### Conditionals:

#### If Else Ternary Expression
```rust
function main() -> u32 {
  let y = if 3==3 ? 1 : 5;
  return y
}
```
#### If Else Conditional Statement
```rust
function main(a: private bool, b: private bool) -> u32 {
  let res = 0;
  if (a) {
    res = 1;
  } else if (b) {
    res = 2;
  } else {
    res = 3;
  }
  return res
}
```
#### For loop
```rust
function main() -> fe {
  let a = 1fe;
  for i in 0..4 {
    a = a + 1fe;
  }
  return a
}
```

### Functions:
```rust
function test1(a : u32) -> u32 {
  return a + 1
}

function test2(b: fe) -> fe {
  return b * 2fe
}

function test3(c: bool) -> bool {
  return c && true
}

function main() -> u32 {
  return test1(5)
}
```


#### Function Scope:
```rust
function foo() -> field {
  // return myGlobal <- not allowed
  return 42fe
}

function main() -> field {
  let myGlobal = 42fe;
  return foo()
}
```

#### Multiple returns:
Functions can return tuples whose types are specified in the function signature.
```rust
function test() -> (u32, u32[2]) {
    return 1, [2, 3]
}

function main() -> u32[3] {
    let (a, b) = test();
    // a, u32[2] b = test() <- explicit type also works
    return [a, ...b]
}
```

#### Parameters:
Main function arguments are allocated as public or private variables in the program's constaint system.
```rust
function main(a: private fe) -> fe {
  return a
}
```
```rust
function main(a: public fe) -> fe {
  return a
}
```
Function parameters are passed by value.
```rust
function test(a: u32) {
    a = 0;
}

function main() -> u32 {
    let a = 1;
    test(a);

    return a // <- returns 1
}
```

### Imports:
Both struct and function imports are supported.

import all: `*`
import alias: `symbol as alias`

/simple_import.leo
```rust
struct Point {
    u32 x
    u32 y
}

function test() -> (u32, u32[2]) {
    return 1, [2, 3]
}
```

/simple.leo
```rust
from "./simple_import" import {
    Point as Foo,
    test
};

// from "./simple_import" import * 

function main() -> (u32[3]) {
    let (a, b) = test();
    let p = Foo { x: 1, y: 2};
    return [a, ...b]
}
```



# Leo CLI

## Develop

### `leo new`

To setup a new package, run:
```
leo new {$NAME}
```
This will create a new directory with a given package name. The new package will have a directory structure as follows:
```
- inputs # Your program inputs
- outputs # Your program outputs
- src
    - lib.leo # Your program library
    - main.leo # Your program
- tests
    - tests.leo # Your program tests
- Leo.toml # Your program manifest
```

### `leo init`

To initialize an existing directory, run:
```
leo init
```
This will initialize the current directory with the same package directory setup.

### `leo build`

To compile your program and verify that it builds properly, run:
```
leo build
```

### `leo test`

To execute unit tests on your program, run:
```
leo test
```

## Run

### `leo setup`

To perform the program setup, producing a proving key and verification key, run:
```
leo setup
```
Leo uses cryptographic randomness from your machine to perform the setup. The proving key and verification key are stored in the `target` directory as `.leo.pk` and `.leo.vk`:

```
{$LIBRARY}/target/{$PROGRAM}.leo.pk
{$LIBRARY}/target/{$PROGRAM}.leo.vk
```

### `leo prove`

To execute the program and produce an execution proof, run:
```
leo prove
```
Leo starts by checking the `target` directory for an existing `.leo.pk` file. If it doesn't exist, it will proceed to run `leo setup` and then continue.

Once again, Leo uses cryptographic randomness from your machine to produce the proof. The proof is stored in the `target` directory as `.leo.proof`:

```
{$LIBRARY}/target/{$PROGRAM}.leo.proof
```

### `leo verify`

To verify the program proof, run:
```
leo verify
```
Leo starts by checking the `target` directory for an existing `.leo.proof` file. If it doesn't exist, it will proceed to run `leo prove` and then continue.

After the verifier is run, Leo will output either `true` or `false` based on the verification.

## Remote

To use remote compilation features, start by authentication with:
```
leo login
```
You will proceed to authenticate using your username and password. Next, Leo will parse your `Leo.toml` file for `remote = True` to confirm whether remote compilation is enabled.

If remote compilation is enabled, Leo syncs your workspace so when you run `leo build`, `leo test`, `leo setup` and `leo prove`, your program will run the program setup and execution performantly on remote machines.

This speeds up the testing cycle and helps the developer to iterate significantly faster.

## Publish

To package your program as a gadget and publish it online, run:
```
leo publish
```
Leo will proceed to snapshot your directory and upload your directory to the circuit manager. Leo will verify that `leo build` succeeds and that `leo test` passes without error.

If your gadget name has already been taken, `leo publish` will fail.

## Deploy

To deploy your program to Aleo, run:
```
leo deploy
```

## TODO

- Change `target` directory to some other directory to avoid collision.
- Figure out how `leo prove` should take in assignments.
- Come up with a serialization format for `.leo.pk`, `.leo.vk`, and `.leo.proof`.