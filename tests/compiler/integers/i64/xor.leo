/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i64, b: i64) -> i64 {
        return a ^ b;
    }
}
