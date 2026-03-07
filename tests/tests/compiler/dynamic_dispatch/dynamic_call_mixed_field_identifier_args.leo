// _dynamic_call with mixed field and identifier types for the first three target arguments.
program test.aleo {
    fn main(prog: identifier, net: field, func: identifier, arg: u64) -> u64 {
        let result: u64 = _dynamic_call::[public u64](prog, net, func, arg);
        return result;
    }
}
