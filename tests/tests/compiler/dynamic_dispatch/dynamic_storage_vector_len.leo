// Interface-based dynamic vector length read: Interface@(target)::v.len()
export interface Logs {
    storage entries: [u64];
}

program test.aleo {
    fn lookup(target: field) -> Final {
        return final { finalize_query(target); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_query(target: field) {
    let n: u32 = Logs@(target)::entries.len();
}
