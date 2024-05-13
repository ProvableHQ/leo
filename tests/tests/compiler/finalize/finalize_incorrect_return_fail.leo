/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping account: address => u64;
    
    async transition mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint_public(receiver, amount);
    }
    
    async function finalize_mint_public(public receiver: address, public amount: u64) -> u64 {
        Mapping::set(account, receiver, amount);
        return 1u8 + 2u8;
    }
}
