// Test: a submodule contains an unreferenced function; dead-code elimination removes it
// and the program only sees the used function.

// --- library: util_lib --- //

// --- Next Module: math.leo --- //
fn used(x: u32) -> u32 {
    return x + x;
}

fn unused(x: u32) -> u32 {
    return x * x * x;
}

// --- Next Program --- //

program dead_code.aleo {
    fn main() -> u32 {
        return util_lib::math::used(5u32);  // 5 + 5 = 10
    }

    @noupgrade
    constructor() {}
}
