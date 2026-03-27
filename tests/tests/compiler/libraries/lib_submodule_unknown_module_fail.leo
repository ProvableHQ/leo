// Test: referencing a non-existent submodule path should fail.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program unknown_module.aleo {
    fn main() -> u32 {
        return math_lib::nonexistent::add(1u32, 2u32);
    }

    @noupgrade
    constructor() {}
}
