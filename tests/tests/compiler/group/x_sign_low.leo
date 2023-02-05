/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar) -> group {
        let b: group = (0, -)group;
    
        return a * b;
    }}
