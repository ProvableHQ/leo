/*
namespace: Compile
expectation: Fail
*/


program test.aleo {
    transition baz(a: (u8, u8)) -> u8 {
        let result: u8 = a.0 + a.1;
        return result;
    }
}

