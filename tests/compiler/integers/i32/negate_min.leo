/*
namespace: Compile
expectation: Fail
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool{
    const a: i32 = -2147483648i32;
    const b: i32 = -a;
    return (b == -a) == y;
}
