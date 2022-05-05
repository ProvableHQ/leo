/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i8 = 127i8;
    return y == true;
}
