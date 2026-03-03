// _dynamic_get is a finalize command and must not be used outside a final fn / finalize block.
program test.aleo {
    fn read(prog: field, net: field, m: field, key: address) -> u64 {
        let val: u64 = _dynamic_get::[u64](prog, net, m, key);
        return val;
    }
}
