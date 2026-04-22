// FAIL: bare read of a mapping name should emit a hint directing to `.get(key)`.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: field) -> Final {
        return final { finalize_query(target); };
    }
}

final fn finalize_query(target: field) {
    let v: u64? = Bank@(target)::balances;
}
