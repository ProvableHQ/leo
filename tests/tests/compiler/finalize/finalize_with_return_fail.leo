/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping account: address => u64;
    mapping values: u8 => u8;
    
    transition mint_public(public receiver: address, public amount: u64) {
        return then finalize(receiver, amount);
    }
    
    finalize mint_public (public receiver: address, public amount: u64) -> u64 {
        Mapping::set(account, receiver, amount);
        return amount;
    }
    
    transition public_adder(public a: u8, public b: u8) {
        return then finalize(a, b);
    } finalize public_adder(a: u8, b: u8) -> public u8 {
        return a + b;
    }
    
}
