/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    const a: [u8; (2, 2, 2)] = [1u8; (2, 2, 2)];
    const b: [u8; (2, 2, 2)] = [[[1u8; 2]; 2]; 2];

    return (a == b) == y;
}
