// Test: calling a function that does not exist in a valid submodule should fail.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program unknown_fn.aleo {
    fn main() -> u32 {
        return math_lib::ops::nonexistent(1u32, 2u32);
    }

    @noupgrade
    constructor() {}
}
