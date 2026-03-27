// Test: a const declared inside a submodule is used by a function within that same module.

// --- library: cfg_lib --- //

// --- Next Module: math.leo --- //
const FACTOR: u32 = 5u32;

fn scaled(x: u32) -> u32 {
    return x * FACTOR;
}

// --- Next Program --- //

program const_in_mod.aleo {
    fn main() -> u32 {
        return cfg_lib::math::scaled(6u32);  // 6 * 5 = 30
    }

    @noupgrade
    constructor() {}
}
