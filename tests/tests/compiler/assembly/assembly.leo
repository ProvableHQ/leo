/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition hello(a: u8, b: u8) -> u8 {
        asm {
            add a b into c;
            add.w c c into d;
        };
        let f: u8 = d + 1u8;
        return f;
    }
}