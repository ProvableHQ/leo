/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> u8 {
    for x: u8 in true? 0u8 : 1u8..2u8 {}
    return 1u8;
}