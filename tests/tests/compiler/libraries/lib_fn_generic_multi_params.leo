// Test: generic library function with two const parameters.

// --- library: math_lib --- //

fn affine::[A: u32, B: u32](x: u32) -> u32 {
    return x * A + B;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::affine::[3u32, 7u32](x);
    }

    @noupgrade
    constructor() {}
}
