/*
namespace: Compile
expectation: Pass
input_file: ../input/dummy.in
*/

function main(y: bool) -> bool {
    const a: i16 = -32768i16;
    return y == true;
}
