// Interface-based dynamic mapping contains: Interface@(target)::mapping_name.contains(key)
export interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn has_account(target: field, key: address) -> Final {
        return final { finalize_has_account(target, key); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_has_account(target: field, key: address) {
    let exists: bool = Bank@(target)::balances.contains(key);
    assert(exists);
}
