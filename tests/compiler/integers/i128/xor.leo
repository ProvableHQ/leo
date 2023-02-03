/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i128, b: i128) -> i128 {
        return a ^ b;
    }
    
}
