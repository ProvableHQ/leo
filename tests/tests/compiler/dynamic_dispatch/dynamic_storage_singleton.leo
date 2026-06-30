// Interface-based dynamic singleton storage read: Interface@(target)::x
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
    let val: u64? = Counter@(target)::total;
}
