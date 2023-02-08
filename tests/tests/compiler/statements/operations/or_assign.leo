/*
namespace: Compile
expectation: Pass
input_files: ../inputs/dummy.in
*/

program test.aleo {    
    transition main(k: bool) -> bool {
        let b: bool = true;
        b ||= k;
    
        return b;
    }}
