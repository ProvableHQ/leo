// Test: non-const expression used as a const argument to a generic library function.
// Expects the same non-const-argument error as for local generic functions.

// --- library: math_lib --- //

fn scale::[N: u32](a: u32) -> u32 {
    return a * N;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32, n: u32) -> u32 {
        return math_lib::scale::[n](x);  // n is a runtime variable, not a const
    }

    @noupgrade
    constructor() {}
}
