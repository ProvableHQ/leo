/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping account: address => u64;
    
    async transition mint_public(public receiver: address, public amount: u64) {
    
    }
    
    async function finalize_mint_public (public receiver: address, public amount: u64) {
        Mapping::set(account, receiver, amount);
    }
}
