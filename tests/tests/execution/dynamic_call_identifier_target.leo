/*
seed = 123456789

[case]
program = "id_caller.aleo"
function = "main"
input = ["3u32", "5u32"]
*/

interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program id_target.aleo : Adder {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import id_target.aleo;

program id_caller.aleo {
    fn main(x: u32, y: u32) -> u32 {
        return id_target.aleo::Adder@('id_target')/sum(x, y);
    }

    @noupgrade
    constructor() {}
}
