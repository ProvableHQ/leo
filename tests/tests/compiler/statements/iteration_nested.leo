/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(x: u32) -> u32 {
        let y: u32 = x;
    
        for i: u32 in 0u32..2u32 {
            for j: u32 in 0u32..2u32 {
                y = y + 1u32;
            }
        }
    
        return y;
    }
}
