// Test: a library function is defined but never called. Should compile without errors.

// --- library: math_lib --- //

fn used(x: u32) -> u32 {
    return x + 1u32;
}

fn unused(x: u32) -> u32 {
    return x * 99u32;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::used(x);
    }

    @noupgrade
    constructor() {}
}
