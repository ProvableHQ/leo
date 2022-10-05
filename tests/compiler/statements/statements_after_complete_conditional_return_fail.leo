/*
namespace: Compile
expectation: Fail
input_file:
 - inputs/u32_3.in
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
