// Test: a library with two named submodules; program calls functions from each independently.

// --- library: multi_lib --- //

// --- Next Module: algebra.leo --- //
fn square(x: u32) -> u32 {
    return x * x;
}

// --- Next Module: bits.leo --- //
fn double(x: u32) -> u32 {
    return x + x;
}

// --- Next Program --- //

program multi_mod.aleo {
    fn main() -> u32 {
        let s: u32 = multi_lib::algebra::square(3u32);   // 9
        let d: u32 = multi_lib::bits::double(4u32);      // 8
        return s + d;                                     // 17
    }

    @noupgrade
    constructor() {}
}
