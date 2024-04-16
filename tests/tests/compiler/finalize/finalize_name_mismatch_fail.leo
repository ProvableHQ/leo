/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping account: address => u64;
    mapping values: u8 => u8;
    
    async transition mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public(receiver, amount);
    }
    
    async function finalize_mint_private (public receiver: address, public amount: u64) {
        Mapping::set(account, receiver, amount);
    }
    
}
