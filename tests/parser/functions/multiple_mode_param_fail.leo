/*
namespace: Parse
expectation: Fail
*/

function x(x: u32, private public const y: i32) {
    return 0;
}

function x(public constant const private x: u32, y: i32) {
    return 0;
}

function x(private constant x: u32, y: i32) {
    return 0;
}