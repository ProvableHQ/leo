/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/u32_3.in
 - inputs/u32_5.in
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
