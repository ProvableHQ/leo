/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(x: u32) -> bool {
    
        for i: u32 in 0u32..9u32 {
            return false;
        }
    
        for i: u32 in 0u32..9u32 {
            if (x == 0u32) {
                return false;
            } else {
                return true;
            }
        }
    
        return x == 1u32;
    }
}
