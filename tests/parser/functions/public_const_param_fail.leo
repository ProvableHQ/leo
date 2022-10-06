/*
namespace: Parse
expectation: Fail
*/

program test.aleo {    
    function x(x: u32, public const y: i32) {
        return 0;
    }
    
    function x(public constant x: u32, y: i32) {
        return 0;
    }}
