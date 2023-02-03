/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i16) -> i16 {
        let b: i16 = 32767i16;
        return b - a;
    }
}
