/*
namespace: Compile
expectation: Pass
input_file: input/type_nested_value_nested_3x2.in
*/

function main() -> bool {
    const a = [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]; // inline
    const b: [[u8; 2]; 3] = [[0; 2]; 3]; // initializer

    return a == b;
}
