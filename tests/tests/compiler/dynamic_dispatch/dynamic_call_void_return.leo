// _dynamic_call with explicit () void return and no input annotations.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64) {
        _dynamic_call::[()](target, net, func, x);
    }
}
