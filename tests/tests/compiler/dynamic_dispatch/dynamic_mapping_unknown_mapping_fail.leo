// Error: mapping name not found in interface.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: field, key: address) -> Final {
        return final { finalize_query(target, key); };
    }
}

final fn finalize_query(target: field, key: address) {
    let val: u64 = Bank@(target)::nonexistent.get(key);
}
