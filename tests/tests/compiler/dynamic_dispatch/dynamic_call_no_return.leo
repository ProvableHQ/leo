program test.aleo {
    fn main(prog: field, net: field, func: field, arg: u64) {
        _dynamic_call(prog, net, func, arg);
    }
}
