/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    mapping account: address => u64;
    mapping values: u8 => u8;
    
    transition mint_public(public receiver: address, public amount: u64) {
        async finalize(receiver, amount);
    }
    
    finalize mint_public (public receiver: address, public amount: u64) -> u64 {
        increment(account, receiver, amount);
        return amount;
    }
    
    transition public_adder(public a: u8, public b: u8) {
        async finalize(a, b);
    } finalize public_adder(a: u8, b: u8) -> public u8 {
        return a + b;
    }
    
}
