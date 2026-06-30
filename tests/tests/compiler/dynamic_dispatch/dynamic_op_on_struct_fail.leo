// FAIL: dynamic interface access requires an interface type, not a struct.
export struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    fn lookup(target: field, key: address) -> Final {
        return final { finalize_query(target, key); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_query(target: field, key: address) {
    let v: u64? = Point@(target)::balances.get(key);
}
