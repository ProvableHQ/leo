/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

// TODO: This test passes, but constant propogation should detect an overflow.

@program
function main(y: bool) -> i32 {
    let a: i32 = -2147483647i32;
    let b: i32 = a - 1i32;

    return -b; // This line attempts to return 2147483648i32 however, the largest i32 value is 2147483647i32.
}
