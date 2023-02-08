/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: scalar, b: scalar, c: scalar) -> bool {
        return b == 1scalar ? a == 1scalar : c == 2scalar;
    }}
