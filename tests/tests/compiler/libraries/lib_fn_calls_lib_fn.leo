// Test: a library function calls another function in the same library.

// --- library: math_lib --- //

fn double(a: u32) -> u32 {
    return a + a;
}

fn quadruple(a: u32) -> u32 {
    return double(a) + double(a);
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::quadruple(x);
    }

    @noupgrade
    constructor() {}
}
