/*
[case]
program = "main.aleo"
function = "compute"
input = ["5u32"]
*/

// Test: function body uses a library-level const. shift(5) = 5 + 10 = 15.

// --- library: math_lib --- //

const OFFSET: u32 = 10u32;

fn shift(x: u32) -> u32 {
    return x + OFFSET;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::shift(x);
    }

    @noupgrade
    constructor() {}
}
