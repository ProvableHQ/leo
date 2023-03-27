/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition main(x: u32) -> bool {
        let y: u32 = x;

        for i: u32 in 0u32..3u32 {
            asm {
                sub y 1u32 into z;
            }
            y = z + z;
        }
        return y == 0u32;
    }
}
