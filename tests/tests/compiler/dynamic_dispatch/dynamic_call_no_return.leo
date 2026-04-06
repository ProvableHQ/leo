// _dynamic_call with no return value.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64) {
        _dynamic_call(target, net, func, x);
    }
}
