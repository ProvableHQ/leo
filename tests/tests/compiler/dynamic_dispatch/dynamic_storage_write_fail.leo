// Writing to an interface storage variable via dynamic dispatch must be rejected;
// dynamic access is read-only.
export interface Counter {
    storage total: u64;
}

program test.aleo {
    fn update(target: field) -> Final {
        return final { finalize_update(target); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_update(target: field) {
    Counter@(target)::total = 5u64;
}
