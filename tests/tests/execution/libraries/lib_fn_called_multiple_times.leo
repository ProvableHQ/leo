/*
[case]
program = "main.aleo"
function = "compute"
input = ["2u32", "3u32"]
*/

// Test: same library function called twice. square(2) + square(3) = 4 + 9 = 13.

// --- library: math_lib --- //

fn square(x: u32) -> u32 {
    return x * x;
}

// --- Next Program --- //

program main.aleo {
    fn compute(a: u32, b: u32) -> u32 {
        let sa: u32 = math_lib::square(a);
        let sb: u32 = math_lib::square(b);
        return sa + sb;
    }

    @noupgrade
    constructor() {}
}
