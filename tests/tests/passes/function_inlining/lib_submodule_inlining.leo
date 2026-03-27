// Test: a non-generic submodule function is inlined into the calling program function.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn double(x: u32) -> u32 {
    return x + x;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32) -> u32 {
        return math_lib::ops::double(x);
    }

    @noupgrade
    constructor() {}
}
