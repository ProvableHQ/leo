// Test: interface mapping with an array VALUE type does not panic in const propagation.
// The array length literal in the value type must be visited by the type checker so its
// NodeID is in the type table before the const propagation reconstructor runs.

interface TokenStorage {
    mapping balances: address => [u64; 8];
}

program test.aleo: TokenStorage {
    mapping balances: address => [u64; 8];

    fn foo() {}
}
