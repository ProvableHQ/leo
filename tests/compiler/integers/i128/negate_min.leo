/*
namespace: Compile
expectation: Fail
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool{
    const a: i128 = -170141183460469231731687303715884105728i128;
    const b: i128 = -a;
    return (b == -a) == y;
}
