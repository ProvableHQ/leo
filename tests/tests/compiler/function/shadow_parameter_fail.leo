/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function tester(hi: u8) -> u8 {
        let hi = 2u8;
        return hi;
    }
    
    function main (y: bool) -> bool {
        return y;
    }}
