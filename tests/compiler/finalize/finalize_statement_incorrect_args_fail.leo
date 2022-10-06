/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    mapping account: address => u64;
    
    transition mint_public(public receiver: address, public amount: u64) {
        async finalize(receiver, amount, amount);
    }
    
    finalize mint_public (public receiver: address, public amount: u64) {
        increment(account, receiver, amount);
    }
}
