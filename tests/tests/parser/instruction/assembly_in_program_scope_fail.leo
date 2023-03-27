/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    asm {}

    transition hello(a: u8, b: u8) -> u8 {

        let f: u8 = d + 1u8;
        return f;
    }

}