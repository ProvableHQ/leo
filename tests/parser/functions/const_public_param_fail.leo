/*
namespace: Parse
expectation: Fail
*/

program test.aleo {    
    function x(x: u32, const public y: i32) {
        return 0;
    }
    
    function x(constant public x: u32, y: i32) {
        return 0;
    }}
