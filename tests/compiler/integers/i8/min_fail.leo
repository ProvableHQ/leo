/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

// TODO: This test passes, but constant propogation should detect an overflow.

@program
function main(y: bool) -> i8 {
    let a: i8 = -127i8;

    return a - 2i8; // This line attempts to return -129i8 however, the smallest i8 value is -128i8.
}