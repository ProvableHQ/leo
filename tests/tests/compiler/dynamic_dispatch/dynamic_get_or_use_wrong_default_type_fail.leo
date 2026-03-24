// FAIL: Default value type must match the type parameter.
program test.aleo {
    mapping result: address => u64;

    fn check(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_check(prog, net, m, key); };
    }
}

final fn finalize_check(prog: field, net: field, m: field, key: address) {
    let val: u64 = _dynamic_get_or_use::[u64](prog, net, m, key, true);
    Mapping::set(result, key, val);
}
