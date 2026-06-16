// FAIL: vectors cannot be read with the bare `Interface@(target)::vec` form; they require `.get(index)`.
interface Logs {
    storage entries: [u64];
}

program test.aleo {
    fn lookup(target: field) -> Final {
        return final { finalize_query(target); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_query(target: field) {
    let v: u64? = Logs@(target)::entries;
}
