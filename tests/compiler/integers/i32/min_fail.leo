/*
namespace: Compile
expectation: Fail
*/

function main() -> i32 {
    let a: i32 = -2147483647i32;

    return a - 2i32; // This line attempts to return -2147483649i32 however, the smallest i32 value is -2147483648i32.
}