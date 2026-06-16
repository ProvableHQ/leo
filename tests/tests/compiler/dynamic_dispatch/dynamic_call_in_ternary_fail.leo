// FAIL: _dynamic_call cannot be used in a ternary arm.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64, pick: bool) -> u64 {
        return pick ? _dynamic_call::[u64](target, net, func, x) : x;
    }

    @noupgrade
    constructor() {}
}
