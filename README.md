# The Leo Language
* All code examples can be copied and pasted into simple.program directly and executed with cargo run
* Programs should be formatted:
    1. Import definitions
    2. Stuct definitions
    3. Function definitions

### Integers:
Currently, all integers are parsed as u32.
You can choose to explicitly add the type or let the compiler interpret implicitly.
```rust
function main() -> (u32) {
    a = 1u32 + 1u32
    b = 1 - 1
    c = 2 * 2
    d = 4 / 2
    e = 2 ** 3
    return a
}
```

### Field Elements:
Field elements must have the type added explicitly.
```rust
function main() -> (fe) {
    f = 21888242871839275222246405745257275088548364400416034343698204186575808495617fe
    a = 1fe + 1fe
    b = 1fe - 1fe
    c = 2fe * 2fe
    d = 4fe / 2fe
    return a
}
```


### Booleans:
```rust
function main() -> (bool) {
    a = true || false
    b = false && false
    c = 1 == 1
    return a
}
```

### Arrays:
Leo supports static arrays with fixed length.
Array type must be explicitly stated
```rust
function main() -> (u32[2]) {
    // initialize an integer array with integer values
    u32[3] a = [1, 2, 3]

    // set a member to a value
    a[2] = 4

    // initialize an array of 4 values all equal to 42
    u32[4] b = [42; 4]

    // initialize an array of 5 values copying all elements of b using a spread
    u32[5] c = [1, ...b]

    // initialize an array copying a slice from `c`
    d = c[1..3]

    // initialize a field array
    fe[2] e = [5fe; 2]

    // initialize a boolean array
    bool[3] f = [true, false || true, true]

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
function main() -> (u32) {
    Point p = Point {x: 1, y: 0}
    return p.x
}
```
```rust
struct Foo {
    bool x
}
function main() -> (Foo) {
    Foo f = Foo {x: true}
    f.x = false
    return f
}
```

### Conditionals:
```rust
function main() -> (u32) {
  y = if 3==3 then 1 else 5 fi
  return y
}
```
```rust
function main() -> (fe) {
  a = 1fe
  for i in 0..4 do
      a = a + 1fe
  endfor
  return a
}
```

### Functions:
```rust
function test1(a : u32) -> (u32) {
    return a + 1
}

function test2(b: fe) -> (fe) {
    return b * 2fe
}

function test3(c: bool) -> (bool) {
  return c && true
}

function main() -> (u32) {
  return test1(5)
}
```


#### Function Scope:
```rust
function foo() -> (field) {
    // return myGlobal <- not allowed
    return 42fe
}

function main() -> (field) {
    myGlobal = 42fe
    return foo()
}
```

### Parameters:
Main function arguments are allocated as public or private variables in the program's constaint system.
```rust
function main(a: private fe) -> (fe) {
  return a
}
```
```rust
function main(a: public fe) -> (fe) {
  return a
}
```

### Imports:
Note that there can only be one main function across all imported files.
/simple_import.leo
```rust
struct Point {
    u32 x
    u32 y
}
```

/simple.leo
```rust
from "./simple_import" import Point

function main() -> (Point) {
    Point p = Point { x: 1, y: 2}
    return p
}
```

Default exports are not currently supported.
There should only be one main function across all files.

# Leo CLI

## Develop

To compile your program and verify that it builds properly, run:
```
leo build
```

To execute unit tests on your program, run:
```
leo test
```

## Run

To perform the program setup, producing a proving key and verification key, run:
```
leo setup
```
Leo uses cryptographic randomness from your machine to perform the setup. The proving key and verification key are stored in the `target` directory as `.leo.pk` and `.leo.vk`:

```
{$LIBRARY}/target/{$PROGRAM}.leo.pk
{$LIBRARY}/target/{$PROGRAM}.leo.vk
```

To execute the program and produce an execution proof, run:
```
leo prove
```
Leo starts by checking the `target` directory for an existing `.leo.pk` file. If it doesn't exist, it will proceed to run `leo setup` and then continue.

Once again, Leo uses cryptographic randomness from your machine to produce the proof. The proof is stored in the `target` directory as `.leo.proof`:

```
{$LIBRARY}/target/{$PROGRAM}.leo.proof
```

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

To deploy your program to the blockchain, run:
```
leo deploy
```

## TODO

- Change `target` directory to some other directory to avoid collision.
- Figure out how `leo prove` should take in assignments.
- Come up with a serialization format for `.leo.pk`, `.leo.vk`, and `.leo.proof`.