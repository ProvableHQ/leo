/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u32, b: u32) -> u32 {
        return a ^ b;
    }}
