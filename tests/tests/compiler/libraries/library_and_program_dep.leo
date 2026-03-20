// Test: a program depends on both a library and another program.
// Library consts should be resolved at compile time and NOT appear in bytecode imports.
// The program import should appear normally.

// --- library: constants --- //

const MULTIPLIER: u32 = 3u32;
const ADDEND: u32 = 7u32;

// --- Next Program --- //

program helper.aleo {
    fn scale(x: u32) -> u32 {
        return x * constants::MULTIPLIER;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import helper.aleo;

program main.aleo {
    fn compute(x: u32) -> u32 {
        let scaled: u32 = helper.aleo/scale(x);
        return scaled + constants::ADDEND;
    }

    @noupgrade
    constructor() {}
}
