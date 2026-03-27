/*
seed = 123456789

[case]
program = "var_caller.aleo"
function = "main"
input = ["3u32", "5u32"]
*/

interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program var_target.aleo: Adder {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import var_target.aleo;

program var_caller.aleo {
    fn main(x: u32, y: u32) -> u32 {
        let target: identifier = 'var_target';
        return var_target.aleo::Adder@(target)/sum(x, y);
    }

    @noupgrade
    constructor() {}
}
