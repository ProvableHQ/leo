/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u64, b: u64) -> u64 {
        return a ^ b;
    }}
