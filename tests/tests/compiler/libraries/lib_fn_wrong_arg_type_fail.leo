// Test: passing the wrong argument type to a library function should fail.

// --- library: math_lib --- //

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::add(x, true);  // bool where u32 expected
    }

    @noupgrade
    constructor() {}
}
