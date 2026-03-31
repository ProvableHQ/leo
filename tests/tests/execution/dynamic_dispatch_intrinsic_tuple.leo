/*
seed = 123456789

[case]
program = "caller.aleo"
function = "main"
input = ["512089499423744773612641field", "3u32", "5u32"]
*/

interface Math {
    fn sum_and_product(a: u32, b: u32) -> (u32, u32);
}

program adder_impl.aleo : Math {
    fn sum_and_product(a: u32, b: u32) -> (u32, u32) {
        return (a + b, a * b);
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import adder_impl.aleo;

program caller.aleo {
    fn main(target: field, x: u32, y: u32) -> (u32, u32) {
        let (s, p): (u32, u32) = _dynamic_call::[(u32, u32)](target, 'aleo', 'sum_and_product', x, y);
        return (s, p);
    }

    @noupgrade
    constructor() {}
}
