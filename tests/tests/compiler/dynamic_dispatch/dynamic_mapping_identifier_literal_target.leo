// Interface-based dynamic mapping get with an identifier literal as target.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(key: address) -> Final {
        return final { finalize_query(key); };
    }
}

final fn finalize_query(key: address) {
    let val: u64 = Bank@('test')::balances.get(key);
}
