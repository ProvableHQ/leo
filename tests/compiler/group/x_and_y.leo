/*
namespace: Compile
expectation: Pass
input_file: inputs/scalar_group.in
*/

program test.aleo {    
    transition main(a: scalar) -> group {
        let b: group = (0, 1)group;
    
        return a * b;
    }
}
