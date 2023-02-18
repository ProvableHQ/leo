/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar) -> bool {
        let s: scalar = 1scalar;
        return s + a == 0scalar;
    }}
