// Test: interface storage variable with an array type does not panic in const propagation.
// The array length literal in the storage type must be visited by the type checker so its
// NodeID is in the type table before the const propagation reconstructor runs.

interface DataStore {
    storage buffer: [u32; 16];
}

program test.aleo: DataStore {
    storage buffer: [u32; 16];

    fn foo() {}
}
