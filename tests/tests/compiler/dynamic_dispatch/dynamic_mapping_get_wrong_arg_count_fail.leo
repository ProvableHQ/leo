// FAIL: dynamic mapping get requires exactly 1 argument (the key).
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: field, key: address) -> Final {
        return final { finalize_query(target, key); };
    }
}

final fn finalize_query(target: field, key: address) {
    let val: u64 = Bank@(target)::balances.get(key, 0u64);
}
