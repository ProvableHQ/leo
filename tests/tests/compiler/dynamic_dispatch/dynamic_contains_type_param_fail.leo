// FAIL: _dynamic_contains does not accept type parameters.
program test.aleo {
    mapping result: address => bool;

    fn check(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_check(prog, net, m, key); };
    }
}

final fn finalize_check(prog: field, net: field, m: field, key: address) {
    let exists: bool = _dynamic_contains::[bool](prog, net, m, key);
    Mapping::set(result, key, exists);
}
