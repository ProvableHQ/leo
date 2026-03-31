// FAIL: First 3 args to _dynamic_call must be field or identifier.
program test.aleo {
    fn main(target: u64, net: field, func: field) -> u64 {
        let result: u64 = _dynamic_call::[u64](target, net, func);
        return result;
    }
}
