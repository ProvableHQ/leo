// FAIL: dynamic mapping get_or_use requires exactly 2 arguments (key and default).
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: field, key: address) -> Final {
        return final { finalize_query(target, key); };
    }
}

final fn finalize_query(target: field, key: address) {
    let val: u64 = Bank@(target)::balances.get_or_use(key);
}
