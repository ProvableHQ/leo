/*
namespace: Parse
expectation: Pass
*/

function x(x: u32, public const y: i32) {
    return 0;
}

function x(public const x: u32, y: i32) {
    return 0;
}