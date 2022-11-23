/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition bar(a: u8) -> (u8, (u8, u8)) {
        return (a, (a + a, a * a));
    }
}

