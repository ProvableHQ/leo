/*
seed = 123456789

[case]
program = "combo_caller.aleo"
function = "main"
input = ["36022851160756008158975255649field", "39607543716895409581647182879453331879277field", "3u32", "5u32"]
*/

interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

program adder_target.aleo : Adder {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

interface Multiplier {
    fn product(a: u32, b: u32) -> u32;
}

program multiplier_target.aleo : Multiplier {
    fn product(a: u32, b: u32) -> u32 {
        return a * b;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import adder_target.aleo;
import multiplier_target.aleo;

program combo_caller.aleo {
    fn main(add_target: field, mul_target: field, x: u32, y: u32) -> u32 {
        let s: u32 = adder_target.aleo::Adder@(add_target)/sum(x, y);
        let p: u32 = multiplier_target.aleo::Multiplier@(mul_target)/product(x, y);
        return s + p;
    }

    @noupgrade
    constructor() {}
}
