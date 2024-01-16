/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u8) -> u8 {
        let b: u8 = 1u8;
        b ^= a;
    
        return b;
    }}
