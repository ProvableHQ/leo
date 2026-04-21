// FAIL: key argument type does not match the mapping's declared key type.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: field) -> Final {
        return final { finalize_query(target); };
    }
}

final fn finalize_query(target: field) {
    let val: u64 = Bank@(target)::balances.get(42u64);
}
