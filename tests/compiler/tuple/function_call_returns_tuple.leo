/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    function foo(a: u8, b: u8) -> (u8, u8) {
        if (a == b) {
            return (a, b);
        }
        let c: u8 = a + b;
        let d: u8 = a - b;
        return (c, d);
    }

    transition bar(flag: bool, a: u8, b: u8) -> (u8, u8) {
        let start: (u8, u8) = foo(a, b);
        if flag {
            start = foo(start.0, start.1);
        } else {

            start = foo(start.1, start.0);
        }
        return start;
    }
}
