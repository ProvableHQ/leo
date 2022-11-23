/*
namespace: Compile
expectation: Fail
*/

// TODO: Compilation should pass, but warnings should be emitted.

program test.aleo {
    transition foo(a: u8, b: u8) -> u8 {
        let c: (u8) = (a);
        let d: (u8) = (3u8 + 4u8);
        return a + b;
    }
}

