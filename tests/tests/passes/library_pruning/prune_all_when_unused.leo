// Test: when the program references no library function, every library function is dropped.

// --- library: math_lib --- //

fn a(x: u32) -> u32 {
    return x + 1u32;
}

fn b(x: u32) -> u32 {
    return x - 1u32;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}
