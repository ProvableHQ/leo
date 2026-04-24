// Interface-based dynamic mapping get with an identifier variable as target.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: identifier, key: address) -> Final {
        return final { finalize_query(target, key); };
    }
}

final fn finalize_query(target: identifier, key: address) {
    let val: u64 = Bank@(target)::balances.get(key);
}
