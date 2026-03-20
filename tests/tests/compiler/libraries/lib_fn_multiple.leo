// Test: multiple distinct non-generic library functions called from the same program.

// --- library: math_lib --- //

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

fn mul(a: u32, b: u32) -> u32 {
    return a * b;
}

fn sub(a: u32, b: u32) -> u32 {
    return a - b;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32, y: u32) -> u32 {
        let s: u32 = math_lib::add(x, y);
        let p: u32 = math_lib::mul(x, y);
        return math_lib::sub(p, s);
    }

    @noupgrade
    constructor() {}
}
