// Error: mapping name not found in interface.
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
    let val: u64 = Bank@(target)::nonexistent.get(key);
}
