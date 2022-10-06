/*
namespace: Compile
expectation: Pass
input_file: inputs/point.in
*/

program test.aleo {    
    transition main(a: group) -> bool {
        return a == (0, 1)group;
    }}
