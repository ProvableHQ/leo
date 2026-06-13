// Interface-based dynamic mapping get with an identifier literal as target.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn lookup(key: address) -> Final {
        return final { finalize_query(key); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_query(key: address) {
    let _val: u64 = Bank@('test')::balances.get(key);
}
