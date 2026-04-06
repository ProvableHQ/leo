// FAIL: _dynamic_get_or_use must be used in finalize context.
program test.aleo {
    fn main(prog: field, net: field, m: field, key: address) -> u64 {
        let val: u64 = _dynamic_get_or_use::[u64](prog, net, m, key, 0u64);
        return val;
    }
}
