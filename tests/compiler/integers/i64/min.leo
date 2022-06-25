/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i64 = -9223372036854775808i64;
    return y == true;
}
