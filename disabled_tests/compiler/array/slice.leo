/*
namespace: Compile
expectation: Pass
input_file: input/three_ones.in
*/

// `{from}..{to}` copies the elements of one array into another exclusively
function main(a: [u8; 3]) -> bool {
    const b = [1u8; 4];
    return a == b[0..3];
}
