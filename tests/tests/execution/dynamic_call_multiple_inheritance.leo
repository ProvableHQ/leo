/*
seed = 123456789

[case]
program = "caller.aleo"
function = "main"
input = ["2000349607123751821667field", "3u32", "5u32"]
*/

interface Adder {
    fn sum(a: u32, b: u32) -> u32;
}

interface Multiplier {
    fn product(a: u32, b: u32) -> u32;
}

interface Calculator : Adder + Multiplier {}

program calc_impl.aleo : Calculator {
    fn sum(a: u32, b: u32) -> u32 {
        return a + b;
    }

    fn product(a: u32, b: u32) -> u32 {
        return a * b;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import calc_impl.aleo;

program caller.aleo {
    fn main(target: field, x: u32, y: u32) -> u32 {
        let s: u32 = calc_impl.aleo/Adder@(target)/sum(x, y);
        let p: u32 = calc_impl.aleo/Multiplier@(target)/product(x, y);
        return s + p;
    }

    @noupgrade
    constructor() {}
}
