// _dynamic_call with explicit public visibility on return type.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64) -> u64 {
        let result: u64 = _dynamic_call::[public u64](target, net, func, x);
        return result;
    }
}
