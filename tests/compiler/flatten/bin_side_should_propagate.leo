/*
namespace: Compile
expectation: Pass
input_file: inputs/int.in
*/

function main(x: u8) -> bool {
    let b: u8 = 10u8;
    let y: u8 = x + b;

    return x == (12u8 - 1u8);
}