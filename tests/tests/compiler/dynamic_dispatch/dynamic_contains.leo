// _dynamic_contains for cross-program mapping existence check.
program test.aleo {
    mapping result: address => bool;

    fn check(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_check(prog, net, m, key); };
    }
}

final fn finalize_check(prog: field, net: field, m: field, key: address) {
    let exists: bool = _dynamic_contains(prog, net, m, key);
    Mapping::set(result, key, exists);
}
