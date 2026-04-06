// _dynamic_call returning a dyn record.
program test.aleo {
    fn main(prog: field, net: field, func: field, arg: u64) -> dyn record {
        let r: dyn record = _dynamic_call::[dyn record](prog, net, func, arg);
        return r;
    }
}
