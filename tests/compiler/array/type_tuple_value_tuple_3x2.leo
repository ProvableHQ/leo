/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main(y: bool) -> bool {
    const a = [[0u8, 0u8], [0u8, 0u8], [0u8, 0u8]]; // inline

    const b: [u8; (3, 2)] = [0; (3, 2)]; // initializer

    return (a == b) == y;
}
