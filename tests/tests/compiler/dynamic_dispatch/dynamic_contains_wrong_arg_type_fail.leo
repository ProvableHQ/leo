// FAIL: First three args to _dynamic_contains must be field or identifier.
program test.aleo {
    mapping result: address => bool;

    fn check(key: address) -> Final {
        return final { finalize_check(key); };
    }
}

final fn finalize_check(key: address) {
    let exists: bool = _dynamic_contains(42u64, 0field, 0field, key);
    Mapping::set(result, key, exists);
}
