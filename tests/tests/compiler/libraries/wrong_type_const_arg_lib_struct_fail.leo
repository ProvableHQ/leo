// Test: passing a const argument of the wrong type to a library generic struct.
// Foo::[N: u32] expects a u32 const arg but receives a bool literal.
// Expects a type mismatch error.

// --- library: foo_lib --- //

struct Foo::[N: u32] {
    x: u32,
}

// --- Next Program --- //

program main.aleo {
    fn compute() -> u32 {
        let f: foo_lib::Foo::[true] = foo_lib::Foo::[true] { x: 0u32 };
        return f.x;
    }

    @noupgrade
    constructor() {}
}
