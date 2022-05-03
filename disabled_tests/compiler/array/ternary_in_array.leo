/*
namespace: Compile
expectation: Pass
input_file: input/three_ones.in
*/

function main (a: [u8; 3]) -> bool {
    let y = a[0..[0u8; 2] == [0u8; 2]? 2u8 : 2u8];
    return y == [1u8, 1];
}