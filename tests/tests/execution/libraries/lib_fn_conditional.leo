/*
[case]
program = "main.aleo"
function = "compute"
input = ["10u32", "3u32"]
*/

// Test: library function with a ternary conditional. min(10, 3) = 3.

// --- library: math_lib --- //

fn min(a: u32, b: u32) -> u32 {
    return a < b ? a : b;
}

// --- Next Program --- //

program main.aleo {
    fn compute(a: u32, b: u32) -> u32 {
        return math_lib::min(a, b);
    }

    @noupgrade
    constructor() {}
}
