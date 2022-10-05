/*
namespace: Compile
expectation: Pass
input_file: inputs/u32_3.in
*/

program test.aleo {    
    transition main(x: u32) -> bool {
        let y: u32 = x;
        
        for i: u32 in 0u32..3u32 {
            y = y - 1u32;
        }
    
        return y == 0u32;
    }
}
