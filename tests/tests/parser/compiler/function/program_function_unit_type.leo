/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition unit0(a: u8, b: u8) {}
    
    transition unit1(a: u8, b: u8) -> () {}
}
