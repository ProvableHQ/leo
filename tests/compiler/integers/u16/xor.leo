/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u16, b: u16) -> u16 {
        return a ^ b;
    }
}
