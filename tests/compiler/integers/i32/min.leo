/*
namespace: Compile
expectation: Skip
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i32 = -2147483648i32;
    return y == true;
}
