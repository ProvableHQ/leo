// Test: library function with a ternary conditional expression.

// --- library: math_lib --- //

fn min(a: u32, b: u32) -> u32 {
    return a < b ? a : b;
}

fn max(a: u32, b: u32) -> u32 {
    return a > b ? a : b;
}

// --- Next Program --- //

program main.aleo {
    fn compute(a: u32, b: u32) -> (u32, u32) {
        return (math_lib::min(a, b), math_lib::max(a, b));
    }

    @noupgrade
    constructor() {}
}
