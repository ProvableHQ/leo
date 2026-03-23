/*
seed = 123456789

[case]
program = "caller.aleo"
function = "main"
input = ["8392569366120525161field", "7u64"]
*/

interface Base {
    fn double_val(x: u64) -> u64;
}

interface Extended : Base {
    fn triple_val(x: u64) -> u64;
}

program impl_ext.aleo : Extended {
    fn double_val(x: u64) -> u64 {
        return x * 2u64;
    }

    fn triple_val(x: u64) -> u64 {
        return x * 3u64;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import impl_ext.aleo;

program caller.aleo {
    fn main(target: field, x: u64) -> u64 {
        return impl_ext.aleo/Base@(target)/double_val(x);
    }

    @noupgrade
    constructor() {}
}
