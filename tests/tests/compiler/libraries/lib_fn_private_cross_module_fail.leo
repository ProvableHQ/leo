// Test: calling a non-`export` library function from another compilation unit fails.
// The helper is not `export`, so it must remain hidden from external code.

// --- library: math_lib --- //

fn private_helper(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32, y: u32) -> u32 {
        return math_lib::private_helper(x, y);
    }

    @noupgrade
    constructor() {}
}
