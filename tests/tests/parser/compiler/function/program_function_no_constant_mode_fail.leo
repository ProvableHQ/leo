/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    
    transition foo(constant a: u8) {}
    
    transition foo(a: u8) -> constant u8 {
        return a + a;
    }
}
