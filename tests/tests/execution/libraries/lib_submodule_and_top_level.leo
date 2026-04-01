/*
[case]
program = "submodule_toplevel.aleo"
function = "compute"
input = ["5u32"]
*/

// --- library: mixed_lib --- //

const BASE: u32 = 10u32;

// --- Next Module: ops.leo --- //
fn add_base(x: u32) -> u32 {
    return x + mixed_lib::BASE;
}

// --- Next Program --- //

program submodule_toplevel.aleo {
    fn compute(x: u32) -> u32 {
        return mixed_lib::ops::add_base(x);  // 5 + 10 = 15
    }

    @noupgrade
    constructor() {}
}
