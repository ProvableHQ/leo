/*
namespace: Compile
expectation: Fail
*/

@inline
function fib(n: u8) -> u8 {
    if n <= 1u8 {
        return n;
    } else {
        return fib(n - 1u8) + fib(n - 2u8);
    }
}