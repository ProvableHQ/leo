/*
namespace: Compile
expectation: Pass
input_file: inputs/int.in
*/

function main (x: u8) -> bool {
    let y: u8 = 100u8;
    y = x;
    let z: u8 = y * 100u8;
    return z == 0u8;
}