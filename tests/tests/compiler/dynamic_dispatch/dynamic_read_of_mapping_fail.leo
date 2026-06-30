// FAIL: bare read of a mapping name should emit a hint directing to `.get(key)`.
export interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn lookup(target: field) -> Final {
        return final { finalize_query(target); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_query(target: field) {
    let v: u64? = Bank@(target)::balances;
}
