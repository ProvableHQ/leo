// Test: Functions called by another Variant::Function should be converted to inline
program test.aleo {
    fn main(x: u32) -> u32 {
        return outer(x);
    }
}

// This is a Variant::Function that calls helper
fn outer(x: u32) -> u32 {
    return helper(x);
}

// helper is called from outer (a Variant::Function), so it should become inline
fn helper(x: u32) -> u32 {
    return x * 2u32;
}
