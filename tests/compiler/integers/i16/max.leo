/*
namespace: Compile
expectation: Pass
input_file: ../inputs/dummy.in
*/

function main(y: bool) -> bool {
    const a: i16 = 32767i16;
    return y == true;
}
