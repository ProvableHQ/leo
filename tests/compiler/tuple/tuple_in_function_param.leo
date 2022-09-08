/*
namespace: Compile
expectation: Pass
*/

@program
function foo(a: (u8, u16)) -> (u8, u16) {
    return a;
}

// TODO: Is the unit expression a a valid input?

@program
function bar(a: (u8, (u8, u8))) -> (u8, (u8, u8)) {
    return a;
}
