// _dynamic_get_or_use for cross-program mapping read with default.
program test.aleo {
    mapping result: address => u64;

    fn get_balance(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_get_balance(prog, net, m, key); };
    }
}

final fn finalize_get_balance(prog: field, net: field, m: field, key: address) {
    let val: u64 = _dynamic_get_or_use::[u64](prog, net, m, key, 0u64);
    Mapping::set(result, key, val);
}
