/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    transition foo(a: (u8, u16)) -> (u8, u16) {
        return a;
    }

    // TODO: Is the unit expression a a valid input?

    transition bar(a: (u8, (u8, u8))) -> (u8, (u8, u8)) {
        return a;
    }
}

