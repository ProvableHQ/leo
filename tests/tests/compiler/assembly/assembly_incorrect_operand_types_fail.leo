/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition hello(a: u8, b: u8) -> u8 {
        let foo: field = 1field;
        let bar: u16 = 4u16;
        asm {
            add a foo into c;
            add.w c bar into d;
        }
        let f: u8 = d + 1u8;
        return f;
    }
}