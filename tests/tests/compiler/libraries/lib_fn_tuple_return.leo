// Test: library function returning a tuple, destructured at the call site.

// --- library: math_lib --- //

fn divmod(a: u32, b: u32) -> (u32, u32) {
    return (a / b, a % b);
}

// --- Next Program --- //

program main.aleo {
    fn compute(a: u32, b: u32) -> u32 {
        let (q, r): (u32, u32) = math_lib::divmod(a, b);
        return q + r;
    }

    @noupgrade
    constructor() {}
}
