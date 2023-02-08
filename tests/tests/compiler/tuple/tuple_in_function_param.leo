/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition foo(a: (u8, u16)) -> (u8, u16) {
        return a;
    }
}

