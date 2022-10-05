/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

program test.aleo {    
    transition main(a: i64, b: i64) -> i64 {
        return a ^ b;
    }
}
