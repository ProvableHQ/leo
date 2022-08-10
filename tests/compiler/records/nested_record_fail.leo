/*
namespace: Compile
expectation: Fail
*/

record Foo {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

record Token {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    foo: Foo,
}