// Test: passing the wrong argument type to a submodule function should fail.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program wrong_arg_type.aleo {
    fn main() -> u32 {
        return math_lib::ops::add(1u32, true);  // bool where u32 expected
    }

    @noupgrade
    constructor() {}
}
