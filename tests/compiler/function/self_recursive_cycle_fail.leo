/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    function fib(n: u8) -> u8 {
        if n <= 1u8 {
            return n;
        } else {
            return fib(n - 1u8) + fib(n - 2u8);
        }
    }

    transition foo(n: u8) -> u8 {
        if n <= 1u8 {
            return n;
        } else {
            return foo(n - 1u8) + foo(n - 2u8);
        }
    }
}
