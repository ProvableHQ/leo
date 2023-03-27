/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition hello(a: u8, b: u8) -> u8 {
        asm {
            add a b into a;
        }
        return a;
    }
}