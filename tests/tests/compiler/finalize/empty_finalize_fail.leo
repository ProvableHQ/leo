/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    async transition mint_public(public receiver: address, public amount: u64) -> Future {
        return finalize_mint(receiver, amount);
    }
    
    async function finalize_mint(public receiver: address, public amount: u64) {}
}
