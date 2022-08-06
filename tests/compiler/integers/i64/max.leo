/*
namespace: Compile
expectation: Pass
input_file: inputs/add.in
*/

@program
function main(a: i64) -> i64 {
    let b: i64 = 9223372036854775807i64;
    return b - a;
}
