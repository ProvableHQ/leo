// _dynamic_get with identifier literal arguments.
program test.aleo {
    mapping result: address => u64;

    fn check(key: address) -> Final {
        return final { finalize_check(key); };
    }
}

final fn finalize_check(key: address) {
    let val: u64 = _dynamic_get::[u64]('some_program', 'aleo', 'balances', key);
    Mapping::set(result, key, val);
}
