// Test: submodule function parameters and body variables are SSA-renamed.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32, y: u32) -> u32 {
        return math_lib::ops::add(x, y);
    }

    @noupgrade
    constructor() {}
}
