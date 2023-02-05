/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: group) -> group {
        return 1scalar * a;
    }}
