/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i8) -> i8 {
        let b: i8 = -127i8;
    
        return b - a; // -128i8
    }
}
