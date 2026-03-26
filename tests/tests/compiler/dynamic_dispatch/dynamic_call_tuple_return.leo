// _dynamic_call returning a tuple of two values.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64) -> (u64, u64) {
        let (a, b): (u64, u64) = _dynamic_call::[(u64, u64)](target, net, func, x);
        return (a, b);
    }
}
