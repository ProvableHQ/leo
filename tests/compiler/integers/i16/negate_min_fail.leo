/*
namespace: Compile
expectation: Fail
input_file: ../inputs/dummy.in
*/

function main() -> i16 {
    let a: i16 = -32767i16;
    let b: i16 = a - 1i16;

    return -b; // This line attempts to return 32768i16 however, the largest i16 value is 32767i16.
}
