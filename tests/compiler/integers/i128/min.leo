/*
namespace: Compile
expectation: Pass
input_file: ../input/dummy.in
*/

function main(y: bool) -> bool {
    const a: i128 = -170141183460469231731687303715884105728;
    return y == true;
}
