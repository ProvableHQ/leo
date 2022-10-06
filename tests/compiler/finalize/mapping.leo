/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    mapping balances: address => u128;
    
    struct Token {
        owner: address;
        balance: u128;
    }
    
    mapping tokens: address => Token;
    
    struct Bar {
        a: u128;
    }
    
    struct Baz {
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
}
