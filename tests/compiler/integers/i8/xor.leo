/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

program test.aleo {    
    transition main(a: i8, b: i8) -> i8 {
        return a ^ b;
    }
}
