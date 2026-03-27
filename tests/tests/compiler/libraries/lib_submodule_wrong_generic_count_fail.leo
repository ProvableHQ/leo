// Test: providing the wrong number of const generic arguments to a submodule generic
// function should fail.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

// --- Next Program --- //

program wrong_generic_count.aleo {
    fn main() -> u32 {
        return math_lib::ops::scale::[3u32, 4u32](5u32);  // 2 const args, expects 1
    }

    @noupgrade
    constructor() {}
}
