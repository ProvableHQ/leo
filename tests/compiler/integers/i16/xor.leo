/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i16, b: i16) -> i16 {
        return a ^ b;
    }
}
