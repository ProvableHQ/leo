// _dynamic_call with mixed field and identifier arguments.
program test.aleo {
    fn main(prog: field, net: identifier, func: identifier, x: u64) -> u64 {
        let result: u64 = _dynamic_call::[u64](prog, net, func, x);
        return result;
    }
}
