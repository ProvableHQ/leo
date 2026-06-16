// PASS: _dynamic_call can be used in a ternary condition because the condition always runs.
program test.aleo {
    fn main(target: field, net: field, func: field, a: u64, b: u64) -> u64 {
        return _dynamic_call::[bool](target, net, func, a) ? a : b;
    }

    @noupgrade
    constructor() {}
}
