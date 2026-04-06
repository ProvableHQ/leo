// FAIL: Number of input type annotations must match the number of call arguments.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64, y: u64) -> u64 {
        let result: u64 = _dynamic_call::[public u64, u64](target, net, func, x, y);
        return result;
    }
}
