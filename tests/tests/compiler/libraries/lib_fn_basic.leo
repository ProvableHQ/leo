// Test: basic non-generic library function call.

// --- library: math_lib --- //

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32, y: u32) -> u32 {
        return math_lib::add(x, y);
    }

    @noupgrade
    constructor() {}
}
