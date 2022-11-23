/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    transition baz(foo: u8, bar: u16) -> u8 {
        let a: (u8, u16) = (foo, bar);
        a = (3u8, 4u16);
        return 1u8 + 1u8;
    }
}

