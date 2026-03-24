// FAIL: _dynamic_get requires a type parameter.
program test.aleo {
    mapping result: address => u64;

    fn check(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_check(prog, net, m, key); };
    }
}

final fn finalize_check(prog: field, net: field, m: field, key: address) {
    let val = _dynamic_get(prog, net, m, key);
    Mapping::set(result, key, val);
}
