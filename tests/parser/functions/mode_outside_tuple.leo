/*
namespace: Parse
expectation: Fail
*/

program test.aleo {
    transition(a: u8) -> public (u8, u8) {
        return (a + a, a * a);
    }
}
