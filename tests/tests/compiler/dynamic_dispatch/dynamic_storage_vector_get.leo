// Interface-based dynamic vector element read: Interface@(target)::v.get(i)
export interface Logs {
    storage entries: [u64];
}

program test.aleo {
    fn lookup(target: field, i: u32) -> Final {
        return final { finalize_query(target, i); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_query(target: field, i: u32) {
    let val: u64? = Logs@(target)::entries.get(1u32 / i);
    assert(val != none);
}
