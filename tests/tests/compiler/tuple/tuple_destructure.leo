/*
namespace: Compile
expectation: Pass
*/


program test.aleo {
    function bax(baq: u8) -> (u8, u8) {
        return (baq + baq, baq * baq);
    }

    transition baz(foo: u8, bar: u8) -> u8 {
        let (a, b): (u8, u8) = (foo, bar);
        let (c, d): (u8, u8) = bax(bar);
        let result: u8 = a + b + c + d;
        return result;
    }
}
