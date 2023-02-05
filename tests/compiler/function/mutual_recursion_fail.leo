/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    function foo(n: u8) -> u8 {
        return bar(n);
    }

    function bar(n: u8) -> u8 {
        return foo(n);
    }

    transition baz(n: u8) -> u8 {
        return bax(n);
    }

    transition bax(n: u8) -> u8 {
        return baz(n);
    }
}
