/*
namespace: Compile
expectation: Fail
*/

program test.aleo {    
    function main(x: u32) -> bool {
        if x == 3u32 {
            return true;
        } else {
            return false;
        }
        let double: u32 = x + x;
        return double;
    }
}
