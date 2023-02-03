/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i8, b: i8) -> i8 {
        return a ^ b;
    }
}
