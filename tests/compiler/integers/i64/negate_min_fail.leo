/*
namespace: Compile
expectation: Fail
input_file: ../inputs/dummy.in
*/

function main() -> i64 {
    let a: i64 = -9223372036854775807i64;
    let b: i64 = a - 1i64;

    return -b; // This line attempts to return 9223372036854775808i64 however, the largest i64 value is 9223372036854775807i64.
}
