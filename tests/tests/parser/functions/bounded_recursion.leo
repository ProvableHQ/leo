/*
namespace: Parse
expectation: Pass
*/

program test.aleo {    
    function x(constant y: u32) -> u8 {
        if y < 5u32 {
            x(y+1u32);
        }
    }
    
    function main(y: bool) -> bool {
        x(1u32);
        return y;
    }
}
