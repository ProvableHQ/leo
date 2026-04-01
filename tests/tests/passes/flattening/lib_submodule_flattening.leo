// Test: a submodule function body containing a ternary is flattened into guarded returns.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn max(a: u32, b: u32) -> u32 {
    return a > b ? a : b;
}

// --- Next Program --- //

program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        return math_lib::ops::max(a, b);
    }

    @noupgrade
    constructor() {}
}
