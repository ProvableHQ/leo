// _dynamic_call returning a u64.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64) -> u64 {
        let result: u64 = _dynamic_call::[u64](target, net, func, x);
        return result;
    }
}
