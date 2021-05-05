/*
namespace: Compile
expectation: Pass
input_file: input/array.in
*/

function main () -> [[u8; 4]; 2] {
    return [[1u8, 2u8, 3u8, 4u8], [5u8, 6u8, 7u8, 8u8]];
}
