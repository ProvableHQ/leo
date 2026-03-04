program test.aleo {
    fn main(cond: bool, prog: field, net: field, func: field) -> u64 {
        if cond {
            return _dynamic_call::[u64](prog, net, func);
        }
        return 0u64;
    }
}
