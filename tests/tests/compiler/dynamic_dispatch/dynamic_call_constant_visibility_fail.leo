// FAIL: constant visibility is not allowed in _dynamic_call type annotations.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64) -> u64 {
        let result: u64 = _dynamic_call::[constant u64](target, net, func, x);
        return result;
    }
}
