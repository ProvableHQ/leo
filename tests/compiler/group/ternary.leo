/*
namespace: Compile
expectation: Pass
input_file: inputs/point.in
*/

program test.aleo {    
    transition main(a: group, b: group, c: group) -> bool {
        const r: group = true ? a : b;
    
        assert(r == c);
        return r == c;
    }}
