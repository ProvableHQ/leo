// Test: interface with array types in function prototypes compiles without panicking.
// Regression test for issue #29293: const propagation panicked because the type
// checker never visited interface function prototypes, leaving array-length literals
// without type table entries.

interface Hasher {
    fn hash(input: [u8; 32]) -> [u8; 32];
}

program test.aleo: Hasher {
    fn hash(input: [u8; 32]) -> [u8; 32] {
        return input;
    }

    @noupgrade
    constructor() {}
}
