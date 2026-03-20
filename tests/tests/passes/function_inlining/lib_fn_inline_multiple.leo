// Test: the same library function called twice; both call sites get inlined independently.

// --- library: math_lib --- //

fn square(x: u32) -> u32 {
    return x * x;
}

// --- Next Program --- //

program test.aleo {
    fn main(a: u32, b: u32) -> u32 {
        let sa: u32 = math_lib::square(a);
        let sb: u32 = math_lib::square(b);
        return sa + sb;
    }

    @noupgrade
    constructor() {}
}
