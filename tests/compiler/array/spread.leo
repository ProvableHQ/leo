/*
namespace: Compile
expectation: Pass
input_file: input/three_ones.in
*/

// A spread operator `...` copies the elements of one array into another
function main(a: [u8; 3]) -> bool {
    const b = [1u8, 1u8];
    const c = [1u8, ...b];
    const d = [...b, 1u8];

    return a == c && d == a;
}
