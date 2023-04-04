/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping account: address => u64;
    
    transition mint_public(public receiver: address, public amount: u64) {
    
    }
    
    finalize mint_public (public receiver: address, public amount: u64) {
        Mapping::set(account, receiver, amount);
    }
}
