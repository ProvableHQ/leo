// Test: calling a library function with the wrong number of arguments should fail.

// --- library: math_lib --- //

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::add(x); // missing second argument
    }

    @noupgrade
    constructor() {}
}
