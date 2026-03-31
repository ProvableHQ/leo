// Calls a function on another program via _dynamic_call.
// The target program is NOT a dependency — it must be loaded via --with.
program test_dynamic_call_with_flag.aleo {
    fn main(prog: field, net: field, func: field, x: u64) -> u64 {
        let result: u64 = _dynamic_call::[private u64](prog, net, func, x);
        return result;
    }

    @noupgrade
    constructor() {}
}
