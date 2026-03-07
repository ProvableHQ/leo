// _dynamic_get_or_use with identifier literals as program/network/mapping targets.
program test.aleo {
    mapping balances: address => u64;

    fn get_balance_or_default(key: address) -> Final {
        return final { finalize_get_balance_or_default(key); };
    }
}

final fn finalize_get_balance_or_default(key: address) {
    let val: u64 = _dynamic_get_or_use::[u64]('some_program', 'aleo', 'balances', key, 0u64);
    assert_eq(val, 0u64);
}
