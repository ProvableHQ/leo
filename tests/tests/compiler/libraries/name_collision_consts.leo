// Test: library consts, module consts, and program consts all share the same names.
// Each should resolve correctly via its qualifier.

// --- library: libA --- //

const X: u32 = 10u32;
const Y: u32 = 20u32;

// --- Next Program --- //

// --- library: libB --- //

const X: u32 = 30u32;
const Y: u32 = 40u32;

// --- Next Program --- //

// Program-level consts with the same names.
const X: u32 = 50u32;
const Y: u32 = 60u32;

program consts.aleo {
    fn foo() -> u32 {
        // libA::X=10, libA::Y=20, libB::X=30, libB::Y=40,
        // X=50, Y=60, mod1::X=70, mod1::Y=80
        return libA::X + libA::Y + libB::X + libB::Y + X + Y + mod1::X + mod1::Y;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: mod1.leo --- //
const X: u32 = 70u32;
const Y: u32 = 80u32;
