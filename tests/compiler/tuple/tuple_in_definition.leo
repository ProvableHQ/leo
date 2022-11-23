/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition baz() -> u8 {
        let a: (u8, u16) = (1u8, 2u16);
        return 1u8 + 1u8;
    }
}

