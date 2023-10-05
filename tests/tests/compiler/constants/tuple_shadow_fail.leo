/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    const (HELLO,GOODBYE): (u8,u8) = (0u8,0u8);
    const HELLO: u8 = 0u8;
    transition foo(a: u8, b: u8, flag: bool) -> u8 {
        return 1u8;
    }
}
