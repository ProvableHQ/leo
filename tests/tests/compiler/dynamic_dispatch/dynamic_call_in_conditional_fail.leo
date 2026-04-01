// FAIL: _dynamic_call cannot be used inside a conditional.
program test.aleo {
    fn main(target: field, net: field, func: field, cond: bool) -> u64 {
        if cond {
            return _dynamic_call::[u64](target, net, func);
        }
        return 0u64;
    }
}
