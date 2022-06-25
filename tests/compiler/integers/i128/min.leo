/*
namespace: Compile
expectation: Skip
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i128 = -170141183460469231731687303715884105728i128;
    return y == true;
}
