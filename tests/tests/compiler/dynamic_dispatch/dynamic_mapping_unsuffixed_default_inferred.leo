// PASS: unsuffixed integer literal as get_or_use default is inferred from the mapping value type.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: field, key: address) -> Final {
        return final { finalize_query(target, key); };
    }
}

final fn finalize_query(target: field, key: address) {
    let val: u64 = Bank@(target)::balances.get_or_use(key, 0);
}
