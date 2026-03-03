program test.aleo {
    fn main(prog: field, net: field, func: field, arg: u64) -> Final {
        let f: Final = _dynamic_call::[Final](prog, net, func, arg);
        return final { f.run(); };
    }
}
