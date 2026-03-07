// _dynamic_get with identifier literals as program/network/mapping targets.
program test.aleo {
    mapping balances: address => u64;

    fn get_balance(key: address) -> Final {
        return final { finalize_get_balance(key); };
    }
}

final fn finalize_get_balance(key: address) {
    let val: u64 = _dynamic_get::[u64]('some_program', 'aleo', 'balances', key);
    assert_eq(val, 0u64);
}
