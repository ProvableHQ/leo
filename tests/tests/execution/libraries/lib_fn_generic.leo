/*
[case]
program = "main.aleo"
function = "compute"
input = ["3u32"]
*/

// --- library: math_lib --- //

fn scale::[N: u32](a: u32) -> u32 {
    return a * N;
}

// --- Next Program --- //

program main.aleo {
    fn compute(x: u32) -> u32 {
        return math_lib::scale::[5u32](x);
    }

    @noupgrade
    constructor() {}
}
