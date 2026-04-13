// FAIL: unsuffixed integer literal as _dynamic_get key argument.
// The key `1` has no expected type; type inference fails.
// Fix: use a suffixed literal such as 1u64.
program test.aleo {
    fn check(prog: field, net: field, mapping_name: field) -> Final {
        return final { finalize_check(prog, net, mapping_name); };
    }
}

final fn finalize_check(prog: field, net: field, mapping_name: field) {
    let _: u64 = _dynamic_get::[u64](prog, net, mapping_name, 1);
}
