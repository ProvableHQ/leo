/*
namespace: Compile
expectation: Pass
input_files: ../inputs/u8.in
*/

function main(a: u8) -> u8 {
    let b: u8 = 1u8;
    b /= a;

    return b;
}