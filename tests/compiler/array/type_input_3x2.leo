/*
namespace: Compile
expectation: Pass
input_file: input/input_tuple_3x2.in
*/

function main(a: [[u8; 2]; 3]) -> bool {
    const b = [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]; // inline

    return a == b;
}
