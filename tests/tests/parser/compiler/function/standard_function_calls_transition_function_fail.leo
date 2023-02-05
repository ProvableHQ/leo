/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(a: u32, b: u32, y: bool) -> u32 {
        if y {
           return adder(a, b);
        } else {
            return subber(a, b);
        }
    }
    
    transition adder(a: u32, b: u32) -> u32 {
        return a + b;
    }
    
    transition subber(a: u32, b: u32) -> u32 {
        return a - b;
    }}
