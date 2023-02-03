/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar, b: group) -> group {
       return a * b;
    }}
