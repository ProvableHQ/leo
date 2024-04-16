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
    
    async function finalize_mint_public (public receiver: address, public amount: u64) -> u64 {
        Mapping::set(account, receiver, amount);
        return amount;
    }
    
    async transition public_adder(public a: u8, public b: u8) -> Future {
        return finalize_public_adder(a, b);
    } async function finalize_public_adder(a: u8, b: u8) -> public u8 {
        return a + b;
    }
    
}
