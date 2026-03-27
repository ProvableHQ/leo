// Test: library function body references a const from the same library.

// --- library: math_lib --- //

const OFFSET: u32 = 10u32;

fn shift(x: u32) -> u32 {
    return x + OFFSET;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::shift(x);
    }

    @noupgrade
    constructor() {}
}
