/*
seed = 123456789
min_height = 16

[case]
program = "multi_caller.aleo"
function = "main"
input = ["2000349607123836232045field", "3u32", "5u32"]
*/

interface MathOps {
    fn compute(a: u32, b: u32) -> (u32, Final);
}

program math_impl.aleo: MathOps {
    storage total: u32;

    fn compute(a: u32, b: u32) -> (u32, Final) {
        let result: u32 = a + b;
        return (
            result,
            final {
                total = total.unwrap_or(0u32) + result;
            },
        );
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import math_impl.aleo;

program multi_caller.aleo {
    fn main(target: field, a: u32, b: u32) -> (u32, Final) {
        let (r1, f1): (u32, Final) = math_impl.aleo::MathOps@(target)/compute(a, b);
        let (r2, f2): (u32, Final) = math_impl.aleo::MathOps@(target)/compute(a, r1);
        return (
            r2,
            final {
                f1.run();
                f2.run();
            },
        );
    }

    @noupgrade
    constructor() {}
}
