/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/scalar_group.in
*/

program test.aleo {    
    transition main(a: scalar, b: group) -> group {
       return a * b;
    }}
