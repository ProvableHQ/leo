// Test: interface storage variable with an array type compiles end-to-end.

interface DataStore {
    storage buffer: [u32; 4];
}

program test.aleo: DataStore {
    storage buffer: [u32; 4];

    fn foo() {}
}
