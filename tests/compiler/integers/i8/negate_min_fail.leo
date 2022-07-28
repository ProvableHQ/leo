/*
namespace: Compile
expectation: Fail
input_file: ../inputs/dummy.in
*/

function main() -> i8 {
    let a: i8 = -127i8;
    let b: i8 = a - 1i8;

    return -b; // This line attempts to return 128i8 however, the largest i8 value is 127i8.
}
