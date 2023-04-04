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
    
    finalize mint_private (public receiver: address, public amount: u64) {
        Mapping::set(account, receiver, amount);
    }
    
}
