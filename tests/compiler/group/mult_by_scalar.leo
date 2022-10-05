/*
namespace: Compile
expectation: Pass
input_file: inputs/point.in
*/

program test.aleo {    
    transition main(a: group) -> group {
        return 1scalar * a;
    }}
