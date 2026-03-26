/*
[case]
program = "main.aleo"
function = "compute"
input = ["5u32"]
*/

// --- library: math_lib --- //

const OFFSET: u32 = 10u32;
const SCALE: u32 = OFFSET + OFFSET;

// --- Next Program --- //

// --- library: config_lib --- //

const BASE: u32 = math_lib::OFFSET + 5u32;
const FACTOR: u32 = math_lib::SCALE + BASE;

// --- Next Program --- //

program helper.aleo {
    fn add_offset(x: u32) -> u32 {
        return x + config_lib::BASE + math_lib::OFFSET;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import helper.aleo;

program main.aleo {
    fn compute(x: u32) -> u32 {
        let a: u32 = helper.aleo::add_offset(x);
        return a + config_lib::FACTOR + math_lib::SCALE;
    }

    @noupgrade
    constructor() {}
}
