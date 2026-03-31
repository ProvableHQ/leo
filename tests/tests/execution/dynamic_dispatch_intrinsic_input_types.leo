/*
seed = 123456789

[case]
program = "caller.aleo"
function = "run"
input = ["97field", "1868917857field", "102field", "5u64"]
*/

program a.aleo {
    fn f(x: u64) -> u64 {
        return x + x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import a.aleo;

program caller.aleo {
    fn run(prog: field, net: field, func: field, x: u64) -> u64 {
        let result: u64 = _dynamic_call::[private u64, u64](prog, net, func, x);
        return result;
    }

    @noupgrade
    constructor() {}
}
