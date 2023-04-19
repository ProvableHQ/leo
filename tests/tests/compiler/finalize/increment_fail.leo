/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    mapping amounts: address => u128;
    
    transition increase_self(amount: u128) {
        return then finalize(self.caller, amount);
    }
    
    finalize increase_self(addr: address, amount: u128) {
        increment(amounts, addr, amount);
    }
}
