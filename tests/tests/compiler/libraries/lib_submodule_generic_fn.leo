// Test: generic function inside a library submodule is correctly monomorphized.

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

fn double(x: u32) -> u32 {
    return x + x;
}

// --- Next Program --- //

program submodule_generic.aleo {
    fn main() -> u32 {
        let scaled: u32 = math_lib::ops::scale::[3u32](7u32);
        let doubled: u32 = math_lib::ops::double(scaled);
        return doubled;
    }
}
