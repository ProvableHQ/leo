/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(k: bool) -> bool {
        let b: bool = true;
        b ||= k;
    
        return b;
    }}
