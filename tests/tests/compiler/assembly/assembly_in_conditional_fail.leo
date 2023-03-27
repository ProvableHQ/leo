/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition hello(a: u8, b: u8) -> u8 {
        if a == b {
            asm {
                add a b into c;
                add.w c c into d;
            }
            return d;
        } else {
            return a + a;
        }
    }
}