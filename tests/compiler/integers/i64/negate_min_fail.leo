/*
namespace: Compile
expectation: Fail
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool{
    const a: i64 = -9223372036854775808i64;
    const b: i64 = -a;
    return (b == -a) == y;
}
