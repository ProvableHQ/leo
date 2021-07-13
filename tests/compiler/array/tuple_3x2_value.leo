/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

// Multidimensional array syntax in leo
function main(y: bool) -> bool {
    const a = [[0u32, 0u32], [0u32, 0u32], [0u32, 0u32]]; // inline
    const b: [u32; (3, 2)] = [0; (3, 2)]; // initializer

    return (a == b) == y;
}
