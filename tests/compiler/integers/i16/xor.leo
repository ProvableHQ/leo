/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

program test.aleo {    
    transition main(a: i16, b: i16) -> i16 {
        return a ^ b;
    }
}
