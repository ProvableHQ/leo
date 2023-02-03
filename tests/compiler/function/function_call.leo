/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(a: u32, b: u32, y: bool) -> u32 {
        if y {
           return adder(a, b);
        } else {
            return subber(a, b);
        }
    }
    
    function adder(a: u32, b: u32) -> u32 {
        return a + b;
    }
    
    function subber(a: u32, b: u32) -> u32 {
        return a - b;
    }}
