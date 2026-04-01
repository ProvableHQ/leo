// Test: invalid mapping key type in an interface prototype produces a type error, not a panic.
// Before the fix, the type checker never visited interface mappings, so this would panic
// in later passes rather than emitting a proper error.

interface Store {
    mapping data: (u32, u64) => u128;
}

program test.aleo: Store {
    mapping data: (u32, u64) => u128;

    fn foo() {}
}
