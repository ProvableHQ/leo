// Test: a monomorphized generic library function is inlined.

// --- library: math_lib --- //

fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32) -> u32 {
        return math_lib::scale::[5u32](x);
    }

    @noupgrade
    constructor() {}
}
