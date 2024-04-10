/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    mapping account: address => u64;
    
    async transition mint_public(public receiver: address, public amount: u64) -> Future {
        return fin_mint(receiver, amount);
    }
    
    async function fin_mint(public receiver: address, public amount: u64) -> u64 {
        Mapping::set(account, receiver, amount);
    }
}
