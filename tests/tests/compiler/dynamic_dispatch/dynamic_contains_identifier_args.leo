// _dynamic_contains with identifier literals as program/network/mapping targets.
program test.aleo {
    mapping balances: address => u64;

    fn check_balance(key: address) -> Final {
        return final { finalize_check_balance(key); };
    }
}

final fn finalize_check_balance(key: address) {
    let exists: bool = _dynamic_contains('some_program', 'aleo', 'balances', key);
    assert(exists);
}
