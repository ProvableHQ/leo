/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function foo(a: u8, b: u8) -> Future {
        return finalize_bar(a, b);
    }
    
    function bar(a: u8, b: u8) -> u8 {
        return a + b;
    }
    
    async function finalize_bar(a: u8, b: u8) -> u8 {
        return a + b;
    }
    
    
    function mint_public(receiver: address, amount: u64) -> Future {
        return finalize_mint(receiver, amount);
    }
    
    async function finalize_mint(receiver: address, amount: u64) {
        Mapping::set(account, receiver, amount);
    }
    
}
