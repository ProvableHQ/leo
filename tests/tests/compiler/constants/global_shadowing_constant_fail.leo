/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    const HELLO: u8 = 0u8;
    const HELLO: u8 = 1u8;
    transition foo(a: u8, b: u8, flag: bool) -> u8 {
        let start: (u8, u8) = (a, b);
        return start.0 + start.1 + HELLO;
    }
}
