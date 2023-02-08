/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function hi() -> u8 {
        return 0u8;
    }
    
    function tester(hi: u8) -> u8 {
        return 0u8;
    }
    
    function main (y: bool) -> bool {
        return y;
    }}
