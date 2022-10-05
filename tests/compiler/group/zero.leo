/*
namespace: Compile
expectation: Pass
input_file: inputs/scalar_group.in
*/

program test.aleo {    
    transition main(a: scalar) -> group {
        let element: group = 0group;
    
        return element * a;
    }}
