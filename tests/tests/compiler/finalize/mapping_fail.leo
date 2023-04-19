/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping foo: (u32, u32) => u32;
    
    mapping floo: baz => u8;

    mapping floop: foo => foo;
    
    mapping bar: foo => baz;

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
