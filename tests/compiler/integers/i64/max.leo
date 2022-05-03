/*
namespace: Compile
expectation: Pass
input_file: ../input/dummy.in
*/

function main(y: bool) -> bool {
    const a: i64 = 9223372036854775807i64;
    return y == true;
}
