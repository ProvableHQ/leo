/*
seed = 123456789

[case]
program = "caller.aleo"
function = "main"
input = ["8392569366120525161field", "5u64"]
*/

interface Base {
    fn get_value() -> u64;
}

interface Extended: Base {
    fn double_val(x: u64) -> u64;
}

program impl_ext.aleo: Extended {
    fn get_value() -> u64 {
        return 0u64;
    }

    fn double_val(x: u64) -> u64 {
        return x * 2u64;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import impl_ext.aleo;

program caller.aleo {
    fn main(target: field, x: u64) -> u64 {
        return impl_ext.aleo::Extended@(target)/double_val(x);
    }

    @noupgrade
    constructor() {}
}
