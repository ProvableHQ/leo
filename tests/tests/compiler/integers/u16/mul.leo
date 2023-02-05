/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u16, b: u16, c: u16) -> bool {
        return a * b == c;
    }
}
