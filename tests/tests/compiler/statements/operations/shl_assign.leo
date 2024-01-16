/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition unsigned(a: u8, b: u8, c: u16, d: u32) -> u8 {
        a <<= b;
        a <<= c;
        a <<= d;
    
        return a;
    }
    
    function signed(a: i8, b: u8, c: u16, d: u32) -> i8 {
        a <<= b;
        a <<= c;
        a <<= d;
    
        return a;
    }}
