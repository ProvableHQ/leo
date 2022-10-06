/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

program test.aleo {    
    function tester(hi: u8) -> u8 {
        const hi = 2u8;
        return hi;
    }
    
    function main (y: bool) -> bool {
        return y;
    }}
