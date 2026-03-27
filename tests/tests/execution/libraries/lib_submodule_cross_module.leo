/*
[case]
program = "submodule_cross.aleo"
function = "compute"
input = ["3u32", "4u32"]
*/

// --- library: utils_lib --- //

// --- Next Module: transform.leo --- //
fn double(x: u32) -> u32 {
    return x + x;
}

// --- Next Module: combine.leo --- //
fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program submodule_cross.aleo {
    fn compute(x: u32, y: u32) -> u32 {
        return utils_lib::combine::sum(utils_lib::transform::double(x), y);  // double(3) + 4 = 6 + 4 = 10
    }

    @noupgrade
    constructor() {}
}
