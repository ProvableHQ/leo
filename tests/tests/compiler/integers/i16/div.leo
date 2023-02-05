/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: i16, b: i16, c: i16) -> bool {
        return a / b == c;
    }
}
