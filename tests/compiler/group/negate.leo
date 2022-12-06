/*
namespace: Compile
expectation: Pass
input_file: inputs/eq.in
*/

program test.aleo {    
    transition main(a: group, b: group) -> bool {
        assert(-a == b);
    
        return -a == b;
    }}
