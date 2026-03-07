// _dynamic_contains with identifier-typed variable arguments in finalize context.
program test.aleo {
    mapping balances: address => u64;

    fn check_balance(prog: identifier, net: identifier, m: identifier, key: address) -> Final {
        return final { finalize_check_balance(prog, net, m, key); };
    }
}

final fn finalize_check_balance(prog: identifier, net: identifier, m: identifier, key: address) {
    let exists: bool = _dynamic_contains(prog, net, m, key);
    assert(exists);
}
