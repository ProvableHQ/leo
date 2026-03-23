/*
seed = 123456789
min_height = 16

[case]
program = "counter_caller.aleo"
function = "main"
input = ["33560297434234537552366628707field", "42u32"]
*/

interface Counter {
    fn increment(val: u32) -> (u32, Final);
}

program counter_impl.aleo : Counter {
    storage count: u32;

    fn increment(val: u32) -> (u32, Final) {
        return (val, final {
            count = count.unwrap_or(0u32) + val;
        });
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import counter_impl.aleo;

program counter_caller.aleo {
    fn main(target: field, val: u32) -> (u32, Final) {
        let (result, f): (u32, Final) = counter_impl.aleo/Counter@(target)/increment(val);
        return (result, final {
            f.run();
        });
    }

    @noupgrade
    constructor() {}
}
