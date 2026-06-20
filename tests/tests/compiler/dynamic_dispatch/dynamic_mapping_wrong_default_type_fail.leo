// FAIL: default value type does not match the mapping's declared value type.
export interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn lookup(target: field, key: address) -> Final {
        return final { finalize_query(target, key); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_query(target: field, key: address) {
    let val: u64 = Bank@(target)::balances.get_or_use(key, true);
}
