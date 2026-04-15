// Test: two library submodules define a generic function with the same name.
// Monomorphized versions must live at lib/[a, "compute::[3u32]"] and
// lib/[b, "compute::[3u32]"] and not collide in the symbol table.

// --- library: arith_lib --- //

// --- Next Module: a.leo --- //
fn compute::[N: u32](x: u32) -> u32 {
    return x * N;
}

// --- Next Module: b.leo --- //
fn compute::[N: u32](x: u32) -> u32 {
    return x + N;
}

// --- Next Program --- //

program multi_fn.aleo {
    fn main(x: u32) -> u32 {
        let r1: u32 = arith_lib::a::compute::[3u32](x);  // x * 3
        let r2: u32 = arith_lib::b::compute::[3u32](x);  // x + 3
        return r1 + r2;
    }

    @noupgrade
    constructor() {}
}
