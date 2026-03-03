// First three args to _dynamic_contains must be field; passing u64 for arg 0 is an error.
program test.aleo {
    fn check(prog: u64, net: field, m: field, key: address) -> Final {
        return final { finalize_check(prog, net, m, key); };
    }
}

final fn finalize_check(prog: u64, net: field, m: field, key: address) {
    let exists: bool = _dynamic_contains(prog, net, m, key);
    assert(exists);
}
