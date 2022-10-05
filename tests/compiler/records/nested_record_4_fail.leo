/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Token3 {
        owner: address,
        gates: u64,
        bar: (Bar, Bar),
    }
    
    struct Bar {
        bar: (Token, Token),
    }
    
    record Token {
        owner: address,
        gates: u64,
        amount: u64,
    }
    
    
    
    
    
}
