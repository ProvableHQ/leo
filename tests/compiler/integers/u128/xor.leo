/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u128, b: u128) -> u128 {
        return a ^ b;
    }}
