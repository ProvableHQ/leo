// _dynamic_call with explicit input type annotations.
// The last type param is the return type, all preceding are input types.
program test.aleo {
    fn main(target: field, net: field, func: field, x: u64) -> u64 {
        let result: u64 = _dynamic_call::[public u64, u64](target, net, func, x);
        return result;
    }
}
