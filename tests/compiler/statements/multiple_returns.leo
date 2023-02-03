/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(x: u32) -> bool {
        if x == 3u32 {
            return true;
        } else {
            return false;
        }
    }
}
