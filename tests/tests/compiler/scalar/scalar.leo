/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar) -> bool {
        const s: scalar = 1scalar;
        return s + a == 0scalar;
    }}
