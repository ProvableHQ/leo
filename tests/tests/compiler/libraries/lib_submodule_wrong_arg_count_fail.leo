// Test: calling a submodule function with the wrong number of arguments should fail.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program wrong_arg_count.aleo {
    fn main() -> u32 {
        return math_lib::ops::add(1u32);  // missing second argument
    }

    @noupgrade
    constructor() {}
}
