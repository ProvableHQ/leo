/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i128 = 170141183460469231731687303715884105727i128;
    return y == true;
}
