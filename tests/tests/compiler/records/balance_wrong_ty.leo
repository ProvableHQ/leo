/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    record Token {
        owner: address,
    }
    
    transition main(a: u8, b:u8) -> u8 {
        return a + b;
    }
}
