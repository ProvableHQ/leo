/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

program test.aleo {    
    transition main(a: i32, b: i32) -> i32 {
        return a ^ b;
    }
    
}
