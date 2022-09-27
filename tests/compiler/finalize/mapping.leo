/*
namespace: Compile
expectation: Pass
*/

mapping balances: address => u128;

circuit Token {
    owner: address;
    balance: u128;
}

mapping tokens: address => Token;

circuit Bar {
    a: u128;
}

circuit Baz {
    a: u128;
}

mapping foo: Bar => Baz;

record RealToken {
    // The token owner.
    owner: address,
    // The Aleo balance (in gates).
    gates: u64,
    // The token amount.
    amount: u64,
}

mapping real_tokens: address => RealToken;

mapping owners: RealToken => address;
