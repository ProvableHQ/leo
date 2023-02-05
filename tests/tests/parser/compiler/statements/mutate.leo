/*
namespace: Compile
expectation: Pass
*/

program test.aleo {    
    transition main(x: u32) -> bool {
        let b: u32 = 5u32;
    
        if x == 3u32 {
            b = 1u32;
        } else {
            b = 0u32;
        }
    
        if x == 3u32 {
            return (b == 1u32);
        } else {
            return (b == 0u32);
        }
    }
}
