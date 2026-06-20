// Method-style access of a singleton storage variable via dynamic dispatch is rejected;
// the user must use the bare form `Interface@(target)::name`.
export interface Counter {
    storage total: u64;
}

program test.aleo {
    fn lookup(target: field) -> Final {
        return final { finalize_query(target); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_query(target: field) {
    let val: u64? = Counter@(target)::total.get(false);
}
