/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

program test.aleo {    
    transition main(a: u64, b: u64) -> u64 {
        return a ^ b;
    }}
