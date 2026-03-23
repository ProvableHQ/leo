// Test: a non-const runtime variable used as a library struct const argument.
// Expects the same non-const-argument error as for local generic structs/functions
// (see const_generics/non_const_arg_fail.leo).

// --- library: foo_lib --- //

struct Foo::[N: u32] {
    x: u32,
}

// --- Next Program --- //

program main.aleo {
    fn bar(n: u32) -> u32 {
        let f: foo_lib::Foo::[n] = foo_lib::Foo::[n] { x: 0u32 };
        return f.x;
    }

    @noupgrade
    constructor() {}
}
