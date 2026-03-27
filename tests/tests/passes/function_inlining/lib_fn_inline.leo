// Test: a library function is inlined at its single call site.

// --- library: math_lib --- //

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32, y: u32) -> u32 {
        return math_lib::add(x, y);
    }

    @noupgrade
    constructor() {}
}
