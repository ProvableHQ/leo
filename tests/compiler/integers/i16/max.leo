/*
namespace: Compile
expectation: Pass
input_file: ../input/dummy.in
*/

function main(y: bool) -> bool {
    const a: i16 = 32767;
    return y == true;
}
