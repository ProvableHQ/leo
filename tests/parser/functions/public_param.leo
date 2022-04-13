/*
namespace: Parse
expectation: Pass
*/

function x(x: u32, public y: i32) {
    return 0;
}

function x(public x: u32, y: i32) {
    return 0;
}
