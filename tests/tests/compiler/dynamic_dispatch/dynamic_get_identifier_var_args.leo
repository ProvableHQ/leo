// _dynamic_get with identifier-typed variable arguments in finalize context.
program test.aleo {
    mapping result: address => u64;

    fn get_balance(prog: identifier, net: identifier, m: identifier, key: address) -> Final {
        return final { finalize_get_balance(prog, net, m, key); };
    }
}

final fn finalize_get_balance(prog: identifier, net: identifier, m: identifier, key: address) {
    let val: u64 = _dynamic_get::[u64](prog, net, m, key);
    Mapping::set(result, key, val);
}
