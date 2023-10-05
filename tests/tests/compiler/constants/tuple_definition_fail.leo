/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    const (HELLO,GOODBYE): (u8,u8) = (1u8, 1u8);
    transition foo(a: u8, b: u8, flag: bool) -> u8 {
        return GOODBYE;
    }
}