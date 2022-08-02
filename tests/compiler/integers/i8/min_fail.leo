/*
namespace: Compile
expectation: Fail
*/

function main() -> i8 {
    let a: i8 = -127i8;

    return a - 2i8; // This line attempts to return -129i8 however, the smallest i8 value is -128i8.
}