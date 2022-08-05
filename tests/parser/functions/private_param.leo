/*
namespace: Parse
expectation: Pass
*/

function x(x: u32, private y: i32) -> u8 {
    return 0u8;
}

function x(private x: u32, y: i32) -> u8 {
    return 0u8;
}
