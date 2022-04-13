/*
namespace: Parse
expectation: Pass
*/

function x(x: u32, constant y: i32) {
    return 0;
}

function x(constant x: u32, y: i32) {
    return 0;
}