/*
namespace: Parse
expectation: Pass
*/

program test.aleo {    
    function x(x: u32, public y: i32) -> u8 {
        return 0u8;
    }
    
    function x(public x: u32, y: i32) -> u8 {
        return 0u8;
    }
}
