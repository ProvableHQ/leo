// _dynamic_get_or_use with identifier-typed variable arguments in finalize context.
program test.aleo {
    mapping result: address => u64;

    fn get_or_default(prog: identifier, net: identifier, m: identifier, key: address) -> Final {
        return final { finalize_get_or_default(prog, net, m, key); };
    }
}

final fn finalize_get_or_default(prog: identifier, net: identifier, m: identifier, key: address) {
    let val: u64 = _dynamic_get_or_use::[u64](prog, net, m, key, 0u64);
    Mapping::set(result, key, val);
}
