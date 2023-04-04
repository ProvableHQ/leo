/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function foo(a: u8, b: u8) -> u8 {
        return a + b then finalize(a, b);
    }
    
    function bar(a: u8, b: u8) -> u8 {
        return a + b;
    }
    
    finalize bar(a: u8, b: u8) -> u8 {
        return a + b;
    }
    
    
    function mint_public(receiver: address, amount: u64) {
        return then finalize(receiver, amount);
    }
    
    finalize mint_public(receiver: address, amount: u64) {
        Mapping::set(account, receiver, amount);
    }
    
}
