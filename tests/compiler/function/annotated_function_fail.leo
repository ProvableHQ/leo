/*
namespace: Compile
expectation: Fail
*/

@test
function foo(a: u8, b: u8) -> u8 {
    return a + b;
}

@program
function bar(a: u8, b: u8) -> u8 {
    return a * b;
}