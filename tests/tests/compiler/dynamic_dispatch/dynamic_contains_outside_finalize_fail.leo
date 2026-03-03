// _dynamic_contains is a finalize command and must not be used outside a final fn / finalize block.
program test.aleo {
    fn check(prog: field, net: field, m: field, key: address) -> bool {
        let exists: bool = _dynamic_contains(prog, net, m, key);
        return exists;
    }
}
