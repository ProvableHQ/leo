// _dynamic_call with identifier-typed arguments.
program test.aleo {
    fn main(prog: identifier, net: identifier, func: identifier, arg: u64) -> u64 {
        let result: u64 = _dynamic_call::[u64](prog, net, func, arg);
        return result;
    }
}
