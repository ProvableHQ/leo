/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    transition mint_public(public receiver: address, public amount: u64) {
        return then finalize(receiver, amount);
    }
    
    finalize mint_public (public receiver: address, public amount: u64) {}
}
