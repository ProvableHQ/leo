// Test: one generic library function whose body calls a non-generic function in the same library.
// scale::[N] calls `add`, which is a concrete helper.

// --- library: math_lib --- //

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

fn scale::[N: u32](x: u32) -> u32 {
    // Use add to perform x * N via repeated addition (N=2 for simplicity).
    return add(x, x);
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::scale::[2u32](x);
    }

    @noupgrade
    constructor() {}
}
