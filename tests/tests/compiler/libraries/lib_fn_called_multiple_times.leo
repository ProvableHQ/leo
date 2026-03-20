// Test: the same library function is called more than once within a single program function.
// Each call site must be independently inlined.

// --- library: math_lib --- //

fn square(x: u32) -> u32 {
    return x * x;
}

// --- Next Program --- //

program main.aleo {
    fn compute(a: u32, b: u32) -> u32 {
        let sa: u32 = math_lib::square(a);
        let sb: u32 = math_lib::square(b);
        return sa + sb;
    }

    @noupgrade
    constructor() {}
}
