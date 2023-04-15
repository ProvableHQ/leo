/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition matches(addr: address) -> bool {
        return self.caller == addr;
    }

    mapping balances: address => u128;
    
    struct Token {
        Owner: address;
        balance: u128;
    }
    
    mapping tokens: address => Token;
    
    struct Bar {
        a: u128;
    }
    
    struct Baz {
        a: u128;
    }
    
    mapping foo: Bar => Baz;
}
