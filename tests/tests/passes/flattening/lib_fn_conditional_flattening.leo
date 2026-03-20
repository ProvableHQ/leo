// Test: a library function with a ternary is flattened into guarded returns.

// --- library: math_lib --- //

fn min(a: u32, b: u32) -> u32 {
    return a < b ? a : b;
}

// --- Next Program --- //

program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        return math_lib::min(a, b);
    }

    @noupgrade
    constructor() {}
}
