/*
namespace: Compile
expectation: Pass
input_file: inputs/three.in
*/

program test.aleo {    
    transition main(a: group, b: group, c: group) -> bool {
        console.assert(a - b == c);
    
        return a - b == c;
    }}
