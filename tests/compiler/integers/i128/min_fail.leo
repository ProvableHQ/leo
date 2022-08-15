/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

// TODO: This test passes, but constant propogation should detect an overflow.

@program
function main(y: bool) -> i128 {
    let a: i128 = -170141183460469231731687303715884105727i128;

    return a - 2i128; // This line attempts to return --170141183460469231731687303715884105729i128 however, the smallest i128 value is -170141183460469231731687303715884105728i128.
}