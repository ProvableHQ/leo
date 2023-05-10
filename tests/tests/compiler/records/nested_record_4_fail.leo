/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    record Token3 {
        owner: address,
        bar: (Bar, Bar),
    }
    
    struct Bar {
        bar: (Token, Token),
    }
    
    record Token {
        owner: address,
        amount: u64,
    }
    
    
    
    
    
}
