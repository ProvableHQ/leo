/*
[case]
program = "submodule_basic.aleo"
function = "compute"
input = ["3u32", "4u32"]
*/

// --- library: math_lib --- //

// --- Next Module: arith.leo --- //
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

fn mul(a: u32, b: u32) -> u32 {
    return a * b;
}

// --- Next Program --- //

program submodule_basic.aleo {
    fn compute(x: u32, y: u32) -> u32 {
        return math_lib::arith::add(x, math_lib::arith::mul(x, y));  // 3 + (3*4) = 15
    }

    @noupgrade
    constructor() {}
}
