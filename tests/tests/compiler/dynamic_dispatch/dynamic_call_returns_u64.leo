program test.aleo {
    fn main(prog: field, net: field, func: field, arg: u64) -> u64 {
        let result: u64 = _dynamic_call::[public u64](prog, net, func, arg);
        return result;
    }
}
