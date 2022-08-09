/*
namespace: Compile
expectation: Fail
*/

@inline
function foo(n: u8) -> u8 {
    return bar(n);
}

@inline
function bar(n: u8) -> u8 {
    return foo(n);
}