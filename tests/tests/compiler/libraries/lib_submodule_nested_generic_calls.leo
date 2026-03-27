// Test: a generic submodule function calls another generic function in the same module.
// Both must be correctly monomorphized when the program supplies a concrete const argument.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

fn double_scale::[N: u32](x: u32) -> u32 {
    return scale::[N](x) + scale::[N](x);
}

// --- Next Program --- //

program nested_generic.aleo {
    fn main() -> u32 {
        return math_lib::ops::double_scale::[4u32](2u32);  // scale::[4](2) + scale::[4](2) = 8 + 8 = 16
    }

    @noupgrade
    constructor() {}
}
