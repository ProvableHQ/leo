/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    transition main(y: bool) -> i16 {
        let a: i16 = 32768i16;
        return a;
    }}
