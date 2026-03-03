program test.aleo {
    mapping balances: address => u64;

    fn check_balance(prog: field, net: field, m: field, key: address) -> Final {
        return final { finalize_check_balance(prog, net, m, key); };
    }
}

final fn finalize_check_balance(prog: field, net: field, m: field, key: address) {
    let exists: bool = _dynamic_contains(prog, net, m, key);
    assert(exists);
}
