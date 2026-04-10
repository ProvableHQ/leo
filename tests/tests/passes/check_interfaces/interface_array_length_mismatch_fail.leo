// Test: interface requires fn hash(input: [u8; 32]) -> [u8; 32], but the
// implementing function uses [u8; 16] for the input parameter.
// The function signature must match exactly, so this should fail.

interface Hasher {
    fn hash(input: [u8; 32]) -> [u8; 32];
}

program test.aleo: Hasher {
    // Wrong array length in the input parameter: [u8; 16] vs [u8; 32].
    fn hash(input: [u8; 16]) -> [u8; 32] {
        return input; // Body is irrelevant; CheckInterfaces runs before TypeChecking.
    }

    @noupgrade
    constructor() {}
}
