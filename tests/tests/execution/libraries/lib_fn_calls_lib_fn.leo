/*
[case]
program = "main.aleo"
function = "compute"
input = ["5u32"]
*/

// --- library: math_lib --- //

fn double(a: u32) -> u32 {
    return a + a;
}

fn quadruple(a: u32) -> u32 {
    return double(a) + double(a);
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::quadruple(x);
    }

    @noupgrade
    constructor() {}
}
