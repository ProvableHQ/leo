// FAIL: unknown vector operation; only get and len are valid on external storage vectors.
export interface Logs {
    storage entries: [u64];
}

program test.aleo {
    fn lookup(target: field, v: u64) -> Final {
        return final { finalize_query(target, v); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_query(target: field, v: u64) {
    Logs@(target)::entries.push(v);
}
