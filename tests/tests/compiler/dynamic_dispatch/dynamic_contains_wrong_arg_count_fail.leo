// FAIL: _dynamic_contains requires exactly 4 arguments.
program test.aleo {
    mapping result: address => bool;

    fn check(prog: field, net: field, key: address) -> Final {
        return final { finalize_check(prog, net, key); };
    }
}

final fn finalize_check(prog: field, net: field, key: address) {
    let exists: bool = _dynamic_contains(prog, net, key);
    Mapping::set(result, key, exists);
}
