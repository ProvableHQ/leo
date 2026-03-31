// FAIL: _dynamic_get_or_use requires exactly 5 arguments.
program test.aleo {
    mapping result: address => u64;

    fn read(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_read(prog, net, m, key); };
    }
}

final fn finalize_read(prog: field, net: field, m: field, key: address) {
    let val: u64 = _dynamic_get_or_use::[u64](prog, net, m, key);
    Mapping::set(result, key, val);
}
