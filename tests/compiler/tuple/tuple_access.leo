/*
namespace: Compile
expectation: Pass
*/


program test.aleo {
    transition baz(foo: u8, bar: u8) -> u8 {
        let a: (u8, u8) = (foo, bar);
        let result: u8 = a.0 + a.1;
        return result;
    }
}

