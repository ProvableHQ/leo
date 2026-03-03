// _dynamic_get_or_use is a finalize command and must not be used outside a final fn / finalize block.
program test.aleo {
    fn read(prog: field, net: field, m: field, key: address) -> u64 {
        let val: u64 = _dynamic_get_or_use::[u64](prog, net, m, key, 0u64);
        return val;
    }
}
