// FAIL: unknown mapping operation; only get, contains, and get_or_use are valid.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: field, key: address) -> Final {
        return final { finalize_query(target, key); };
    }
}

final fn finalize_query(target: field, key: address) {
    Bank@(target)::balances.remove(key);
}
