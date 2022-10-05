/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    struct Foo {
        // The token amount.
        token: Token,
    }
    
    record Token {
        // The token owner.
        owner: address,
        // The Aleo balance (in gates).
        gates: u64,
        // The token amount.
        foo: Foo,
    }
}
