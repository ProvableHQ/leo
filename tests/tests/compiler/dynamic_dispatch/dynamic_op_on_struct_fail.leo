// FAIL: dynamic interface access requires an interface type, not a struct.
struct Point {
    x: u32,
    y: u32,
}

program test.aleo {
    fn query(target: field, key: address) -> Final {
        return final { finalize_query(target, key); };
    }
}

final fn finalize_query(target: field, key: address) {
    let v: u64? = Point@(target)::balances.get(key);
}
