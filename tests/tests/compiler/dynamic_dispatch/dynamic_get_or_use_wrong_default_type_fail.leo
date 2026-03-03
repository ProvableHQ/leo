// The default argument to _dynamic_get_or_use must match the type parameter.
program test.aleo {
    mapping result: address => u64;

    fn read(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_read(prog, net, m, key); };
    }
}

final fn finalize_read(prog: field, net: field, m: field, key: address) {
    // Type param is u64 but default is u32 — mismatch.
    let val: u64 = _dynamic_get_or_use::[u64](prog, net, m, key, 0u32);
    Mapping::set(result, key, val);
}
