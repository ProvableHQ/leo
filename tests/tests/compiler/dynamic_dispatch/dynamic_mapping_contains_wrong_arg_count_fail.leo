// FAIL: dynamic mapping contains requires exactly 1 argument (the key).
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn check(target: field, key: address) -> Final {
        return final { finalize_check(target, key); };
    }
}

final fn finalize_check(target: field, key: address) {
    let exists: bool = Bank@(target)::balances.contains(key, true);
}
