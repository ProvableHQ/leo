/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition main(x: u32) -> bool {
        let y: u32 = x;

        {
            y = y + 5u32;
        }

        return y == 8u32;
    }}