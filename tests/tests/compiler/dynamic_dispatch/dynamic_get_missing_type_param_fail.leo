// _dynamic_get requires exactly one type parameter specifying the return type.
program test.aleo {
    mapping result: address => u64;

    fn read(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_read(prog, net, m, key); };
    }
}

final fn finalize_read(prog: field, net: field, m: field, key: address) {
    // No type parameter provided.
    let val = _dynamic_get(prog, net, m, key);
    Mapping::set(result, key, val);
}
