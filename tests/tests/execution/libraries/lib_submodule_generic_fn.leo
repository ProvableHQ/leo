/*
[case]
program = "submodule_generic.aleo"
function = "compute"
input = ["7u32"]
*/

// --- library: math_lib --- //

// --- Next Module: ops.leo --- //
fn scale::[N: u32](x: u32) -> u32 {
    return x * N;
}

// --- Next Program --- //

program submodule_generic.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::ops::scale::[3u32](x);  // 3 * 7 = 21
    }

    @noupgrade
    constructor() {}
}
