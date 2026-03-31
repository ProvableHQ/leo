// _dynamic_get for cross-program mapping read.
program test.aleo {
    mapping result: address => u64;

    fn get_balance(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_get_balance(prog, net, m, key); };
    }
}

final fn finalize_get_balance(prog: field, net: field, m: field, key: address) {
    let val: u64 = _dynamic_get::[u64](prog, net, m, key);
    Mapping::set(result, key, val);
}
