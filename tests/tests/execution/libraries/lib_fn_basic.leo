/*
[case]
program = "main.aleo"
function = "compute"
input = ["3u32", "4u32"]
*/

// --- library: math_lib --- //

fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32, y: u32) -> u32 {
        return math_lib::add(x, y);
    }

    @noupgrade
    constructor() {}
}
