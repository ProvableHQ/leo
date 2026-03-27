// Test: a generic submodule function is monomorphized and const-propagated by the
// ConstPropUnrollAndMorphing pass when called with a concrete const argument.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

// --- Next Program --- //

program test.aleo {
    fn main(x: u32) -> u32 {
        return math_lib::ops::scale::[4u32](x);
    }

    @noupgrade
    constructor() {}
}
