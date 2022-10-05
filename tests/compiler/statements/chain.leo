/*
namespace: Compile
expectation: Pass
input_file:
 - inputs/u32_3.in
 - inputs/u32_5.in
 - inputs/u32_6.in
*/

program test.aleo {    
    transition main(x: u32) -> bool {
        let c: u32 = 0u32;
    
        if x == 1u32 {
            c = 1u32;
        } else if x == 2u32 {
            c = 2u32;
        } else {
            c = 3u32;
        }
        return c == x;
    }
}
