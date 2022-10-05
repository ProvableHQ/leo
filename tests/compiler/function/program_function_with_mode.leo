/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition foo(a: u8, b: u8) -> public u8 {
        return a + b;
    }
    
    // TODO: Need to test private modifier when it is supported.
    
    transition foo1(a: u8, b: u8) -> u8 {
        return a + b;
    }
    
    transition foo2(a: u8, public b: u8) -> public u8 {
        return a + b;
    }
    
}
