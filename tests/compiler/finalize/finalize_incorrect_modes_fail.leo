/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping account: address => u64;
    
    transition mint_public(public receiver: address, public amount: u64) {
        async finalize(receiver, amount);
    }
    
    finalize mint_public (public receiver: address, constant amount: u64) -> constant u64 {
        increment(account, receiver, amount);
    }
    
    transition mint_public2(public receiver: address, public amount: u64) {
        async finalize(receiver, amount);
    }
    
    finalize mint_public2(public receiver: address, amount: u64) -> u64 {
        increment(account, receiver, amount);
        return amount + amount;
    }
}
