/*
namespace: Compile
expectation: Fail
*/

function main() -> i16 {
    let a: i16 = -32767i16;

    return a - 2i16; // This line attempts to return -32769i16 however, the smallest i16 value is -32768i16.
}