/*
namespace: Compile
expectation: Pass
input_file: input/type_tuple_value_nested_4x3x2.in
*/

function main(a: [[[u8; 2]; 3]; 4]) -> bool {
    const b = [[[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]],
               [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]]; // inline

    return a == b;
}
