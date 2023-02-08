/*
namespace: Compile
expectation: Pass
*/


program test.aleo {

    transition foo(a: u8) -> u8 {
        return bar(a) + baz(a);
    }

    function bar(a: u8) -> u8 {
        return a * a;
    }

    function baz(a: u8) -> u8 {
        return a + a;
    }
}
