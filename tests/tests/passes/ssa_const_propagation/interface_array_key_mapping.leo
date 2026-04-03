// Test: An interface declaring a mapping with an array key type does not panic in
// const propagation passes. The interface's array length literal must be visited
// by the type checker so its NodeID is in the type table.

interface TokenStorage {
    mapping allowances: [address; 2] => u128;
}

program test.aleo: TokenStorage {
    mapping allowances: [address; 2] => u128;

    fn foo() {}
}
