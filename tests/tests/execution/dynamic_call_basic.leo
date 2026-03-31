/*
seed = 123456789

[case]
program = "caller.aleo"
function = "main"
input = ["512089499423744773612641field", "3u32", "5u32"]
*/

interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program adder_impl.aleo: Adder {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import adder_impl.aleo;

program caller.aleo {
    fn main(target: field, x: u32, y: u32) -> u32 {
        return adder_impl.aleo::Adder@(target)::sum(x, y);
    }

    @noupgrade
    constructor() {}
}
