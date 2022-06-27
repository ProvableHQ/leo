/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main(constant a: u8) -> u8 {
    const d: u8 = 1u8 + 1u8;
    return d == 2u8;
}
