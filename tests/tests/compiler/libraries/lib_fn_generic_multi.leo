// Test: a generic library function called at two distinct const arguments,
// producing two independent monomorphized copies.

// --- library: math_lib --- //

fn scale::[N: u32](a: u32) -> u32 {
    return a * N;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> (u32, u32) {
        let a: u32 = math_lib::scale::[3u32](x);
        let b: u32 = math_lib::scale::[7u32](x);
        return (a, b);
    }

    @noupgrade
    constructor() {}
}
