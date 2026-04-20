/*
[case]
program = "parent.aleo"
function = "use_deep"
input = []

[case]
program = "parent.aleo"
function = "scale"
input = ["3u32"]

[case]
program = "parent.aleo"
function = "combined_consts"
input = []
*/

// Test: accessing constants from a 2-level deep submodule (3-segment locator path).
// Also mixes a top-level constant with a 1-level constant and a 2-level constant.

program child.aleo {
    const TOP_CONST: u32 = 5u32;

    fn get_top() -> u32 {
        return TOP_CONST;
    }

    @noupgrade
    constructor() {}
}

// --- Next Module: math.leo --- //

const MID_CONST: u32 = 20u32;

// --- Next Module: math/ops.leo --- //

const FACTOR: u32 = 7u32;

// --- Next Program --- //

import child.aleo;

program parent.aleo {
    // Access a constant 2 levels deep: child.aleo::math::ops::FACTOR
    fn use_deep() -> u32 {
        return child.aleo::math::ops::FACTOR;
    }

    // Multiply input by the deep constant.
    fn scale(x: u32) -> u32 {
        return x * child.aleo::math::ops::FACTOR;
    }

    // Sum constants from all three depths: top-level, 1-level, 2-level.
    fn combined_consts() -> u32 {
        return child.aleo::TOP_CONST + child.aleo::math::MID_CONST + child.aleo::math::ops::FACTOR;
    }

    @noupgrade
    constructor() {}
}
