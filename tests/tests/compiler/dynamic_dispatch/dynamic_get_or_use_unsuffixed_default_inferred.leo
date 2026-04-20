// PASS: unsuffixed integer literal as _dynamic_get_or_use default argument, inferred from type parameter.
// The default `1` resolves to u64 because the type parameter provides the expected type.
program test.aleo {
    fn get(prog: field, net: field, mapping_name: field, key: u64) -> Final {
        return final { finalize_get(prog, net, mapping_name, key); };
    }
}

final fn finalize_get(prog: field, net: field, mapping_name: field, key: u64) {
    let _: u64 = _dynamic_get_or_use::[u64](prog, net, mapping_name, key, 1);
}
