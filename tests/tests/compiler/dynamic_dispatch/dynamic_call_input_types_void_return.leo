// _dynamic_call with input type annotations but void return.
// The () at the end means void return, the preceding type is the input.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64) {
        _dynamic_call::[public u64, ()](target, net, func, x);
    }
}
