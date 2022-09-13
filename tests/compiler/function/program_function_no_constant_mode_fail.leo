/*
namespace: Compile
expectation: Fail
*/


@program
function foo(constant a: u8) {}

@program
function foo(a: u8) -> constant u8 {
    return a + a;
}
