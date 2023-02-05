/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar, b: scalar, c: scalar) -> bool {
       return a + b == c;
    }}
