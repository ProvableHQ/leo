// Test: passing the wrong number of const arguments to a library generic struct.
// Expects a type-checker error for incorrect const argument arity.

// --- library: foo_lib --- //

struct Foo::[N: u32] {
    x: u32,
}

// --- Next Program --- //

program main.aleo {
    fn compute() -> u32 {
        let f: foo_lib::Foo::[1u32, 2u32] = foo_lib::Foo::[1u32, 2u32] { x: 0u32 };
        return f.x;
    }

    @noupgrade
    constructor() {}
}
