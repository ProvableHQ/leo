// FAIL: _dynamic_get must be used in finalize context.
program test.aleo {
    fn main(prog: field, net: field, m: field, key: address) -> u64 {
        let val: u64 = _dynamic_get::[u64](prog, net, m, key);
        return val;
    }
}
