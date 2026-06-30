// Interface-based dynamic mapping get with an explicit network argument.
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
    let val: u64 = Bank@(target, 'aleo')::balances.get(key);
}
