// Test: passing a runtime variable as a const generic argument to a submodule generic
// function should fail with a non-const-argument error.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

// --- Next Program --- //

program non_const_generic.aleo {
    fn main(n: u32) -> u32 {
        return math_lib::ops::scale::[n](5u32);  // n is a runtime variable, not a const
    }

    @noupgrade
    constructor() {}
}
