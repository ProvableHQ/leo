/*
[case]
program = "main.aleo"
function = "compute"
input = ["3u32"]
*/

// Test: fn in lib B calls fn from lib A. quadruple(3) = double(double(3)) = 12.

// --- library: base_lib --- //

fn double(x: u32) -> u32 {
    return x + x;
}

// --- Next Program --- //

// --- library: derived_lib --- //

fn quadruple(x: u32) -> u32 {
    return base_lib::double(base_lib::double(x));
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return derived_lib::quadruple(x);
    }

    @noupgrade
    constructor() {}
}
