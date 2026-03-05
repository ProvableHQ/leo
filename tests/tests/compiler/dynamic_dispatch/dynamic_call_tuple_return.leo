program test.aleo {
    fn main(prog: field, net: field, func: field, r: dyn record) -> (dyn record, dyn record) {
        let (a, b): (dyn record, dyn record) = _dynamic_call::[dyn record, dyn record](prog, net, func, r);
        return (a, b);
    }
}
