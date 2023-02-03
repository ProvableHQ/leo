/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u64, b: u64, c: u64) -> bool {
        return a & b == c;
    }
}
